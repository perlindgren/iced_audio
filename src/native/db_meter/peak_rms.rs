//! A DSP [`Detector`] that calculates the peak and rms levels of a stereo signal
use crate::core::audio_to_gui_stream;
///
/// [`Detector`]: ../db_meter/trait.Detector.html
use crate::native::db_meter::{peak, Detector, DetectorOutput};

use circular_queue::CircularQueue;

static RMS_WINDOW_SIZE_SEC: f32 = 0.3;
static RMS_BLOCK_SIZE: usize = 256;

#[allow(missing_debug_implementations)]
struct RmsCache {
    block_cache: CircularQueue<f32>,
    block_sum: f32,
    smps_to_cache: Vec<f32>,
    smps_not_cached: usize,
}

/// A DSP [`Detector`] that calculates the peak and rms levels of a stereo signal
///
/// This algorithm caches blocks of 256 samples for effeciency. The RMS window size
/// will always be a multiple of 256, so it may be a bit off the target of 300ms. For a
/// basic decibel meter, this slight innacurracy is worth the better performance.
///
/// [`Detector`]: ../db_meter/trait.Detector.html
#[allow(missing_debug_implementations)]
pub struct PeakRmsDetector {
    rms_window_size: usize,
    one_over_rms_window_size: f32,
    rms_block_size: usize,
    sample_rate: f32,
    left_rms_cache: Option<RmsCache>,
    right_rms_cache: Option<RmsCache>,
}

impl PeakRmsDetector {
    /// Creates a new `PeakRmsDetector`
    pub fn new() -> Self {
        Self {
            rms_window_size: 0,
            one_over_rms_window_size: 0.0,
            sample_rate: 0.0,
            rms_block_size: 0,
            left_rms_cache: None,
            right_rms_cache: None,
        }
    }

    fn rms_db(
        s1: &[f32],
        s2: &[f32],
        rms_cache: &mut RmsCache,
        one_over_rms_window_size: f32,
    ) -> Option<f32> {
        let mut rms_db: Option<f32> = None;

        // Retrieve the uncached samples in the `smps_to_cache` buffer
        let mut uncached_smps =
            &rms_cache.smps_to_cache[..rms_cache.smps_not_cached];

        let mut total_smps_not_cached =
            uncached_smps.len() + s1.len() + s2.len();

        let mut s1 = &s1[..];
        let mut s2 = &s2[..];

        // If there is a new sample block to cache
        if total_smps_not_cached >= RMS_BLOCK_SIZE {
            // While there are new sample blocks to cache
            while total_smps_not_cached >= RMS_BLOCK_SIZE {
                let mut sum: f32 = 0.0;

                // All of the next block is in uncached_smps and s1
                if uncached_smps.len() + s1.len() >= RMS_BLOCK_SIZE {
                    let s1_part = &s1[..RMS_BLOCK_SIZE - uncached_smps.len()];

                    // Calculate the sum of squares in this block
                    for smp in uncached_smps.iter() {
                        sum += (*smp) * (*smp);
                    }
                    for smp in s1_part.iter() {
                        sum += (*smp) * (*smp);
                    }

                    // Remove cached samples in this block
                    uncached_smps = &uncached_smps[0..0];
                    s1 = &s1[s1_part.len()..];
                    rms_cache.smps_not_cached = 0;
                }
                // All of the next block is in s1 and s2
                else {
                    let s2_part = &s2[..RMS_BLOCK_SIZE - s1.len()];

                    // Calculate the sum of squares in this block
                    for smp in s1.iter() {
                        sum += (*smp) * (*smp);
                    }
                    for smp in s2_part.iter() {
                        sum += (*smp) * (*smp);
                    }

                    // Remove cached samples in this block
                    s1 = &s1[0..0];
                    s2 = &s2[s2_part.len()..];
                };

                // Store the new block sum in the block cache
                if let Some(popped) = rms_cache.block_cache.push(sum) {
                    rms_cache.block_sum -= popped;
                }
                rms_cache.block_sum += sum;

                total_smps_not_cached -= RMS_BLOCK_SIZE;
            }

            // If block cache is full, compute the new RMS
            if rms_cache.block_cache.is_full() {
                // Calculate the RMS
                rms_db = Some(crate::core::math::amplitude_to_db_f32(
                    (rms_cache.block_sum * one_over_rms_window_size).sqrt(),
                ));
            }
        }

        // Store remaining new uncached samples in the `smps_to_cache` buffer
        if s1.len() + s2.len() != 0 {
            rms_cache.smps_to_cache[rms_cache.smps_not_cached
                ..(rms_cache.smps_not_cached + s1.len())]
                .copy_from_slice(s1);
            rms_cache.smps_not_cached += s1.len();

            rms_cache.smps_to_cache[rms_cache.smps_not_cached
                ..(rms_cache.smps_not_cached + s2.len())]
                .copy_from_slice(s2);
            rms_cache.smps_not_cached += s2.len();
        }

        rms_db
    }
}

