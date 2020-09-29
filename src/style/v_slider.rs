//! Various styles for the [`VSlider`] widget
//!
//! [`VSlider`]: ../../native/v_slider/struct.VSlider.html

use iced::Color;
use iced_native::{image, Point, Align};

use crate::core::Normal;
use crate::style::{default_colors, text_marks, tick_marks};

/// The appearance of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct Style {
    /// A rail that the handle slides in.
    pub rail: Rail,
    /// A rectangle filled from the starting value to the center
    /// of the handle.
    pub value_fill: Option<ValueFill>,
    /// The height of the handle in pixels.
    pub handle_height: u16,
    /// The bottom layer of the handle.
    pub handle_bottom: HandleLayer,
    /// The top layer of the handle.
    pub handle_top: HandleLayer,
}

/// Classic rail style
#[derive(Debug, Clone)]
pub struct ClassicRail {
    /// Colors of the left and right of the rail.
    pub colors: (Color, Color),
    /// Width (thickness) of the left and right of the rail in pixels.
    pub widths: (u16, u16),
    /// The spacing from the ends of the rail to the top and bottom of
    /// the widget in pixels.
    pub edge_padding: u16,
}


/// Background rectangle rail style
#[derive(Debug, Clone)]
pub struct RectangleRail {
    /// * Color of the rectangle.
    pub color: Color,
    /// * Color of the border.
    pub border_color: Color,
    /// * Width of the border.
    pub border_width: u16,
    /// * Radius of the corners.
    pub border_radius: u16,
    /// Width of the rectangle in pixels. Set to `None` to use the
    /// width of the widget.
    pub width: Option<u16>,
    /// The spacing from the ends of the rail to the top and bottom of
    /// the widget in pixels.
    pub edge_padding: u16,
}

/// Texture rail style
#[derive(Debug, Clone)]
pub struct TextureRail {
    /// The image handle.
    pub image_handle: image::Handle,
    /// Width of the texture in pixels. Set to `None` to use the
    /// width of the widget.
    pub width: Option<u16>,
    /// Height of the texture in pixels. Set to `None` to use the
    /// height of the widget.
    pub height: Option<u16>,
    /// The spacing from the ends of the rail to the top and bottom of
    /// the widget in pixels. This is only effective when `height` is `None`.
    pub edge_padding: u16,
    /// Offset of the texture in pixels.
    pub offset: Point,
}

/// The appearance of the rail of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum Rail {
    /// No Rail
    None,
    /// Classic rail
    Classic(ClassicRail),
    /// A background rectangle
    Rectangle(RectangleRail),
    /// Textured rail
    Texture(TextureRail),
}

/// Where to start the fill from.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueFillMode {
    /// Start from the bottom
    FromBottom {
        /// The spacing from the end of the value fill to the bottom of
        /// the widget in pixels.
        padding: u16,
    },
    /// Start from the top
    FromTop {
        /// The spacing from the end of the value fill to the top of
        /// the widget in pixels.
        padding: u16,
    },
    /// Start from the center
    FromCenter
}

/// A rectangle filled from the starting value to the center
/// of the handle.
#[derive(Debug, Clone)]
pub struct ValueFill {
    /// Color of the value fill rectangle.
    pub color: Color,
    /// Width of the border.
    pub border_width: u16,
    /// Radius of the border.
    pub border_radius: u16,
    /// Color of the border.
    pub border_color: Color,
    /// The spacing in pixels between the center of the handle
    /// and the value fill rectangle.
    pub handle_spacing: u16,
    /// The width (thickness) of the value fill rectangle. Set to
    /// `None` to use the full width of the widget.
    pub width: Option<u16>,
    /// Where to start the fill from.
    pub fill_mode: ValueFillMode,
    /// The horizontal offset of the value fill rectangle in pixels.
    pub h_offset: u16,
}

/// Rectangle handle layer
#[derive(Debug, Clone)]
pub struct RectangleLayer {
    /// Color of the rectangle.
    pub color: Color,
    /// Color of the border.
    pub border_color: Color,
    /// Width of the border.
    pub border_width: u16,
    /// Radius of the corners.
    pub border_radius: u16,
    /// Width of the rectangle in pixels. Set to `None` to use the
    /// width of the widget.
    pub width: Option<u16>,
    /// Height of the rectangle in pixels. Set to `None` to use the
    /// height of the handle.
    pub height: Option<u16>,
    /// Offset from the center of the handle in pixels.
    pub offset: Point,
}

