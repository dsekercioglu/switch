use bevy::prelude::*;
use bevy::utils::tracing::Instrument;

pub struct AudioResource {
    music: Handle<AudioSource>,
}

pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music: Handle<AudioSource> = asset_server.load("Rhythm 3.mp3");
    let resource = AudioResource {
        music,
    };
    commands.insert_resource(resource);
}

pub fn play_audio(resource: Res<AudioResource>,
                  audio: Res<Audio>) {
    audio.play(resource.music.clone());
}