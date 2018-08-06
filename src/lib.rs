extern crate sample;

pub mod acf;
pub mod pitch_extractor;

use pitch_extractor::{PitchExtractor, YinPitchExtractor};

const CHANNELS: usize = 1;
const WINDOW_SIZE: usize = 100;
const PITCH_MAX_T: usize = 150;
const RMS_THRESHOLD: f32 = 0.01;

type SampleT = f32;
type FrameT = [SampleT; CHANNELS];
type WindowSliceT = [FrameT; WINDOW_SIZE];

pub struct PitchExtractorContainer {
    pitch_extractor: YinPitchExtractor<FrameT>,
    rms: sample::rms::Rms<FrameT, WindowSliceT>,
}

impl PitchExtractorContainer {
    fn new() -> Self {
        let rms_window = sample::ring_buffer::Fixed::from(
            WindowSliceT::from([[0.0; CHANNELS]; WINDOW_SIZE]),
        );
        let rms = sample::rms::Rms::new(rms_window);
        Self {
            pitch_extractor: YinPitchExtractor::<FrameT>::new(WINDOW_SIZE, PITCH_MAX_T),
            rms: rms,
        }
    }

    fn add_frames(&mut self, new_frames: &[FrameT]) {
        self.pitch_extractor.add_frames(new_frames);
        for &frame in new_frames.iter() {
            self.rms.next(frame);
        }
    }

    fn pitch(&mut self, sample_rate: f64) -> f32 {
        let pitch_samples = self.pitch_extractor.extract_pitch();
        let pitch = sample_rate / (pitch_samples[0] as f64);
        pitch as f32
    }

    fn rms(&self) -> f32 {
        self.rms.current()[0]
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
    let input = unsafe { std::slice::from_raw_parts(input_ptr, length) };
    let frames = sample::slice::to_frame_slice::<&[SampleT], FrameT>(input).unwrap();
    container.add_frames(frames);
    if container.rms() >= RMS_THRESHOLD {
        return container.pitch(sample_rate);
    }
    0.0
}

#[no_mangle]
pub extern "C" fn rzero_new_pitch_extractor() -> *mut PitchExtractorContainer {
    let _container = unsafe { std::mem::transmute(Box::new(PitchExtractorContainer::new())) };
    _container
}

#[no_mangle]
pub extern "C" fn rzero_free_pitch_extractor(pitch_extractor_ptr: *mut PitchExtractorContainer) {
    let _container: Box<PitchExtractorContainer> =
        unsafe { std::mem::transmute(pitch_extractor_ptr) };
    // Drop _container automatically.
}
