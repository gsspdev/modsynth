trait WaveShape {
    fn generate_sample(time: f32, frequency: f32, sample_rate: f32) -> f32;
}

struct SineWave;
struct SquareWave;
// ... other waveforms

impl Waveform for SineWave {
    fn generate_sample(time: f32, frequency: f32, _sample_rate: f32) -> f32 {
        (time * frequency * 2.0 * std::f32::consts::PI).sin()
    }
}

impl Waveform for SquareWave {
    fn generate_sample(time: f32, frequency: f32, _sample_rate: f32) -> f32 {
        if (time * frequency * 2.0 * std::f32::consts::PI).sin() >= 0.0 { 1.0 } else { -1.0 }
    }
}

// Usage in your audio callback
let waveform: Box<dyn Waveform> = Box::new(SineWave);
// or Box::new(SquareWave) etc.

*sample = waveform.generate_sample(time, pitch, sample_rate);
