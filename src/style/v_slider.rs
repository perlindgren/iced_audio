//! Various styles for the [`VSlider`] widget
//!
//! [`VSlider`]: ../../native/v_slider/struct.VSlider.html

use iced::Color;
use iced_native::{image, Point};

use crate::style::{default_colors, text_marks, tick_marks};

/// The appearance of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct Style {
    /// A rail that the handle slides in.
    pub rail: Option<Rail>,
    /// A rectangle filled to the edge of the handle.
    pub value_fill: Option<ValueFill>,
    /// The bottom layer of the handle.
    pub handle_bottom: Option<HandleLayer>,
    /// The top layer of the handle.
    pub handle_top: Option<HandleLayer>,
    /// The height of the handle in pixels.
    pub handle_height: u16,
}

/// The appearance of the rail of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum Rail {
    /// Classic rail
    Classic {
        /// Colors of the left and right of the rail.
        colors: (Color, Color),
        /// Width (thickness) of the left and right of the rail in pixels.
        widths: (u16, u16),
        /// The spacing from the ends of the rail to the top and bottom of
        /// the widget in pixels.
        edge_padding: u16,
    },
    /// A background rectangle
    Rectangle {
        /// * Color of the rectangle.
        color: Color,
        /// * Color of the border.
        border_color: Color,
        /// * Width of the border.
        border_width: u16,
        /// * Radius of the corners.
        border_radius: u16,
    },
    /// Textured rail
    Texture {
        /// The image handle.
        image_handle: image::Handle,
        /// Width of the texture in pixels. Set to `None` to use the
        /// width of the widget.
        width: Option<u16>,
        /// Height of the texture in pixels. Set to `None` to use the
        /// height of the widget.
        height: Option<u16>,
        /// The spacing from the ends of the rail to the top and bottom of
        /// the widget in pixels. This is only effective when `height` is `None`.
        edge_padding: u16,
        /// Offset of the texture in pixels.
        offset: Point,
    },
}

/// The appearance of a value fill rectangle in a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct ValueFill {
    /// Color of the value fill rectangle.
    pub color: Color,
    /// Width of the border.
    pub border_width: u16,
    /// Radius of the corners
    pub border_radius: u16,
    /// Color of the border.
    pub border_color: Color,
    /// The spacing in pixels between the center of the handle
    /// and the value fill rectangle.
    pub handle_spacing: u16,
    /// The width (thickness) of the value fill rectangle. Set to
    /// `None` to use the full width of the widget.
    pub width: Option<u16>,
    /// Whether the start from the middle (true), or one of the edges (false).
    pub bipolar: bool,
    /// Whether the value should start from the bottom (true), or
    /// from the top (false). Only effective when `bipolar` is false.
    pub from_bottom: bool,
    /// The spacing from the end of the value fill to the top or bottom of
    /// the widget in pixels. This is only effective when `bipolar` is false.
    pub edge_padding: u16,
    /// The horizontal offset of the value fill rectangle in pixels.
    pub h_offset: u16,
}

/// The appearance of a handle layer in a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum HandleLayer {
    /// A rectangle
    Rectangle {
        /// Color of the rectangle.
        color: Color,
        /// Color of the border.
        border_color: Color,
        /// Width of the border.
        border_width: u16,
        /// Radius of the corners.
        border_radius: u16,
        /// Width of the rectangle in pixels. Set to `None` to use the
        /// width of the widget.
        width: Option<u16>,
        /// Height of the rectangle in pixels. Set to `None` to use the
        /// height of the handle.
        height: Option<u16>,
        /// Offset from the center of the handle in pixels.
        offset: Point,
    },
    /// A circle
    Circle {
        /// Color of the circle.
        color: Color,
        /// Color of the border.
        border_color: Color,
        /// Width of the border.
        border_width: u16,
        /// Diameter of the circle in pixels. Set to `None` to use the
        /// height of the handle.
        diameter: Option<u16>,
        /// Offset from the center of the handle in pixels.
        offset: Point,
    },
    /// A texture
    Texture {
        /// The handle to the texture.
        image_handle: image::Handle,
        /// Width of the texture in pixels. Set to `None` to use the
        /// width of the widget.
        width: Option<u16>,
        /// Height of the texture in pixels. Set to `None` to use the
        /// height of the handle.
        height: Option<u16>,
        /// Offset from the center of the handle in pixels.
        offset: Point,
    },
    // TODO: Triangle and hexagon.
}

