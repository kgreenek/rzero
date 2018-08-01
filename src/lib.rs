extern crate sample;

pub mod acf;
pub mod pitch_extractor;

use pitch_extractor::{PitchExtractor, YinPitchExtractor};

pub struct PitchExtractorContainer {
    pitch_extractor: YinPitchExtractor<[f32; 1]>,
}

impl PitchExtractorContainer {
    fn new() -> Self {
        Self{pitch_extractor: YinPitchExtractor::<[f32; 1]>::new(100, 150)}
    }
}

#[no_mangle]
pub extern "C" fn rzero_extract_pitch(
    pitch_extractor_ptr: *mut PitchExtractorContainer,
    input_ptr: *const f32,
    length: usize,
    sample_rate: f64,
) -> f32 {
    let container = unsafe { &mut *pitch_extractor_ptr };
    let pitch_extractor = &mut container.pitch_extractor;
    let input = unsafe { std::slice::from_raw_parts(input_ptr, length) };
    let frames = sample::slice::to_frame_slice::<&[f32], [f32; 1]>(input).unwrap();
    pitch_extractor.add_frames(frames);
    let pitch_samples = pitch_extractor.extract_pitch();
    let pitch = sample_rate / (pitch_samples[0] as f64);
    pitch as f32
}

#[no_mangle]
pub extern "C" fn rzero_new_pitch_extractor() -> *mut PitchExtractorContainer {
    let _container = unsafe {
        std::mem::transmute(Box::new(PitchExtractorContainer::new()))
    };
    _container
}

#[no_mangle]
pub extern "C" fn rzero_free_pitch_extractor(pitch_extractor_ptr: *mut PitchExtractorContainer) {
    let _container: Box<PitchExtractorContainer> =
        unsafe{ std::mem::transmute(pitch_extractor_ptr) };
    // Drop _container automatically.
}
