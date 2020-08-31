//! The default DSP [`Detector`] for the [`RtWaveView`]
//!
//! [`Detector`]: ../rt_wave_view/trait.Detector.html
//! [`RtWaveView`]: ../rt_wave_view/struct.RtWaveView.html

use crate::core::{audio_to_gui_stream};
use crate::native::rt_wave_view::{PlotPoint, Detector, Plot};

#[inline]
fn min_max(s: &[f32]) -> PlotPoint {
    let mut p = PlotPoint {
        max: f32::MIN,
        min: f32::MAX,
    };

    for val in s.iter() {
        if *val > p.max {
            p.max = *val;
        }
        if *val < p.min {
            p.min = *val;
        }
    }

    p
}

/// The detection mode of a [`Detector`] for an [`RtWaveView`].
///
/// [`Detector`]: ../rt_wave_view/trait.Detector.html
/// [`RtWaveView`]: ../rt_wave_view/struct.RtWaveView.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    /// Use only the left/mono channel as the input.
    MonoOrLeftOnly,
    /// Use only the right channel as the input.
    RightOnly,
    /// Use both channels as input.
    Dual,
    /// Use both channels as input, but combine them into
    /// a mono signal. Uses more CPU.
    StereoToMono,
}

#[allow(missing_debug_implementations)]
struct Params {
    gain: f32,
    mode: Mode,
    sample_rate: f64,
    window_size: f64,
}

impl Params {
    pub fn new(
        gain: f32,
        mode: Mode,
        sample_rate: f64,
        window_size: f64,
    ) -> Self {
        Self {
            gain,
            mode,
            sample_rate,
            window_size,
        }
    }
}

/// The default DSP [`Detector`] for the [`RtWaveView`]
///
/// [`Detector`]: ../rt_wave_view/trait.Detector.html
/// [`RtWaveView`]: ../rt_wave_view/struct.RtWaveView.html
#[allow(missing_debug_implementations)]
pub struct PeakDetector {
    params: Params,
}

impl PeakDetector {
    /// Creates a new `PeakDetector`
    pub fn new(
        gain: f32,
        mode: Mode,
        sample_rate: f64,
        window_size: f64,
    ) -> Self {
        Self {
            params: Params::new(gain, mode, sample_rate, window_size),
        }
    }

    /// Sets the detection mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.params.mode = mode;
    }
}

impl Detector for PeakDetector {
    fn process(
        &mut self,
        left_stream: &audio_to_gui_stream::Consumer,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        left_plot: Option<&mut Plot>,
        right_plot: Option<&mut Plot>,
        _delta_gui_time: f32,
    ) {
        match self.params.mode {
            Mode::MonoOrLeftOnly => {
                if let Some(plot) = left_plot {
                    left_stream.read_access(|s1: &[f32], s2: &[f32]| {
                        let num_smps = s1.len() + s2.len();

                        let plot_points_per_sec = plot.len() as f64 / self.params.window_size;
                        let num_new_plot_points = num_smps as f64 * plot_points_per_sec / self.params.sample_rate;

                        /*
                        if num_smps != 0 && plot.len() != 0 {
                            let smps_per_plot_point = num_smps as f64 / plot.len() as f64;
                            let mut i_float: f64 = 0.0;

                            if s2.len() == 0 {
                                // All new samples are in s1

                                for plot_point in plot.iter_mut() {
                                    let next_i_float = i_float + smps_per_plot_point;

                                    let s1_part = &s1[i_float.round() as usize..next_i_float.round() as usize];

                                    *plot_point = min_max(s1_part);

                                    i_float = next_i_float;
                                }
                            } else {
                                // All new samples are in both

                                let mut end_index = smps_per_plot_point.round() as usize;
                                let mut i: usize = 0;

                                // plot all continous chunks in s1
                                while end_index <= s1.len() {
                                    let s1_part = &s1[i_float.round() as usize..end_index];

                                    plot[i] = min_max(s1_part);

                                    i_float += smps_per_plot_point;
                                    end_index = (i_float + smps_per_plot_point).round() as usize;

                                    i += 1;
                                }

                                // plot split chunks
                                let start_index = i_float.round() as usize;
                                i_float -= s1.len() as f64;
                                if start_index < s1.len() {
                                    let s1_end_part = &s1[start_index..];
                                    let s1_end_plot_point = min_max(s1_end_part);

                                    i_float += smps_per_plot_point;

                                    let s2_start_part = &s2[0..i_float.round() as usize];
                                    let s2_start_plot_point = min_max(s2_start_part);

                                    plot[i] = PlotPoint {
                                        max: f32::max(s1_end_plot_point.max, s2_start_plot_point.max),
                                        min: f32::min(s1_end_plot_point.min, s2_start_plot_point.min),
                                    };

                                    i += 1;
                                }

                                // plot all continous chunks in s2
                                end_index = (i_float + smps_per_plot_point).round() as usize;
                                while end_index <= s2.len() {
                                    let s2_part = &s2[i_float.round() as usize..end_index];

                                    plot[i] = min_max(s2_part);

                                    i_float += smps_per_plot_point;
                                    end_index = (i_float + smps_per_plot_point).round() as usize;

                                    i += 1;
                                }
                            }
                        }
                        */
                    });
                }
            }
            Mode::RightOnly => {

            }
            Mode::Dual => {

            }
            Mode::StereoToMono => {
                
            }
        }
    }

    fn clear(&mut self) {}

    fn set_window_size(&mut self, window_size: f32) {}

    fn set_sample_rate(&mut self, sample_rate: f32) {}

    fn set_gain(&mut self, gain: f32) {
        self.params.gain = gain as f32;
    }
}
