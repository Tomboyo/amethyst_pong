use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Time, Transform},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::{Builder, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::World,
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform},
    GameData, SimpleState, StateData,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(Default)]
pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet = load_sprite_sheet(data.world);
        data.world.insert(sprite_sheet.clone());

        initialize_camera(data.world);
        initialize_scoreboard(data.world);
        initialize_paddles(data.world, sprite_sheet);
        initialize_ball_spawn_timeout(data.world);
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(
            Transform::default()
                .set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0)
                .to_owned(),
        )
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource::<AssetStorage<FontAsset>>(),
    );

    let left = world
        .create_entity()
        .with(UiTransform::new(
            "left".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            -50.,
            -50.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let right = world
        .create_entity()
        .with(UiTransform::new(
            "right".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            50.,
            -50.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(ScoreUi { left, right });
}

fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(
            Transform::default()
                .set_translation_xyz(PADDLE_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0)
                .to_owned(),
        )
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(
            Transform::default()
                .set_translation_xyz(ARENA_WIDTH - (PADDLE_WIDTH / 2.0), ARENA_HEIGHT / 2.0, 0.0)
                .to_owned(),
        )
        .with(sprite_render)
        .build();
}

fn initialize_ball_spawn_timeout(world: &mut World) {
    let expiry = {
        let time = world.fetch::<Time>();
        time.absolute_time_seconds() + 3.0
    };

    world
        .create_entity()
        .with(BallSpawnTimeout {
            expiry,
            velocity: [10.0, 15.0],
        })
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub radius: f32,
    pub velocity: [f32; 2],
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct BallSpawnTimeout {
    pub expiry: f64,
    pub velocity: [f32; 2],
}

impl Component for BallSpawnTimeout {
    type Storage = DenseVecStorage<Self>;
}

pub struct ScoreUi {
    pub left: Entity,
    pub right: Entity,
}
