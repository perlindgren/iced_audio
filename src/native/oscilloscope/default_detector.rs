//! The default DSP [`Detector`] for the [`Oscilloscope`]
//!
//! [`Detector`]: ../oscilloscope/trait.Detector.html
//! [`Oscilloscope`]: ../oscilloscope/struct.Oscilloscope.html

use crate::core::{audio_to_gui_stream, Normal};
use crate::native::oscilloscope::Detector;

use bit_mask_ring_buf::BMRingBuf;

/// The detection mode of a [`Detector`] for an [`Oscilloscope`].
///
/// [`Detector`]: trait.Detector.html
/// [`Oscilloscope`]: struct.Oscilloscope.html
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
struct Channel {
    buffer: BMRingBuf<f32>,
    latest_window_phase: f32,
    buffer_i: isize,
}

impl Channel {
    fn process(
        &mut self,
        s1: &[f32],
        s2: &[f32],
        plot: Option<&mut [f32]>,
        plot_2: Option<&mut [f32]>,
        params: &Params,
    ) {
        // Write latest data to the window buffer.
        self.buffer.write_latest_2(s1, s2, self.buffer_i);

        // Move buffer_i forward and constrain it so it doesn't keep growing.
        let samples_elapsed = s1.len() + s2.len();
        self.buffer_i = self
            .buffer
            .constrain(self.buffer_i + samples_elapsed as isize);

        // Find the phase inside the time window of the latest sample.
        let num_windows_elapsed = self.latest_window_phase
            + (samples_elapsed as f32 * params.smp_to_window_phase_ratio);
        let do_write = num_windows_elapsed >= 1.0;
        self.latest_window_phase = num_windows_elapsed.fract();

        if do_write {
            // Get the time offset from the latest sample to the beginning of the latest complete window.
            let mut phase =
                self.latest_window_phase + params.phase.value() as f32;
            if phase >= 1.0 {
                phase -= 1.0;
            }
            let time_offset = (phase * params.window_size) + params.window_size;

            // Get the floating index of the sample at the beginning of the latest complete window.
            let mut float_index =
                self.buffer_i as f32 - (time_offset * params.sample_rate);

            // Find the positive floating index so casting to isize can be used instead of .floor()
            if float_index < 0.0 {
                let float_index_floor = float_index.floor();
                let offset = float_index - float_index_floor;
                float_index = self.buffer.constrain(float_index_floor as isize)
                    as f32
                    + offset;
            }

            if let Some(plot) = plot {
                let plot_index_delta = (params.window_size / plot.len() as f32)
                    * params.sample_rate;

                for plot_value in plot.iter_mut() {
                    // Linearly interpolate value

                    let float_index_floor = float_index as isize;
                    let inter_smp_frac = float_index - float_index_floor as f32;

                    let smp1 = self.buffer[float_index_floor];
                    let smp2 = self.buffer[float_index_floor + 1];

                    *plot_value = (smp1
                        + ((smp2 - smp1) * inter_smp_frac as f32))
                        * params.gain;

                    float_index += plot_index_delta;
                }

                if let Some(plot_2) = plot_2 {
                    plot_2.copy_from_slice(plot);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.latest_window_phase = 0.0;
        self.buffer_i = 0;
    }
}

#[allow(missing_debug_implementations)]
struct Params {
    window_size: f32,
    sample_rate: f32,
    gain: f32,
    phase: Normal,
    mode: Mode,
    sample_rate_recip: f32,
    smp_to_window_phase_ratio: f32,
}

impl Params {
    pub fn new(
        window_size: f32,
        sample_rate: f32,
        gain: f32,
        phase: Normal,
        mode: Mode,
    ) -> Self {
        let sample_rate_recip = 1.0 / sample_rate;
        let smp_to_window_phase_ratio = sample_rate_recip / window_size;

        Self {
            window_size,
            sample_rate,
            gain: gain as f32,
            phase,
            mode,
            sample_rate_recip,
            smp_to_window_phase_ratio,
        }
    }

    pub fn update_window_size(&mut self, window_size: f32) {
        self.window_size = window_size;
        self.smp_to_window_phase_ratio = self.sample_rate_recip / window_size;
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.sample_rate_recip = 1.0 / sample_rate;
        self.smp_to_window_phase_ratio =
            self.sample_rate_recip / self.window_size;
    }
}

/// The default DSP [`Detector`] for the [`Oscilloscope`]
///
/// [`Detector`]: ../oscilloscope/trait.Detector.html
/// [`Oscilloscope`]: ../oscilloscope/struct.Oscilloscope.html
#[allow(missing_debug_implementations)]
pub struct DefaultDetector {
    params: Params,
    left_channel: Option<Channel>,
    right_channel: Option<Channel>,
    temp_plots: Option<(Vec<f32>, Vec<f32>)>,
}

impl DefaultDetector {
    /// Creates a new `DefaultDetector`
    pub fn new(
        window_size: f32,
        sample_rate: f32,
        gain: f32,
        phase: Normal,
        mode: Mode,
    ) -> Self {
        let mut new_self = Self {
            params: Params::new(window_size, sample_rate, gain, phase, mode),
            left_channel: None,
            right_channel: None,
            temp_plots: None,
        };

        new_self.allocate_buffers();

        new_self
    }

    fn allocate_buffers(&mut self) {
        if self.params.mode == Mode::RightOnly {
            self.left_channel = None;
        } else {
            let buffer_size = (self.params.window_size * 2.0 * self.params.sample_rate) as usize;

            if let Some(channel) = &mut self.left_channel {
                channel.buffer.set_len(buffer_size)
            } else {
                self.left_channel = Some(Channel {
                    buffer: BMRingBuf::from_len(buffer_size),
                    latest_window_phase: 0.0,
                    buffer_i: 0,
                });
            }
        }

        if self.params.mode == Mode::MonoOrLeftOnly {
            self.right_channel = None;
        } else {
            let buffer_size = (self.params.window_size * 2.0 * self.params.sample_rate) as usize;

            if let Some(channel) = &mut self.right_channel {
                channel.buffer.set_len(buffer_size);
            } else {
                self.right_channel = Some(Channel {
                    buffer: BMRingBuf::from_len(buffer_size),
                    latest_window_phase: 0.0,
                    buffer_i: 0,
                });
            }
        }

        if self.params.mode == Mode::StereoToMono {
            self.temp_plots = Some((Vec::new(), Vec::new()));
        } else {
            self.temp_plots = None;
        }
    }

    /// Sets the detection mode
    pub fn set_mode(&mut self, mode: Mode) {
        if self.params.mode != mode {
            self.params.mode = mode;
            self.allocate_buffers();
        }
    }
}

impl Detector for DefaultDetector {
    fn process(
        &mut self,
        left_stream: &audio_to_gui_stream::Consumer,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        left_plot: Option<&mut [f32]>,
        right_plot: Option<&mut [f32]>,
        _delta_gui_time: f32,
    ) {
        match self.params.mode {
            Mode::MonoOrLeftOnly => {
                left_stream.read_access(|s1: &[f32], s2: &[f32]| {
                    if let Some(channel) = &mut self.left_channel {
                        channel.process(
                            s1,
                            s2,
                            left_plot,
                            right_plot,
                            &self.params,
                        );
                    }
                });
            }
            Mode::RightOnly => {
                if let Some(right_stream) = right_stream {
                    right_stream.read_access(|s1: &[f32], s2: &[f32]| {
                        if let Some(channel) = &mut self.right_channel {
                            channel.process(
                                s1,
                                s2,
                                left_plot,
                                right_plot,
                                &self.params,
                            );
                        }
                    });
                }
            }
            Mode::Dual => {
                left_stream.read_access(|s1: &[f32], s2: &[f32]| {
                    if let Some(channel) = &mut self.left_channel {
                        channel.process(s1, s2, left_plot, None, &self.params);
                    }
                });

                if let Some(right_stream) = right_stream {
                    right_stream.read_access(|s1: &[f32], s2: &[f32]| {
                        if let Some(channel) = &mut self.right_channel {
                            channel.process(
                                s1,
                                s2,
                                right_plot,
                                None,
                                &self.params,
                            );
                        }
                    });
                }
            }
            Mode::StereoToMono => {
                let mut left_processed = false;
                let mut right_processed = false;

                let left_plot_len = if let Some(left_plot) = &left_plot {
                    Some(left_plot.len())
                } else {
                    None
                };

                left_stream.read_access(|s1: &[f32], s2: &[f32]| {
                    if let Some(channel) = &mut self.left_channel {
                        if let Some(left_plot_len) = left_plot_len {
                            let temp_plot = if let Some((temp_left_plot, _)) =
                                &mut self.temp_plots
                            {
                                temp_left_plot.resize(left_plot_len, 0.0);

                                Some(&mut temp_left_plot[..])
                            } else {
                                None
                            };

                            channel.process(
                                s1,
                                s2,
                                temp_plot,
                                None,
                                &self.params,
                            );
                        } else {
                            channel.process(s1, s2, None, None, &self.params);
                        }

                        left_processed = true;
                    }
                });

                if let Some(right_stream) = right_stream {
                    right_stream.read_access(|s1: &[f32], s2: &[f32]| {
                        if let Some(channel) = &mut self.right_channel {
                            if let Some(left_plot_len) = left_plot_len {
                                let temp_plot =
                                    if let Some((_, temp_right_plot)) =
                                        &mut self.temp_plots
                                    {
                                        temp_right_plot
                                            .resize(left_plot_len, 0.0);

                                        Some(&mut temp_right_plot[..])
                                    } else {
                                        None
                                    };

                                channel.process(
                                    s1,
                                    s2,
                                    temp_plot,
                                    None,
                                    &self.params,
                                );
                            } else {
                                channel.process(
                                    s1,
                                    s2,
                                    None,
                                    None,
                                    &self.params,
                                );
                            }

                            right_processed = true;
                        }
                    });
                }

                if left_processed && right_processed {
                    if let Some(left_plot) = left_plot {
                        if let Some((temp_left_plot, temp_right_plot)) =
                            &mut self.temp_plots
                        {
                            let len = left_plot.len();

                            let left_plot = &mut left_plot[..len];
                            let temp_left_plot = &mut temp_left_plot[..len];
                            let temp_right_plot = &mut temp_right_plot[..len];

                            // Sum left and right plots together.
                            for i in 0..len {
                                left_plot[i] = (temp_left_plot[i]
                                    + temp_right_plot[i])
                                    / 2.0;
                            }
                        }

                        if let Some(right_plot) = right_plot {
                            right_plot.copy_from_slice(left_plot);
                        }
                    }
                }
            }
        }
    }

    fn clear(&mut self) {
        if let Some(channel) = &mut self.left_channel {
            channel.clear();
        }
        if let Some(channel) = &mut self.right_channel {
            channel.clear();
        }
    }

    fn set_window_size(&mut self, window_size: f32) {
        if self.params.window_size != window_size {
            self.params.update_window_size(window_size);
            self.allocate_buffers();
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        if self.params.sample_rate != sample_rate {
            self.params.update_sample_rate(sample_rate);
            self.allocate_buffers();
        }
    }

    fn set_gain(&mut self, gain: f32) {
        self.params.gain = gain as f32;
    }

    fn set_phase(&mut self, phase: Normal) {
        self.params.phase = phase;
    }
}
