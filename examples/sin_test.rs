
let mut sin_osc  = SINOSC.lock().unwrap();
*amp_of_sample = sin_osc.generate_sample(time, pitch, sample_rate);