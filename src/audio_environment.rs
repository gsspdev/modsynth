use cpal::traits::{DeviceTrait, HostTrait };

pub struct AudioEnvironment {
    pub device: cpal::Device,
    pub config: cpal::StreamConfig,
}

impl AudioEnvironment {
    pub fn audio_prepare() -> Self {
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
