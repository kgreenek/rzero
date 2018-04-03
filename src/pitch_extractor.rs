use acf;
use acf::Acf;
use sample::{Frame, Sample};

pub trait PitchExtractor<F> where F: Frame {
    fn n_channels() -> usize;

    /// Adds the new frame and returns the pitch for the buffer with the frame incorporated.
    fn next(&mut self, new_frame: F) -> &[usize];

    /// Adds a new frame without doing the expensive calculation to update the pitch.
    fn add_frame(&mut self, new_frame: F);

    /// Adds the frames without doing the expensive calculation to update the pitch.
    fn add_frames(&mut self, new_frames: &[F]);

    /// Calculates the pitch. Use this after calling `add_frame` one or more times to only do the
    /// expensive pitch calculation when needed, instead of every frame.
    fn extract_pitch(&mut self) -> &[usize];

    /// Returns the last calculated pitch.
    fn pitch(&self) -> &[usize];
}

#[derive(Clone)]
pub struct YinPitchExtractor<F> where F: Frame {
    acf: acf::DiffSquaredAcf<F>,
    acf_norm: Vec<F::Float>,
    pitch: Vec<usize>,
    dirty: bool,
}

impl<F> YinPitchExtractor<F> where F: Frame {
    pub fn new(window_size: usize, max_t: usize) -> Self {
        let mut acf_norm = vec![F::Float::equilibrium(); max_t];
        acf_norm[0] = acf_norm[0].map(|_| { <F::Float as Frame>::Sample::identity() });
        YinPitchExtractor {
            acf: acf::DiffSquaredAcf::new(window_size, max_t),
            acf_norm: acf_norm,
            pitch: vec![0; F::n_channels()],
            dirty: false,
        }
    }

    fn update_acf_norm(&mut self) {
        let mut acf_sum = F::Float::equilibrium();
        let current_acf = &self.acf.current();
        for (i, value) in current_acf[..self.acf_norm.len()].iter().enumerate() {
            if i == 0 {
                continue;
            }
            acf_sum = acf_sum.add_amp(value.to_signed_frame());
            self.acf_norm[i] = acf_sum.zip_map(*value, |acf_sum_sample, value_sample| {
                value_sample / (acf_sum_sample / (i as f32).to_sample())
            });
        }
    }
}

impl<F> PitchExtractor<F> for YinPitchExtractor<F> where F: Frame {
    #[inline]
    fn n_channels() -> usize {
        F::n_channels()
    }

    fn next(&mut self, new_frame: F) -> &[usize] {
        self.add_frame(new_frame);
        self.extract_pitch()
    }

    fn add_frame(&mut self, new_frame: F) {
        self.dirty = true;
        self.acf.next(new_frame);
    }

    fn add_frames(&mut self, new_frames: &[F]) {
        self.dirty = true;
        for new_frame in new_frames {
            self.acf.next(*new_frame);
        }
    }

    /// Extracts the pitch from the frames that have been added with the `add_frame` method. The
    /// pitch is calculated lazily, so the work to actually calculate the pitch isn't done until
    /// this method is called.
    ///
    /// ```
    /// extern crate rzero;
    ///
    /// use rzero::pitch_extractor::{PitchExtractor, YinPitchExtractor};
    ///
    /// fn main() {
    ///   let mut pitch_extractor = YinPitchExtractor::<[f32; 1]>::new(2, 8);
    ///   pitch_extractor.add_frames(
    ///       &[[0.0], [1.0], [0.0], [-1.0], [0.0], [1.0], [0.0], [-1.0], [0.0]][..]);
    ///   assert_eq!(pitch_extractor.extract_pitch(), [4]);
    /// }
    /// ```
    fn extract_pitch(&mut self) -> &[usize] {
        if !self.dirty {
            return self.pitch();
        }
        self.dirty = false;
        self.update_acf_norm();
        for channel in 0..F::n_channels() {
            self.pitch[channel] = 0;
            let mut acf_norm_min_value = *self.acf_norm[0].channel(channel).unwrap();
            for (frame_index, &acf_norm_frame) in self.acf_norm[..].iter().enumerate() {
                let acf_norm_sample = acf_norm_frame.channel(channel).unwrap();
                if *acf_norm_sample < 0.1.to_sample() {
                    self.pitch[channel] = frame_index;
                    break;
                }
                if *acf_norm_sample < acf_norm_min_value {
                    acf_norm_min_value = *acf_norm_sample;
                    self.pitch[channel] = frame_index;
                }
            }
        }
        self.pitch()
    }

    fn pitch(&self) -> &[usize] {
        self.pitch.as_slice()
    }
}
