use std::sync::Mutex;
use lazy_static::lazy_static;

pub trait WaveShape {
    fn generate_sample(&self, _time: f32, _frequency: f32, _sample_rate: f32) -> f32 {
        let zero_amplitude: f32 = 0.0;
        return zero_amplitude
    }
}

pub struct SineWave {}

impl WaveShape for SineWave {
    fn generate_sample(&self, time: f32, frequency: f32, _sample_rate: f32) -> f32 {
       (time * frequency * 2.0 * std::f32::consts::PI).sin()
    }
}

lazy_static! {
    pub static ref SINOSC: Mutex<Box<dyn WaveShape + Send>> = Mutex::new(Box::new(SineWave {}));
}