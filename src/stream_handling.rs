use std::io::Read;
use std::sync::Mutex;
use cpal::traits::{DeviceTrait};

use crate::AudioEnvironment;
// use crate::user_input::user_input::user_pitch;

// pub fn prompt_user() {
//     println!("Enter a pitch in Hz: ");
//     std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
//     let user_pitch: f32 = user_input.trim().parse().expect("Please type a number!");
//     println!("Your pitch is: {}", user_pitch);
// }


pub fn run() {
    let audio_env = AudioEnvironment::audio_prepare();
    let sample_rate = audio_env.config.sample_rate.0 as f32;
    let samples_played = Mutex::new(0f32); // consider wrapping in cell or refcell or mutex
    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);
    let _time: u32 = 0;

    let _stream = audio_env.device
        .build_output_stream(&audio_env.config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut samples_played_lock = samples_played.lock().unwrap();

            for amp_of_sample in data.iter_mut() {
                let time = *samples_played_lock / sample_rate;
                let mut user_pitch: Option<f32> = Some(240.0);
                let default_pitch: f32 = 440.0;
                
                let pitch = user_pitch.unwrap_or(default_pitch);
                
                let half_pitch = pitch / 2.0; // temp workaround, pitch was 2x higher than expected

                *amp_of_sample = (time * half_pitch * 2.0 * std::f32::consts::PI).sin();
                *samples_played_lock += 1.0;

                // Prints time, pitch, amp, sample # - every 1000 samples
                if *samples_played_lock % 10000.0 == 0.0 {
                    println!("time: {}, pitch: {}, amp_of_sample: {}, samples_played_lock: {}", time, pitch, amp_of_sample, samples_played_lock);
                }
            }
        }, err_fn, None
        )
        .expect("Failed to build output stream.");

    println!("440hz sin wave test. Enter to exit.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

}
