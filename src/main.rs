#![allow(unused_imports)]
#![allow(dead_code)]
use colored::*;

use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::scaling::divide_by_N_sqrt;

use modsynth::live_visualization::live_visualization;

use modsynth::{audio_environment, osc_math};
use modsynth::stream_handling::run;
use modsynth::osc_math::ShapeMath;
// use tui::widgets::canvas::Shape;

// use user_input::select_sound;

fn main() {
    // osc_math::test_osc_math();
    run();
    // live_visualization();
 }