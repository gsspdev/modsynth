// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//
// pub fn initialize_audio_stream() {
//     let output_stream = prepare_audio_stream();
//
//     let output_stream_play = output_stream.play().unwrap();
//     // other implementation...
// }
//
// fn prepare_audio_stream() -> cpal::Stream {
//     let host = cpal::default_host();
//     let device = host.default_output_device().expect("no output device available");
//     let mut supported_configs_range = device.supported_output_configs()
//         .expect("error while querying configs");
//     let supported_config = supported_configs_range.next()
//         .expect("no supported config?!")
//         .with_max_sample_rate();
//     let config = supported_config.config();
//
//     let output_stream = device.build_output_stream(
//         &config,
//         move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//             // react to stream events and read or write stream data here.
//         },
//         move |err| {
//             // react to errors here.
//         },
//         None, // None=blocking, Some(Duration)=timeout
//     ).unwrap();
//
//     output_stream
// // pub fn play_audio() {
// //     stream.play().unwrap();
// //
// }

