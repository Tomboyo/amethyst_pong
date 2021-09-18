use amethyst::assets::Handle;
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData};
use amethyst::prelude::Builder;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::shred::{ResourceId, World};

use crate::pong::{Ball, BallSpawnTimeout, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct BallSpawnTimeoutSystem;

#[derive(SystemData)]
pub struct Data<'s> {
    sprite_sheet: Read<'s, Option<Handle<SpriteSheet>>>,
    time: Read<'s, Time>,
    entities: Entities<'s>,
    lazy_update: Read<'s, LazyUpdate>,
    timeouts: ReadStorage<'s, BallSpawnTimeout>,
}

impl<'s> System<'s> for BallSpawnTimeoutSystem {
    type SystemData = Data<'s>;

    fn run(&mut self, data: Self::SystemData) {
        for (timeout, entity) in (&data.timeouts, &data.entities).join() {
            if timeout.expiry < data.time.absolute_time_seconds() {
                data.entities.delete(entity).unwrap();

                let sprite_sheet = data.sprite_sheet.clone().unwrap();
                data.lazy_update
                    .create_entity(&data.entities)
                    .with(Ball {
                        radius: 2.0,
                        velocity: timeout.velocity,
                    })
                    .with(
                        Transform::default()
                            .set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0)
                            .to_owned(),
                    )
                    .with(SpriteRender::new(sprite_sheet.clone(), 1))
                    .build();
            }
        }
    }
}
