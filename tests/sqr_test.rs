use std::io::Read;
use cpal::traits::{DeviceTrait};

use crate::AudioEnvironment;

pub fn run() {
    let audio_env = AudioEnvironment::audio_prepare();
    let sample_rate = audio_env.config.sample_rate.0 as f32;
    let mut samples_played = 0f32;
    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);
    let _time: u32 = 0;

    let _stream = audio_env.device
        .build_output_stream(&audio_env.config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                let time = samples_played / sample_rate;
                let pitch = 440.0;
                let half_pitch = pitch / 2.0; // temp workaround, pitch was 2x higher than expected
                *sample = (time * half_pitch * 2.0 * std::f32::consts::PI).sin();
                samples_played += 1.0;
            }
        }, err_fn, None
        )
        .expect("Failed to build output stream.");

    println!("440hz sqr wave test. Enter to exit.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

}

pub fn main() {
    run();
}