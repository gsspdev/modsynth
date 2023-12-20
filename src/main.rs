use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::{Duration, Instant};
use cpal::SampleRate;
use cpal::{Device, Host, StreamConfig};

pub struct AudioEnvironment {
    host: Host,
    device: Device,
    config: StreamConfig,
    sample_rate: SampleRate,
}


impl AudioEnvironment {
    fn audio_prepare() -> Self {
        let host: Host = cpal::default_host();
        let device: Device = host
            .default_output_device()
            .expect("Failed to find default output device.");
        let config: StreamConfig = device
            .default_output_config()
            .expect("Failed to get default output config")
            .config();
        let sample_rate: SampleRate = config.sample_rate;
        return AudioEnvironment {
            host,
            device,
            config,
            sample_rate,
        };
    }
}

pub fn run() {
    let audio_env = AudioEnvironment::audio_prepare();
    const update_interval: Duration = Duration::from_secs(60);
    let mut time: u32 = 0;

    let sample_rate = 44100 as f32;
    let mut samples_played = 0f32;

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let stream = audio_env.device
        .build_output_stream(
            &audio_env.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    let time = samples_played / sample_rate;
                    *sample = time.sin();
                    samples_played += 1.0;
                }
            },
            err_fn, Some(Duration::from_secs(60))
        )
        .expect("Failed to build output stream.");

    stream.play().expect("Failed to start audio stream.");

    println!("Press Enter to stop the audio.");
    let _ = std::io::stdin().read_line(&mut String::new());

    loop {
        let start = Instant::now();
        let mut output = 0;
        println!("Output at time {}: {}", time, output);
    }
}

fn main() {
    run();

}
