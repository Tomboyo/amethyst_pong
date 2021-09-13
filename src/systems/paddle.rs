use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            }
            .map(|x| {
                // TODO: amethyst::core::timing::Time for delta time impl
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + (25.0 * x * time.delta_seconds()))
                        .min(ARENA_HEIGHT - (PADDLE_HEIGHT / 2.0))
                        .max(PADDLE_HEIGHT / 2.0),
                );
            });
        }
    }
}