impl Default for PeakRmsDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Detector for PeakRmsDetector {
    fn update_sample_rate(&mut self, sample_rate: f32) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;

            let rms_window_size =
                (RMS_WINDOW_SIZE_SEC * sample_rate as f32).round();
            self.rms_block_size =
                (rms_window_size / RMS_BLOCK_SIZE as f32).round() as usize;
            self.rms_window_size = self.rms_block_size * RMS_BLOCK_SIZE;
            self.one_over_rms_window_size = 1.0 / self.rms_window_size as f32;

            let mut left_vec: Vec<f32> = Vec::new();
            left_vec.resize(RMS_BLOCK_SIZE, 0.0);
            let mut right_vec: Vec<f32> = Vec::new();
            right_vec.resize(RMS_BLOCK_SIZE, 0.0);

            self.left_rms_cache = Some(RmsCache {
                block_cache: CircularQueue::with_capacity(self.rms_block_size),
                block_sum: 0.0,
                smps_to_cache: left_vec,
                smps_not_cached: 0,
            });

            self.right_rms_cache = Some(RmsCache {
                block_cache: CircularQueue::with_capacity(self.rms_block_size),
                block_sum: 0.0,
                smps_to_cache: right_vec,
                smps_not_cached: 0,
            });
        }
    }

    fn process(
        &mut self,
        left_stream: &audio_to_gui_stream::Consumer,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        _delta_gui_time: f32,
    ) -> DetectorOutput {
        let mut output = DetectorOutput::empty();

        left_stream.read_access(|s1: &[f32], s2: &[f32]| {
            if s1.len() + s2.len() > 0 {
                // calculate peak
                output.left_peak_db = Some(peak::calc_peak_db(s1, s2));

                if let Some(left_rms_cache) = &mut self.left_rms_cache {
                    output.left_bar_db = Self::rms_db(
                        s1,
                        s2,
                        left_rms_cache,
                        self.one_over_rms_window_size,
                    );
                }
            }
        });

        if let Some(right_stream) = right_stream {
            right_stream.read_access(|s1: &[f32], s2: &[f32]| {
                if s1.len() + s2.len() > 0 {
                    // calculate peak
                    output.right_peak_db = Some(peak::calc_peak_db(s1, s2));

                    if let Some(right_rms_cache) = &mut self.right_rms_cache {
                        output.right_bar_db = Self::rms_db(
                            s1,
                            s2,
                            right_rms_cache,
                            self.one_over_rms_window_size,
                        );
                    }
                }
            });
        }

        output
    }

    fn clear(&mut self) {
        if let Some(left_rms_cache) = &mut self.left_rms_cache {
            left_rms_cache.smps_not_cached = 0;
            left_rms_cache.block_cache.clear();
            left_rms_cache.block_sum = 0.0;
        }

        if let Some(right_rms_cache) = &mut self.right_rms_cache {
            right_rms_cache.smps_not_cached = 0;
            right_rms_cache.block_cache.clear();
            right_rms_cache.block_sum = 0.0;
        }
    }
}
