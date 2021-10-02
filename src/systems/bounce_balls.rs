use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct BounceBallsSystem;

impl<'s> System<'s> for BounceBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, locals): Self::SystemData) {
        for (ball, local) in (&mut balls, &locals).join() {
            let y = local.translation().y;
            let x = local.translation().x;

            // Bounce off of the bottom and top boundaries
            if (y - ball.radius <= 0.0 && ball.velocity[1] < 0.0)
                || (y + ball.radius >= ARENA_HEIGHT && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Bounce off of paddles
            for (paddle, local) in (&paddles, &locals).join() {
                let paddle_x = local.translation().x;
                let paddle_y = local.translation().y;

                if point_in_rect(x, y, paddle_x, paddle_y, paddle.width, paddle.height)
                    && ((paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0))
                {
                    ball.velocity[0] = -ball.velocity[0];
                }
            }
        }
    }
}

fn point_in_rect(
    x: f32,
    y: f32,
    rect_x: f32,
    rect_y: f32,
    rect_width: f32,
    rect_height: f32,
) -> bool {
    x < rect_x + rect_width
        && x > rect_x - rect_width
        && y < rect_y + rect_height
        && y > rect_y - rect_height
}
