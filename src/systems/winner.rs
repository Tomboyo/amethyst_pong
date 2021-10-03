use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World, Write, WriteStorage,
};
use amethyst::prelude::Builder;
use amethyst::shred::{ReadExpect, ResourceId};
use amethyst::ui::UiText;

use crate::audio::{play_sound, Sounds};
use crate::pong::{Ball, BallSpawnTimeout, ScoreUi, Scores, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

#[derive(SystemData)]
pub struct Data<'s> {
    balls: ReadStorage<'s, Ball>,
    transforms: ReadStorage<'s, Transform>,
    ui_text: WriteStorage<'s, UiText>,
    entities: Entities<'s>,
    lazy_update: Read<'s, LazyUpdate>,
    output: Option<Read<'s, Output>>,
    time: Read<'s, Time>,
    audio_storage: Read<'s, AssetStorage<Source>>,
    score_ui: ReadExpect<'s, ScoreUi>,
    sounds: ReadExpect<'s, Sounds>,
    scores: Write<'s, Scores>,
}

impl<'s> System<'s> for WinnerSystem {
    type SystemData = Data<'s>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (ball, transform, entity) in (&data.balls, &data.transforms, &data.entities).join() {
            let x = transform.translation().x;
            if x <= -ball.radius || x >= ARENA_WIDTH + ball.radius {
                if x <= -ball.radius {
                    data.scores.right += 1;
                    data.ui_text.get_mut(data.score_ui.right).unwrap().text =
                        data.scores.right.to_string();
                } else {
                    data.scores.left += 1;
                    data.ui_text.get_mut(data.score_ui.left).unwrap().text =
                        data.scores.left.to_string();
                }

                data.entities.delete(entity).unwrap();
                if let Some(ref output) = data.output {
                    play_sound(&data.sounds.score, &data.audio_storage, output);
                };

                data.lazy_update
                    .create_entity(&data.entities)
                    .with(BallSpawnTimeout {
                        expiry: data.time.absolute_time_seconds() + 3.0,
                        velocity: [-ball.velocity[0], -ball.velocity[1]],
                    })
                    .build();
            }
        }
    }
}
