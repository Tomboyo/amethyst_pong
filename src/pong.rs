use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Time, Transform},
    ecs::{Component, DenseVecStorage},
    prelude::{Builder, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::World,
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(Default)]
pub struct Pong {
    ball_spawn_countdown: Option<f32>,
    // TODO: Lazy<T> type for this?
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ball_spawn_countdown.replace(3.0);
        self.sprite_sheet_handle
            .replace(load_sprite_sheet(data.world));

        initialize_camera(data.world);
        initialize_paddles(data.world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Spawn the ball after the countdown expires.
        if let Some(mut t) = self.ball_spawn_countdown.take() {
            {
                let time = data.world.fetch::<Time>();
                t -= time.delta_seconds();
            }
            if t <= 0.0 {
                initialize_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_countdown.replace(t);
            }
        }

        Trans::None
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

fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    world
        .create_entity()
        .with(Ball {
            radius: 2.0,
            velocity: [10.0, 15.0],
        })
        .with(
            Transform::default()
                .set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0)
                .to_owned(),
        )
        .with(SpriteRender::new(sprite_sheet, 1))
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
