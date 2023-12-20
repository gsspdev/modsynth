use std::time::{Instant};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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

    stream.play().expect("Failed to start audio stream.");

    println!("Press Enter to stop the audio.");
    let _ = std::io::stdin().read_line(&mut String::new());

    loop {
        let _start = Instant::now();
        let _output = 0;
    }
}

fn main() {
    run();

}
