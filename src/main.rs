#![allow(unused_imports)]
#![allow(dead_code)]
mod audio_environment;
mod stream_handling;

use crate::audio_environment::AudioEnvironment;
use crate::stream_handling::run;
// use crate::stream_handling::prompt_user;

fn main() {
    // prompt_user();
    run();

}
