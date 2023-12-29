use std::io::Read;
use std::sync::Mutex;
use cpal::traits::{DeviceTrait};

use crate::audio_environment::AudioEnvironment;
// use crate::WaveShape;
use crate::osc_shapes::{SINOSC, WaveShape, SineWave};
// use SineWave::generate_sin;

pub fn run() {
    let audio_env = AudioEnvironment::audio_prepare();
    let sample_rate = audio_env.config.sample_rate.0 as f32;
    let current_sample = Mutex::new(0f32); // consider wrapping in cell or refcell or mutex
    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);
    let _time: u32 = 0;

    let _stream = audio_env.device
        .build_output_stream(&audio_env.config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut current_sample_lock = current_sample.lock().unwrap();

            for amp_of_sample in data.iter_mut() {
                let user_pitch: Option<f32> = Some(240.0); // will be expanded to prompt_user() function
                let default_pitch: f32 = 440.0;
                let pitch = user_pitch.unwrap_or(default_pitch);

                let time = *current_sample_lock / sample_rate;

                let sin_osc  = SINOSC.lock().unwrap();
                *amp_of_sample = sin_osc.generate_sample(time, pitch, sample_rate);

                // *amp_of_sample = (time * pitch * std::f32::consts::PI).sin();
                *current_sample_lock += 1.0;

                if *current_sample_lock <= 4000000.0 {
                    println!("amplitude: {}, sample: {}", amp_of_sample, *current_sample_lock);
                }
            }
        }, err_fn, None
        )
        .expect("Failed to build output stream.");

    println!("440hz sin wave test. Enter to exit.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

}
