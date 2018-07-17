extern crate sample;

pub mod acf;
pub mod pitch_extractor;

use pitch_extractor::{PitchExtractor, YinPitchExtractor};

#[no_mangle]
pub extern "C" fn rzero_extract_pitch(
    pitch_extractor_ptr: *mut YinPitchExtractor<[f32; 1]>,
    input_ptr: *const f32,
    length: usize,
    sample_rate: f64,
) -> f32 {
    let pitch_extractor = unsafe { &mut *pitch_extractor_ptr };
    let input = unsafe { std::slice::from_raw_parts(input_ptr, length as usize) };
    let frames = sample::slice::to_frame_slice::<&[f32], [f32; 1]>(input).unwrap();
    pitch_extractor.add_frames(frames);
    let pitch_samples = pitch_extractor.extract_pitch();
    let pitch = sample_rate / (pitch_samples[0] as f64);
    pitch as f32
}

#[no_mangle]
pub extern "C" fn rzero_new_pitch_extractor() -> *mut YinPitchExtractor<[f32; 1]> {
    let _pitch_extractor = unsafe {
        std::mem::transmute(Box::new(YinPitchExtractor::<[f32; 1]>::new(100, 150)))
    };
    _pitch_extractor
}

#[no_mangle]
pub extern "C" fn rzero_free_pitch_extractor(
    pitch_extractor_ptr: *mut YinPitchExtractor<[f32; 1]>
) {
    let _pitch_extractor: Box<YinPitchExtractor<[f32; 1]>> =
        unsafe{ std::mem::transmute(pitch_extractor_ptr) };
    // Drop _pitch_extractor automatically.
}

//#[no_mangle]
//pub extern "C" fn extract_pitch_raw_old(
//    input_ptr: *mut f32,
//    output_ptr: *mut f32,
//    length: i32,
//    input_channels: i32,
//    output_channels: i32,
//    sample_rate: f64,
//) -> f32 {
//    unsafe {
//        let input = std::slice::from_raw_parts_mut(input_ptr, (input_channels * length) as usize);
//        return extract_pitch(input, sample_rate);
//    }
//}
//
//pub extern "C" fn extract_pitch(input: &[f32], sample_rate: f64) -> f32 {
//    let window_size = 512 as usize;
//    let min_hz = 100.0;
//    let max_hz = 1000.0;
//    let min_sample = sample_from_hz(max_hz, sample_rate);
//    let max_sample = sample_from_hz(min_hz, sample_rate);
//    let diff2 = diff_squared(input, window_size, min_sample, max_sample);
//    let mut min_diff_sample = 0;
//    let mut min_diff_value = 1.0;
//    for i in 0..diff2.len() {
//        if diff2[i] < min_diff_value {
//            println!("new min: {} at {}", diff2[i], i);
//            min_diff_sample = i + min_sample;
//            min_diff_value = diff2[i];
//        }
//    }
//    hz_from_sample(min_diff_sample, sample_rate) as f32
//}
//
//pub fn hz_from_sample(sample: usize, sample_rate: f64) -> f64 {
//    sample_rate / (sample as f64)
//}
//
//pub fn sample_from_hz(hz: f64, sample_rate: f64) -> usize {
//    (sample_rate / hz) as usize
//}
//
//pub fn diff_squared(
//    input: &[f32],
//    window_size: usize,
//    start_sample: usize,
//    end_sample: usize,
//) -> Vec<f32> {
//    if (end_sample - 1) + (window_size - 1) >= input.len() {
//        println!("Frame too short. Ignorning...");
//        return Vec::new();
//    }
//    //assert!(start_sample < input.len() - window_size);
//    let output_size = end_sample - start_sample;
//    let mut output: Vec<f32> = vec![0.0; output_size];
//    //let mut output = Vec::new();
//    for t in start_sample..end_sample {
//        for i in 0..window_size {
//            //(input[t + i] - input[i]).powf(2.0);
//            output[t] = (input[t + i] - input[i]).powf(2.0);
//            //output.push((input[t + i] - input[i]).powf(2.0));
//        }
//    }
//    output
//}
