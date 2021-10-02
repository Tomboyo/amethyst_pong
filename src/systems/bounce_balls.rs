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
    let w = rect_width / 2.;
    let h = rect_height / 2.;

    x <= rect_x + w && x >= rect_x - w && y <= rect_y + h && y >= rect_y - h
}

#[cfg(test)]
mod tests {
    use super::*;

    // This rectangle is used in the following tests.
    //   a       b
    //  +-------+
    //  |   |   |
    //  |   |c  |
    //  |---+---|
    //  |   |   |
    //  |d  |   |e
    //  +-------+
    //
    // c: (50, 50)
    // a: (40, 0)
    // b: (60, 0)
    // d: (40, 100)
    // e: (60, 100)
    fn f(x: f32, y: f32) -> bool {
        point_in_rect(x, y, 50., 50., 20., 100.)
    }

    mod point_in_rect {
        mod when_point_inside {
            use crate::systems::bounce_balls::tests::f;

            #[test]
            fn upper_left() {
                assert!(f(40., 0.));
            }

            #[test]
            fn upper_right() {
                assert!(f(60., 0.));
            }

            #[test]
            fn lower_left() {
                assert!(f(40., 100.));
            }

            #[test]
            fn lower_right() {
                assert!(f(60., 100.));
            }

            #[test]
            fn center() {
                assert!(f(50., 50.));
            }
        }

        mod when_point_outside {
            use crate::systems::bounce_balls::tests::f;

            #[test]
            fn above() {
                assert!(!f(50., -0.1));
            }

            #[test]
            fn below() {
                assert!(!f(50., 100.1));
            }

            #[test]
            fn right() {
                assert!(!f(60.1, 50.));
            }

            #[test]
            fn left() {
                assert!(!f(39.9, 50.));
            }
        }
    }
}
