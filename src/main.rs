#![allow(unused_imports)]
#![allow(dead_code)]

use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::scaling::divide_by_N_sqrt;

use modsynth::live_visualization::live_visualization;

use modsynth::audio_environment;
use modsynth::stream_handling::run;

fn main() {

    // let samples: &[f32] = &[0.0, 3.14, 2.718, -1.0, -2.0, -4.0, 7.0, 7.0];
    // let hann_window = hann_window(&samples[0..8]);
    // let spectrum_hann_window = samples_fft_to_spectrum(
    //     &hann_window,
    //     44100,
    //     FrequencyLimit::All,
    //     Some(&divide_by_N_sqrt),
    //     ).unwrap();
    //
    // for (fr, fr_val) in spectrum_hann_window.data().iter() {
    //     println!("{}: {}", fr, fr_val)
    // }
    loop {
        run();
        live_visualization();
    }
}

