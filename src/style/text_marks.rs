//! Various styles for a [`TextMarkGroup`] in a bar meter widget
///
/// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
use iced_graphics::{Color, Font};

use crate::core::Offset;
use crate::style::default_colors;

/// The alignment of text in text marks.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    /// Align to the start of the text.
    Start,
    /// Align to the end of the text.
    End,
    /// Align to the center of the text.
    Center,
}

/// The placement of text marks relative to the widget
#[derive(Debug, Clone, PartialEq)]
pub enum Placement {
    /// Text marks on both sides of the widget.
    BothSides {
        /// Whether to align the text marks to the inside of the widget (true),
        /// or the outside of the widget (false).
        inside: bool,
        /// The offset of the text in pixels.
        offset: Offset,
    },
    /// Text marks only on the left/top side of the widget.
    LeftOrTop {
        /// Whether to align the text marks to the inside of the widget (true),
        /// or the outside of the widget (false).
        inside: bool,
        /// The offset of the text in pixels.
        offset: Offset,
    },
    /// Text marks only on the right/bottom side of the widget.
    RightOrBottom {
        /// Whether to align the text marks to the inside of the widget (true),
        /// or the outside of the widget (false).
        inside: bool,
        /// The offset of the text in pixels.
        offset: Offset,
    },
    /// Text marks in the center of the widget.
    Center {
        /// The alignment of the text.
        align: Align,
        /// The offset of the text in pixels.
        offset: Offset,
    },
}

impl std::default::Default for Placement {
    fn default() -> Self {
        Placement::LeftOrTop {
            inside: false,
            offset: Default::default(),
        }
    }
}

/// The style of a [`TextMarkGroup`] for a bar meter widget
///
/// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the text.
    pub color: Color,
    /// The size of the text.
    pub text_size: u16,
    /// The font of the text.
    pub font: Font,
    /// The width of the text bounds.
    pub bounds_width: u16,
    /// The height of the text bounds.
    pub bounds_height: u16,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            color: default_colors::TEXT_MARK,
            text_size: 12,
            font: Default::default(),
            bounds_width: 30,
            bounds_height: 14,
        }
    }
}
