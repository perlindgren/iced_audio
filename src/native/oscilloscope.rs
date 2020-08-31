//! Display an oscilloscope.

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{audio_to_gui_stream, Normal};

pub mod default_detector;

/// A visualizer that displays average/peak decibel levels. It can be
/// either mono or stereo.
///
/// A [`Oscilloscope`] will try to fill the size of its container.
///
/// [`Oscilloscope`]: struct.Oscilloscope.html
#[allow(missing_debug_implementations)]
pub struct Oscilloscope<'a, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    style: Renderer::Style,
}

impl<'a, Renderer: self::Renderer> Oscilloscope<'a, Renderer> {
    /// Creates a new [`Oscilloscope`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`Oscilloscope`]
    ///
    /// [`State`]: struct.State.html
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn new(state: &'a mut State) -> Self {
        Oscilloscope {
            state,
            width: Length::Fill,
            height: Length::Fill,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`Oscilloscope`].
    ///
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Oscilloscope`].
    ///
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`Oscilloscope`].
    ///
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

/// The local state of an [`Oscilloscope`].
///
/// [`Oscilloscope`]: struct.Oscilloscope.html
#[derive(Debug)]
pub struct State {
    left_plot: Vec<f32>,
    right_plot: Option<Vec<f32>>,
    left_active: bool,
    right_active: bool,
    is_dual: bool,
}

impl State {
    /// Creates a new [`Oscilloscope`] state.
    ///
    /// * `resolution` - The resolution of the plot (the number of plot points)
    /// * `dual_plots` - Wether to have two plots for the left and right channel (true),
    /// or a single plot for both (false).
    ///
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn new(resolution: usize, dual_plots: bool) -> Self {
        let right_plot: Option<Vec<f32>> = if dual_plots {
            Some(vec![0.0; resolution])
        } else {
            None
        };

        Self {
            left_plot: vec![0.0; resolution],
            right_plot,
            left_active: false,
            right_active: false,
            is_dual: dual_plots,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Oscilloscope<'a, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> Renderer::Output {
        let left_plot = if self.state.left_active {
            Some(&self.state.left_plot[..])
        } else {
            None
        };

        let right_plot = if let Some(right_plot) = &self.state.right_plot {
            if self.state.right_active {
                Some(&right_plot[..])
            } else {
                None
            }
        } else {
            None
        };

        renderer.draw(
            layout.bounds(),
            &self.style,
            left_plot,
            right_plot,
            self.state.is_dual,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

/// The renderer of an [`Oscilloscope`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`Oscilloscope`] in your user interface.
///
/// [`Oscilloscope`]: struct.Oscilloscope.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`Oscilloscope`].
    ///
    /// It receives:
    ///   * the bounds of the [`Oscilloscope`]
    ///   * the style of the [`Oscilloscope`]
    ///
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        style: &Self::Style,
        left_plot: Option<&[f32]>,
        right_plot: Option<&[f32]>,
        is_dual: bool,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Oscilloscope<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        oscilloscope: Oscilloscope<'a, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(oscilloscope)
    }
}

/// A DSP processor used to generate an oscilloscope plot of a stereo signal
pub trait Detector {
    /// Process new samples and store the resulting oscilloscope plot. If `None` is given for the plot,
    /// then do any processing without plotting the result.
    fn process(
        &mut self,
        left_stream: &audio_to_gui_stream::Consumer,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        left_plot: Option<&mut [f32]>,
        right_plot: Option<&mut [f32]>,
        _delta_gui_time: f32,
    );

    /// Clear any buffers / set to 0
    fn clear(&mut self);

    /// Called when the window size changes
    ///
    /// * `window_size` - The window size in seconds
    fn set_window_size(&mut self, window_size: f32);

    /// Called when the sample rate changes
    ///
    /// * `sample_rate` - The sample rate in samples per second
    fn set_sample_rate(&mut self, sample_rate: f32);

    /// Called when the gain changes
    ///
    /// * `gain` - The input gain in amplitude (not dB)
    fn set_gain(&mut self, gain: f32);

    /// Called when the phase changes
    ///
    /// * `phase` - The phase of the starting point in the window
    fn set_phase(&mut self, phase: Normal);
}

/// Processes audio to animate an [`Oscilloscope`]
///
/// [`Oscilloscope`]: struct.Oscilloscope.html
#[allow(missing_debug_implementations)]
pub struct Animator {
    /// The current detector
    pub detector: Box<dyn Detector>,
}

impl Animator {
    /// Creates a new `Animator` for an [`Oscilloscope`]
    ///
    /// ## It expects:
    ///
    /// * `detector` - A [`Detector`] that generates the oscilloscope plot of a signal
    ///
    /// [`State`]: struct.State.html
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    /// [`Detector`]: trait.Detector.html
    pub fn new(detector: Box<dyn Detector>) -> Self {
        Self { detector }
    }

    /// Updates to the next frame.
    ///
    /// * `delta_time` - the elapsed time since the last frame (since update() was last called)
    /// * `oscilloscope` - the [`State`] of the [`Oscilloscope`] to be animated
    /// * `left_stream` - The left/mono audio stream. Set this to `None` if there is no audio stream.
    /// * `right_stream` - The right audio stream. Set this to `None` for a mono audio stream.
    /// * `skip_plotting` - Whether to skip plotting for this frame (true) or not (false).
    ///
    /// [`State`]: struct.State.html
    /// [`Oscilloscope`]: struct.Oscilloscope.html
    pub fn update(
        &mut self,
        delta_gui_time: f32,
        oscilloscope: &mut State,
        left_stream: Option<&audio_to_gui_stream::Consumer>,
        right_stream: Option<&audio_to_gui_stream::Consumer>,
        skip_plotting: bool,
    ) {
        if let Some(left_stream) = left_stream {
            oscilloscope.left_active = true;

            let (left_plot, right_plot) = if skip_plotting {
                (None, None)
            } else {
                (
                    Some(&mut oscilloscope.left_plot[..]),
                    if let Some(right_plot) = &mut oscilloscope.right_plot {
                        Some(&mut right_plot[..])
                    } else {
                        None
                    },
                )
            };

            self.detector.process(
                left_stream,
                right_stream,
                left_plot,
                right_plot,
                delta_gui_time,
            );
        } else {
            oscilloscope.left_active = false;
        }

        oscilloscope.right_active = if let Some(_) = right_stream {
            true
        } else {
            false
        };
    }

    /// Clear any buffers / set to 0
    pub fn clear(&mut self) {
        self.detector.clear();
    }

    /// Updates the window size
    ///
    /// * `window_size` - The window size in seconds
    pub fn set_window_size(&mut self, window_size: f32) {
        self.detector.set_window_size(window_size);
    }

    /// Updates the sample rate
    ///
    /// * `sample_rate` - The sample rate in samples per second
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.detector.set_sample_rate(sample_rate);
    }

    /// Updates the input gain
    ///
    /// * `gain` - The input gain in amplitude (not dB)
    pub fn set_gain(&mut self, gain: f32) {
        self.detector.set_gain(gain);
    }

    /// Updates the phase
    ///
    /// * `phase` - The phase of the starting point in the window
    pub fn set_phase(&mut self, phase: Normal) {
        self.detector.set_phase(phase);
    }
}