/// Circle handler layer
#[derive(Debug, Clone)]
pub struct CircleLayer {
    /// Color of the circle.
    pub color: Color,
    /// Color of the border.
    pub border_color: Color,
    /// Width of the border.
    pub border_width: u16,
    /// Diameter of the circle in pixels. Set to `None` to use the
    /// height of the handle.
    pub diameter: Option<u16>,
    /// Offset from the center of the handle in pixels.
    pub offset: Point,
}

/// Texture handler layer
#[derive(Debug, Clone)]
pub struct TextureLayer {
    /// The handle to the texture.
    pub image_handle: image::Handle,
    /// Width of the texture in pixels. Set to `None` to use the
    /// width of the widget.
    pub width: Option<u16>,
    /// Height of the texture in pixels. Set to `None` to use the
    /// height of the handle.
    pub height: Option<u16>,
    /// Offset from the center of the handle in pixels.
    pub offset: Point,
}

/// The appearance of a handle layer in a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum HandleLayer {
    /// No layer
    None,
    /// A rectangle
    Rectangle(RectangleLayer),
    /// A circle
    Circle(CircleLayer),
    /// A texture
    Texture(TextureLayer),

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
    /// * `value` - The current normalized value. This can be use to
    /// change the style based on the value of the slider.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn active(&self, value: Normal) -> Style;

    /// Produces the style of a hovered [`VSlider`].
    ///
    /// * `value` - The current normalized value. This can be use to
    /// change the style based on the value of the slider.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn hovered(&self, value: Normal) -> Style;

    /// Produces the style of an [`VSlider`] that is being dragged.
    ///
    /// * `value` - The current normalized value. This can be use to
    /// change the style based on the value of the slider.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn dragging(&self, value: Normal) -> Style;

    /// The style of tick marks for a [`VSlider`]
    ///
    /// For no tick marks, set this to return `None`.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn tick_marks_style(
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

    /// The style of text marks for an [`VSlider`]
    ///
    /// For no text marks, set this to return `None`.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn text_marks_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style::default())
    }
}

struct Default;

impl Default {
    fn handle_bottom() -> RectangleLayer {
        RectangleLayer {
            color: default_colors::LIGHT_BACK,
            border_color: default_colors::BORDER,
            border_width: 1,
            border_radius: 2,
            width: None,
            height: None,
            offset: Point::ORIGIN,
        }
    }
}

impl StyleSheet for Default {
    fn active(&self, _value: Normal) -> Style {
        Style {
            rail: Rail::Classic(ClassicRail {
                colors: default_colors::SLIDER_RAIL,
                widths: (1, 1),
                edge_padding: 12,
            }),
            value_fill: None,
            handle_height: 30,
            handle_bottom: HandleLayer::Rectangle(
                Self::handle_bottom()
            ),
            // The notch in the middle of the handle.
            handle_top: HandleLayer::Rectangle(
                RectangleLayer {
                    color: default_colors::BORDER,
                    border_color: Color::TRANSPARENT,
                    border_width: 0,
                    border_radius: 0,
                    width: None,
                    height: Some(4),
                    offset: Point::ORIGIN,
                }
            ),
        }
    }

    fn hovered(&self, value: Normal) -> Style {
        let active = self.active(value);
        Style {
            handle_bottom: HandleLayer::Rectangle(
                RectangleLayer {
                    color: default_colors::LIGHT_BACK_HOVER,
                    ..Self::handle_bottom()
                }
            ),
            ..active
        }
    }

    fn dragging(&self, value: Normal) -> Style {
        let active = self.active(value);
        Style {
            handle_bottom: HandleLayer::Rectangle(
                RectangleLayer {
                    color: default_colors::LIGHT_BACK_DRAG,
                    ..Self::handle_bottom()
                }
            ),
            ..active
        }
    }

    fn tick_marks_style(
        &self,
    ) -> Option<(tick_marks::Style, tick_marks::Placement)> {
        Some((
            tick_marks::Style {
                tier_1: Some(tick_marks::Shape::Line {
                    length: 24,
                    width: 2,
                    color: default_colors::TICK_TIER_1,
                }),
                tier_2: Some(tick_marks::Shape::Line {
                    length: 22,
                    width: 1,
                    color: default_colors::TICK_TIER_2,
                }),
                tier_3: Some(tick_marks::Shape::Line {
                    length: 18,
                    width: 1,
                    color: default_colors::TICK_TIER_3,
                }),
            },
            tick_marks::Placement::Center {
                fill_length: false,
            },
        ))
    }

    fn text_marks_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style {
            placement: text_marks::Placement::Center {
                align: Align::End
            },
            offset: Point { x: -16.0, y: 0.0 },
            ..text_marks::Style::default()
        })
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
