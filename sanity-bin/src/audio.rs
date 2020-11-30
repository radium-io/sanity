extern crate amethyst;

use amethyst::{
    assets::AssetStorage,
    assets::Loader,
    audio::Mp3Format,
    audio::{output::Output, AudioSink, Source, SourceHandle},
    ecs::{World, WorldExt},
};

const INTRO_SOUND: &str = "sound/ship-voice.mp3";
const VO_SOUND: &str = "sound/take-care.mp3";
const MUSIC_TRACKS: &[&str] = &["sound/sanity-ost.mp3"];

pub struct Sounds {
    pub intro: SourceHandle,
    pub vo: SourceHandle,
}

use std::{iter::Cycle, vec::IntoIter};

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, Mp3Format, (), &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        (
            Sounds {
                intro: load_audio_track(&loader, &world, INTRO_SOUND),
                vo: load_audio_track(&loader, &world, VO_SOUND),
            },
            Music { music },
        )
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_intro(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.intro) {
            output.play_once(sound, 0.7);
        }
    }
}

pub fn play_vo(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.vo) {
            output.play_once(sound, 0.7);
        }
    }
}