/// The placement of a modulation range indicator in a [`VSlider`]
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum ModRangePlacement {
    /// In the center of the widget.
    Center,
    /// To the left of the widget.
    Left,
    /// To the right of the widget.
    Right,
}

/// The appearance of a modulation range indicator in a [`VSlider`]
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct ModRangeStyle {
    /// The color of the background rectangle. Set to `None` for no
    /// background rectangle.
    pub back_color: Option<Color>,
    /// The border width of the background rectangle.
    pub border_width: u16,
    /// The border radius of the background rectangle.
    pub border_radius: u16,
    /// The border color of the background rectangle.
    pub border_color: Color,
    /// The color of a filled portion.
    pub filled_color: Color,
    /// The color of a filled portion when the range is inversed.
    pub filled_color_inv: Color,
    /// The width of the rectangle in pixels. Set this to `None` to use
    /// the width of the widget.
    pub width: Option<u16>,
    /// The placement relative to the widget.
    pub placement: ModRangePlacement,
    /// The spacing from the end of the rectangle to the top or bottom of
    /// the widget in pixels.
    pub edge_padding: u16,
    /// The offset in pixels.
    pub offset: Point,
}

/// A set of rules that dictate the style of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
pub trait StyleSheet {
    /// Produces the style of an active [`VSlider`].
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`VSlider`].
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn hovered(&self) -> Style;

    /// Produces the style of an [`VSlider`] that is being dragged.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn dragging(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`VSlider`]
    ///
    /// For no tick marks, set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn tick_mark_style(
        &self,
    ) -> Option<(tick_marks::Style, tick_marks::Placement)> {
        Some((
            tick_marks::Style::default(),
            tick_marks::Placement::default(),
        ))
    }

    /// The style of a [`ModulationRange`] for a [`VSlider`]
    ///
    /// For no modulation range, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn mod_range_style(&self) -> Option<ModRangeStyle> {
        None
    }

    /// The style of a second [`ModulationRange`] for a [`VSlider`]
    ///
    /// For no second modulation range, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn mod_range_style_2(&self) -> Option<ModRangeStyle> {
        None
    }

    /// The style of a [`TextMarkGroup`] for an [`VSlider`]
    ///
    /// For no text marks, set this to return `None`.
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn text_mark_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style::default())
    }
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            rail: Some(Rail::Classic {
                colors: default_colors::SLIDER_RAIL,
                widths: (1, 1),
                edge_padding: 12,
            }),
            value_fill: None,
            handle_height: 30,
            handle_bottom: Some(HandleLayer::Rectangle {
                color: default_colors::LIGHT_BACK,
                border_color: default_colors::BORDER,
                border_width: 1,
                border_radius: 2,
                width: None,
                height: None,
                offset: Point::ORIGIN,
            }),
            // The notch in the middle of the handle
            handle_top: Some(HandleLayer::Rectangle {
                color: default_colors::BORDER,
                border_color: Color::TRANSPARENT,
                border_width: 0,
                border_radius: 0,
                width: None,
                height: Some(4),
                offset: Point::ORIGIN,
            }),
        }
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        Style {
            handle_bottom: Some(HandleLayer::Rectangle {
                color: default_colors::LIGHT_BACK_HOVER,
                border_color: default_colors::BORDER,
                border_width: 1,
                border_radius: 2,
                width: None,
                height: None,
                offset: Point::ORIGIN,
            }),
            ..active
        }
    }

    fn dragging(&self) -> Style {
        let active = self.active();
        Style {
            handle_bottom: Some(HandleLayer::Rectangle {
                color: default_colors::LIGHT_BACK_DRAG,
                border_color: default_colors::BORDER,
                border_width: 1,
                border_radius: 2,
                width: None,
                height: None,
                offset: Point::ORIGIN,
            }),
            ..active
        }
    }

    fn tick_mark_style(
        &self,
    ) -> Option<(tick_marks::Style, tick_marks::Placement)> {
        Some((
            tick_marks::Style {
                tier_1: Some(tick_marks::Shape::Line {
                    length: 8,
                    width: 2,
                    color: default_colors::TICK_TIER_1,
                }),
                tier_2: Some(tick_marks::Shape::Line {
                    length: 6,
                    width: 1,
                    color: default_colors::TICK_TIER_2,
                }),
                tier_3: Some(tick_marks::Shape::Line {
                    length: 4,
                    width: 1,
                    color: default_colors::TICK_TIER_3,
                }),
            },
            tick_marks::Placement::CenterSplit {
                fill_length: false,
                gap: 6,
            },
        ))
    }

    fn text_mark_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style::default())
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
