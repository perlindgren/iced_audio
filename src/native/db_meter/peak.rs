//! A DSP [`Detector`] that calculates the peak levels of a stereo signal
use crate::core::audio_to_gui_stream;
///
/// [`Detector`]: ../db_meter/trait.Detector.html
use crate::native::db_meter::{Detector, DetectorOutput};

/// Calculates the peak dB of the two slices combined
pub fn calc_peak_db(s1: &[f32], s2: &[f32]) -> f32 {
    let mut max_peak: f32 = 0.0;

    for smp in s1.iter() {
        let abs_smp = smp.abs();
        if abs_smp > max_peak {
            max_peak = abs_smp;
        }
    }

    for smp in s2.iter() {
        let abs_smp = smp.abs();
        if abs_smp > max_peak {
            max_peak = abs_smp;
        }
    }

    crate::core::math::amplitude_to_db_f32(max_peak)
}

/// A DSP [`Detector`] that calculates the peak levels of a stereo signal
///
/// [`Detector`]: ../db_meter/trait.Detector.html
#[allow(missing_debug_implementations)]
#[derive(Default, Copy, Clone)]
pub struct PeakDetector;

impl PeakDetector {
    /// Creates a new `PeakDetector`
    pub fn new() -> Self {
        Self {}
    }
}

impl Detector for PeakDetector {
    fn update_sample_rate(&mut self, _sample_rate: f32) {}

    fn process(
        &mut self,
        left_stream: &audio_to_gui_stream::Consumer,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        _delta_gui_time: f32,
    ) -> DetectorOutput {
        let mut output = DetectorOutput::empty();

        left_stream.read_access(|s1: &[f32], s2: &[f32]| {
            let total_len = s1.len() + s2.len();

            if total_len > 0 {
                output.left_peak_db = Some(calc_peak_db(s1, s2));
                output.left_bar_db = output.left_peak_db;
            }
        });

        if let Some(right_stream) = right_stream {
            right_stream.read_access(|s1: &[f32], s2: &[f32]| {
                let total_len = s1.len() + s2.len();

                if total_len > 0 {
                    output.right_peak_db = Some(calc_peak_db(s1, s2));
                    output.right_bar_db = output.right_peak_db;
                }
            });
        }

        output
    }

    fn clear(&mut self) {}
}
