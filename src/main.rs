use std::io::Read;
use std::time::{Instant};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioEnvironment {
    device: cpal::Device,
    config: cpal::StreamConfig,
}

impl AudioEnvironment {
    fn audio_prepare() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Failed to find default output device.");

        let config = device
            .default_output_config()
            .expect("Failed to get default output config")
            .config();

        AudioEnvironment{ device, config }
    }
}

pub fn run() {
    let audio_env = AudioEnvironment::audio_prepare();
    let sample_rate = audio_env.config.sample_rate.0 as f32;
    let mut samples_played = 0f32;

    // Shared stream handle
    let stream_handle = Arc::new(Mutex::new(None));
    let stream_handle_clone = Arc::clone(&stream_handle);

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let _time: u32 = 0;

    let stream = audio_env.device
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

    // Store the stream in the shared handle
    *stream_handle_clone.lock().unwrap() = Some(stream);

    // Start the stream
    if let Some(ref stream) = *stream_handle_clone.lock().unwrap() {
        stream.play().expect("Failed to start audio stream.");
    }

    println!("440hz sine wave playing. Press enter to exit.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    // Stop the stream on user input
    if let Some(ref stream) = *stream_handle.lock().unwrap() {
        stream.pause().expect("Failed to pause the audio stream.");
    };
}

fn main() {
    run();

}
