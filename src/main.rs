mod audio_host;
// mod available_hosts;
use cpal::*;

fn main() {
    // Assuming the module is named `audio_host`
    // let audio_playing = audio_host::initialize_audio_stream();
    audio_host::initialize_audio_stream();
    dbg!(cpal::available_hosts());

}
