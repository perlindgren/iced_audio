//! Various styles for the [`Oscilloscope`] widget
//!
//! [`Oscilloscope`]: ../../native/oscilloscope/struct.Oscilloscope.html

use iced::Color;

use crate::style::default_colors;

/// The appearance of an [`Oscilloscope`].
///
/// [`Oscilloscope`]: ../../native/oscilloscope/struct.Oscilloscope.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub back_color: Color,
    /// The width of the border of the background rectangle
    pub back_border_width: u16,
    /// The color of the border of the background rectangle
    pub back_border_color: Color,

    /// The color of the left/mono line plot
    pub left_plot_color: Color,
    /// The color of the right line plot
    pub right_plot_color: Color,

    /// The width (thickness) of the left/mono line plot
    pub left_plot_width: f32,
    /// The width (thickness) of the right line plot
    pub right_plot_width: f32,

    /// The color of the center line marking 0 amplitude
    pub center_line_color: Option<Color>,
    /// The width of the center line marking 0 amplitude
    pub center_line_width: u16,

    /// The color of the line seperating the left and right plot.
    /// This will be ignored if the oscillator is in mono mode.
    pub div_line_color: Color,
    /// The width of the line seperating the left and right plot.
    /// This will be ignored if the oscillator is in mono mode.
    pub div_line_width: u16,
}

/// A set of rules that dictate the style of an [`Oscilloscope`].
///
/// [`Oscilloscope`]: ../../native/oscilloscope/struct.Oscilloscope.html
pub trait StyleSheet {
    /// Produces the style of an [`Oscilloscope`].
    ///
    /// [`Oscilloscope`]: ../../native/oscilloscope/struct.Oscilloscope.html
    fn style(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
    fn style(&self) -> Style {
        Style {
            back_color: default_colors::OSCILLOSCOPE_BACK,
            back_border_width: 1,
            back_border_color: default_colors::DB_METER_BORDER,
            left_plot_color: default_colors::DB_METER_LOW,
            right_plot_color: default_colors::DB_METER_LOW,
            left_plot_width: 1.4,
            right_plot_width: 1.4,
            center_line_color: Some(default_colors::OSCILLOSCOPE_CENTER_LINE),
            center_line_width: 1,
            div_line_color: default_colors::DB_METER_BORDER,
            div_line_width: 2,
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
