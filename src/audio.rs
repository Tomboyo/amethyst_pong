use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    prelude::WorldExt,
    shred::World,
};

pub struct Sounds {
    pub bounce: SourceHandle,
    pub score: SourceHandle,
}

pub fn initialize_audio(world: &mut World) {
    let sounds = {
        let loader = world.read_resource::<Loader>();
        let load_audio = |path| loader.load(path, OggFormat, (), &world.read_resource());
        Sounds {
            bounce: load_audio("sound/bounce.ogg"),
            score: load_audio("sound/score.ogg"),
        }
    };

    world.insert(sounds);
}

pub fn play_sound(sound: &SourceHandle, storage: &AssetStorage<Source>, output: &Output) {
    let sound = storage.get(&sound).unwrap();
    output.play_once(sound, 1.0);
}
