use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData};
use amethyst::prelude::Builder;

use crate::pong::{Ball, BallSpawnTimeout, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, transforms, entities, lazy_update, time): Self::SystemData) {
        for (ball, transform, entity) in (&balls, &transforms, &entities).join() {
            let x = transform.translation().x;
            if x <= -ball.radius || x >= ARENA_WIDTH + ball.radius {
                entities.delete(entity).unwrap();

                lazy_update
                    .create_entity(&entities)
                    .with(BallSpawnTimeout {
                        expiry: time.absolute_time_seconds() + 3.0,
                        velocity: [-ball.velocity[0], -ball.velocity[1]],
                    })
                    .build();
            }
        }
    }
}
