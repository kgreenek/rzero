use acf;
use acf::Acf;
use sample::{ring_buffer, Frame, Sample};
use std::cmp::Ordering;

pub trait PitchExtractor<F>
where
    F: Frame,
{
    fn n_channels() -> usize;

    /// Adds the new frame and returns the pitch for the buffer with the frame incorporated.
    fn next(&mut self, new_frame: F) -> &[usize];

    /// Returns the last calculated pitch.
    fn current(&self) -> &[usize];
}

#[derive(Clone)]
pub struct PitchEstimate<S>
where
    S: Sample,
{
    pitch: usize,
    /// The confidence of this pitch. Lower is better.
    /// If pitch is 0, then this should be ignored.
    confidence: S::Float,
}

impl<S> PitchEstimate<S>
where
    S: Sample,
{
    pub fn new() -> Self {
        PitchEstimate {
            pitch: 0,
            confidence: S::Float::equilibrium(),
        }
    }
}

impl<S> Ord for PitchEstimate<S>
where
    S: Sample,
{
    fn cmp(&self, other: &PitchEstimate<S>) -> Ordering {
        if self.pitch == 0 {
            if other.pitch == 0 {
                return Ordering::Equal;
            }
            return Ordering::Greater;
        }
        if other.pitch == 0 {
            return Ordering::Less;
        }
        if self.confidence < other.confidence {
            return Ordering::Less;
        }
        if self.confidence > other.confidence {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl<S> PartialOrd for PitchEstimate<S>
where
    S: Sample,
{
    fn partial_cmp(&self, other: &PitchEstimate<S>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> PartialEq for PitchEstimate<S>
where
    S: Sample,
{
    fn eq(&self, other: &PitchEstimate<S>) -> bool {
        self.pitch == other.pitch && self.confidence == other.confidence
    }
}

impl<S> Eq for PitchEstimate<S>
where
    S: Sample,
{}

#[derive(Clone)]
pub struct YinPitchExtractor<F>
where
    F: Frame,
{
    acf: acf::DiffSquaredAcf<F>,
    acf_norm: Vec<F::Float>,
    pitch_estimates: ring_buffer::Fixed<Vec<Vec<PitchEstimate<F::Sample>>>>,
    /// Stores the last best pitch value.
    pitch: Vec<usize>,
}

impl<F> YinPitchExtractor<F>
where
    F: Frame,
{
    pub fn new(window_size: usize, max_t: usize) -> Self {
        let mut acf_norm = vec![F::Float::equilibrium(); max_t];
        acf_norm[0] = acf_norm[0].map(|_| <<F::Float as Frame>::Sample as Sample>::identity());
        YinPitchExtractor {
            acf: acf::DiffSquaredAcf::new(window_size, max_t),
            acf_norm: acf_norm,
            pitch_estimates: ring_buffer::Fixed::from(vec![
                vec![PitchEstimate::new(); F::n_channels()];
                // TODO(kgreenek): Pass this as a parameter.
                max_t * 2
            ]),
            pitch: vec![0; F::n_channels()],
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

    fn update_pitch_estimates(&mut self) {
        let mut pitch_estimates = vec![PitchEstimate::new(); F::n_channels()];
        for channel in 0..F::n_channels() {
            for (frame_index, &acf_norm_frame) in self.acf_norm[..].iter().enumerate() {
                let acf_norm_sample = acf_norm_frame.channel(channel).unwrap();
                if *acf_norm_sample < 0.1.to_sample() {
                    pitch_estimates[channel].pitch = frame_index;
                    pitch_estimates[channel].confidence = *acf_norm_sample;
                    break;
                }
            }
        }
        self.pitch_estimates.push(pitch_estimates);
    }

    fn update_pitch(&mut self) {
        // TODO(kgreenek): There must be a fancy functional way of doing this.
        let mut best_pitch_estimate_frame = vec![PitchEstimate::new(); F::n_channels()];
        for pitch_estimate_frame in self.pitch_estimates.iter() {
            for (channel, pitch_estimate) in pitch_estimate_frame.iter().enumerate() {
                if *pitch_estimate < best_pitch_estimate_frame[channel] {
                    best_pitch_estimate_frame[channel] = pitch_estimate.clone();
                }
            }
        }
        for (channel, pitch_estimate) in best_pitch_estimate_frame.iter().enumerate() {
            self.pitch[channel] = pitch_estimate.pitch;
        }
    }
}

impl<F> PitchExtractor<F> for YinPitchExtractor<F>
where
    F: Frame,
{
    #[inline]
    fn n_channels() -> usize {
        F::n_channels()
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
    fn next(&mut self, new_frame: F) -> &[usize] {
        self.acf.next(new_frame);
        self.update_acf_norm();
        self.update_pitch_estimates();
        self.update_pitch();
        self.current()
    }

    fn current(&self) -> &[usize] {
        self.pitch.as_slice()
    }
}
