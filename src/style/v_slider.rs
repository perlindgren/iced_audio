//! Various styles for the [`VSlider`] widget
//!
//! [`VSlider`]: ../../native/v_slider/struct.VSlider.html

use iced::Color;
use iced_native::{image, Point};

use crate::style::{bar_text_marks, default_colors};

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
    /// Textured rail
    Texture {
        /// The image texture.
        texture: image::Handle,
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
}

/// The appearance of a value fill rectangle in a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum ValueFill {
    /// Unipolar value fill
    Unipolar {
        /// Color of the value fill rectangle.
        color: Color,
        /// Radius of the corners
        corner_radius: u16,
        /// The spacing in pixels between the edge of the handle
        /// and the value fill rectangle.
        handle_spacing: u16,
        /// The width (thickness) of the value fill rectangle. Set to
        /// `None` to use the full width of the widget.
        width: Option<u16>,
        /// The horizontal offset of the value fill rectangle in pixels.
        h_offset: u16,
        /// Whether the value should start from the bottom (true), or
        /// from the top (false).
        from_bottom: bool,
    },
    /// Bipolar value fill
    Bipolar {
        /// Color of the value fill rectangle in the bottom position.
        bottom_color: Color,
        /// Color of the value fill rectangle in the top position.
        top_color: Color,
        /// Radius of the corners
        corner_radius: u16,
        /// The spacing in pixels between the edge of the handle
        /// and the value fill rectangle.
        handle_spacing: u16,
        /// The width (thickness) of the value fill rectangle. Set to
        /// `None` to use the full width of the widget.
        width: Option<u16>,
        /// The horizontal offset of the value fill rectangle in pixels.
        h_offset: u16,
    },
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

/// The appearance of a [`TickMarkGroup`] in a [`VSlider`]
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Copy, Clone)]
pub struct TickMarkStyle {
    /// The length of a tier 1 tick mark relative to the length of the `VSlider` in pixels.
    pub length_scale_tier_1: f32,
    /// The length of a tier 2 tick mark relative to the length of the `VSlider` in pixels.
    pub length_scale_tier_2: f32,
    /// The length of a tier 3 tick mark relative to the length of the `VSlider` in pixels.
    pub length_scale_tier_3: f32,

    /// The width (thickness) of a tier 1 tick mark in pixels.
    pub width_tier_1: u16,
    /// The width (thickness) of a tier 2 tick mark in pixels.
    pub width_tier_2: u16,
    /// The width (thickness) of a tier 3 tick mark in pixels.
    pub width_tier_3: u16,

    /// The color of a tier 1 tick mark.
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark.
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark.
    pub color_tier_3: Color,

    /// The vertical distance from the center rail to a tick mark in pixels. Setting this
    /// to `0` will cause each tick mark to be a single continous line going
    /// through the the rail, as apposed to a line above and a line below the
    /// rail.
    pub center_offset: u16,
}

impl std::default::Default for TickMarkStyle {
    fn default() -> Self {
        Self {
            length_scale_tier_1: 1.65,
            length_scale_tier_2: 1.55,
            length_scale_tier_3: 1.4,

            width_tier_1: 2,
            width_tier_2: 1,
            width_tier_3: 1,

            color_tier_1: default_colors::TICK_TIER_1,
            color_tier_2: default_colors::TICK_TIER_2,
            color_tier_3: default_colors::TICK_TIER_3,

            center_offset: 0,
        }
    }
}

/// The position of a modulation range indicator in a [`VSlider`]
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum ModRangePosition {
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
    /// The color of the first modulation range rectangle.
    color_1: Color,
    /// The color of a second modulation range rectangle.
    color_2: Color,
    /// The color of a third modulation range rectangle.
    color_3: Color,
    /// The width of the filled rectangle in pixels.
    width: u16,
    /// The position relative to the widget.
    position: ModRangePosition,
    /// The horizontal offset in pixels.
    h_offset: u16,
    /// The spacing between each modulation range rectangle in pixels.
    spacing: u16,
    /// The width of the real-time modulation indicator rectangle in pixels.
    rt_indicator_width: u16,
    /// The height of the real-time modulation indicator rectangle in pixels.
    rt_indicator_height: u16,
    /// The corner radius of the real-time modulation indicator rectangle.
    rt_indicator_radius: u16,
    /// The color of the first real-time modulation indicator rectangle.
    rt_indicator_color_1: Color,
    /// The color of a second real-time modulation indicator rectangle.
    rt_indicator_color_2: Color,
    /// The color of a third real-time modulation indicator rectangle.
    rt_indicator_color_3: Color,
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
    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        Some(TickMarkStyle::default())
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

    /// The style of a [`TextMarkGroup`] for an [`VSlider`]
    ///
    /// For no text marks, set this to return `None`.
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style::default())
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

    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        Some(TickMarkStyle::default())
    }

    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style::default())
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
