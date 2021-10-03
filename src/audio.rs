use std::{iter::Cycle, vec::IntoIter};

use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    prelude::WorldExt,
    shred::World,
};

const MUSIC_TRACKS: &[&str] = &["sound/Computer_Music_All-Stars_-_Albatross_v2.ogg"];

pub struct Sounds {
    pub bounce: SourceHandle,
    pub score: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub fn initialize_audio(world: &mut World) {
    let (music, sounds) = {
        let loader = world.read_resource::<Loader>();
        let load_audio = |path: &str| loader.load(path, OggFormat, (), &world.read_resource());

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);

        (
            Music {
                music: MUSIC_TRACKS
                    .iter()
                    .map(|path| load_audio(path))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .cycle(),
            },
            Sounds {
                bounce: load_audio("sound/bounce.ogg"),
                score: load_audio("sound/score.ogg"),
            },
        )
    };

    world.insert(music);
    world.insert(sounds);
}

pub fn play_sound(sound: &SourceHandle, storage: &AssetStorage<Source>, output: &Output) {
    let sound = storage.get(&sound).unwrap();
    output.play_once(sound, 1.0);
}
