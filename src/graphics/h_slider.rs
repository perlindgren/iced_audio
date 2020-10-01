//! Display an interactive vertical slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::{ModulationRange, Normal};
use crate::graphics::{
    text_marks, text_marks_render, tick_marks, tick_marks_render,
};
use crate::native::h_slider;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Point, Rectangle};

pub use crate::native::h_slider::State;
pub use crate::style::h_slider::{
    CircleLayer, ClassicRail, HandleLayer, ModRangePlacement, ModRangeStyle,
    Rail, RectangleLayer, RectangleRail, Style, StyleSheet, TextureLayer,
    TextureRail, ValueFill, ValueFillMode,
};

/// A vertical slider GUI widget that controls a [`Param`]
///
/// An [`HSlider`] will try to fill the vertical space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`HSlider`]: struct.HSlider.html
pub type HSlider<'a, Message, ID, Backend> =
    h_slider::HSlider<'a, Message, Renderer<Backend>, ID>;

impl<B: Backend> h_slider::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        mod_range_1: Option<ModulationRange>,
        mod_range_2: Option<ModulationRange>,
        tick_marks: Option<&tick_marks::Group>,
        text_marks: Option<&text_marks::Group>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging(normal)
        } else if is_mouse_over {
            style_sheet.hovered(normal)
        } else {
            style_sheet.active(normal)
        };

        let tick_mark_style = style_sheet.tick_marks_style();
        let text_mark_style = style_sheet.text_marks_style();

        let bounds = Rectangle {
            x: bounds.x.round(),
            y: bounds.y.round(),
            width: bounds.width.round(),
            height: bounds.height.round(),
        };
        let handle_width = f32::from(style.handle_width);

        let mark_bounds = Rectangle {
            x: bounds.x + (handle_width / 2.0).round(),
            y: bounds.y,
            width: bounds.width - handle_width,
            height: bounds.height,
        };

        let tick_marks_primitive = if let Some(tick_marks) = tick_marks {
            if let Some((tick_mark_style, placement)) = &tick_mark_style {
                tick_marks_render::draw_horizontal_tick_marks(
                    &mark_bounds,
                    tick_marks,
                    tick_mark_style,
                    *placement,
                    false,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        };

        let text_marks_primitive = if let Some(text_marks) = text_marks {
            if let Some(text_mark_style) = &text_mark_style {
                text_marks_render::draw_horizontal_text_marks(
                    &mark_bounds,
                    text_marks,
                    text_mark_style,
                    false,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        };

        let rail = draw_rail(&style.rail, &bounds);

        let handle_bounds = Rectangle {
            x: (bounds.x + (mark_bounds.width * normal.value())).round(),
            y: bounds.y,
            width: handle_width,
            height: bounds.height,
        };

        let value_fill = if let Some(value_fill_style) = &style.value_fill {
            draw_value_fill(value_fill_style, &bounds, &handle_bounds, normal)
        } else {
            Primitive::None
        };

        let mod_range_1_primitive = if let Some(mod_range) = mod_range_1 {
            if let Some(mod_range_style) = &style_sheet.mod_range_style() {
                draw_mod_range(
                    mod_range_style,
                    &bounds,
                    mod_range.start,
                    mod_range.end,
                    true,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        };

        let mod_range_2_primitive = if let Some(mod_range) = mod_range_2 {
            if let Some(mod_range_style) = &style_sheet.mod_range_style_2() {
                draw_mod_range(
                    mod_range_style,
                    &bounds,
                    mod_range.start,
                    mod_range.end,
                    true,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        };

        let handle_bottom =
            draw_handle_layer(&style.handle_bottom, &handle_bounds);
        let handle_top = draw_handle_layer(&style.handle_top, &handle_bounds);

        (
            Primitive::Group {
                primitives: vec![
                    rail,
                    tick_marks_primitive,
                    text_marks_primitive,
                    value_fill,
                    mod_range_1_primitive,
                    mod_range_2_primitive,
                    handle_bottom,
                    handle_top,
                ],
            },
            mouse::Interaction::default(),
        )
    }
}

fn draw_rail(rail: &Rail, bounds: &Rectangle) -> Primitive {
    match rail {
        Rail::None => Primitive::None,
        Rail::Classic(classic_rail) => {
            let (top_color, bottom_color) = classic_rail.colors;
            let top_width = f32::from(classic_rail.widths.0);
            let bottom_width = f32::from(classic_rail.widths.1);
            let edge_padding = f32::from(classic_rail.edge_padding);

            let x = bounds.x + edge_padding;
            let width = bounds.width - (edge_padding * 2.0);

            let center_y = bounds.y + (bounds.height / 2.0);
            let start_y =
                (center_y - ((top_width + bottom_width) / 2.0)).round();

            let top_rail = Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: start_y,
                    width,
                    height: top_width,
                },
                background: Background::Color(top_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };
            let bottom_rail = Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: start_y + top_width,
                    width,
                    height: bottom_width,
                },
                background: Background::Color(bottom_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };

            Primitive::Group {
                primitives: vec![top_rail, bottom_rail],
            }
        }
        Rail::Rectangle(rectangle_rail) => {
            let height = if let Some(height) = rectangle_rail.height {
                f32::from(height)
            } else {
                bounds.height
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x + f32::from(rectangle_rail.edge_padding),
                    y: (bounds.y + ((bounds.height - height) / 2.0)).round(),
                    width: bounds.width
                        - (f32::from(rectangle_rail.edge_padding) * 2.0),
                    height,
                },
                background: Background::Color(rectangle_rail.color),
                border_radius: rectangle_rail.border_radius,
                border_width: rectangle_rail.border_width,
                border_color: rectangle_rail.border_color,
            }
        }
        Rail::Texture(texture_rail) => {
            let width = if let Some(width) = texture_rail.width {
                f32::from(width) - (f32::from(texture_rail.edge_padding) * 2.0)
            } else {
                bounds.width
            };

            let height = if let Some(height) = texture_rail.height {
                f32::from(height)
            } else {
                bounds.height
            };

            Primitive::Image {
                handle: texture_rail.image_handle.clone(),
                /// The bounds of the image
                bounds: Rectangle {
                    x: (bounds.x
                        + texture_rail.offset.x
                        + f32::from(texture_rail.edge_padding)
                        + ((bounds.width - width) / 2.0))
                        .round(),
                    y: (bounds.y
                        + texture_rail.offset.y
                        + ((bounds.height - height) / 2.0))
                        .round(),
                    width,
                    height,
                },
            }
        }
    }
}

fn draw_value_fill(
    value_fill: &ValueFill,
    bounds: &Rectangle,
    handle_bounds: &Rectangle,
    value_normal: Normal,
) -> Primitive {
    let (x, width) = match value_fill.fill_mode {
        ValueFillMode::FromLeft { padding } => {
            if value_normal.value() == 0.0 {
                return Primitive::None;
            }

            let x = bounds.x + f32::from(padding);
            (
                x,
                (handle_bounds.center_x()
                    - f32::from(value_fill.handle_spacing)
                    + f32::from(value_fill.border_width)
                    - x)
                    .floor(),
            )
        }
        ValueFillMode::FromRight { padding } => {
            if value_normal.value() == 1.0 {
                return Primitive::None;
            }

            let x = (handle_bounds.center_x()
                + f32::from(value_fill.handle_spacing)
                - f32::from(value_fill.border_width))
            .round();
            (x, bounds.x + bounds.width - f32::from(padding) - x)
        }
        ValueFillMode::FromCenter => {
            if value_normal.value() == 0.5 {
                return Primitive::None;
            }

            let center_x = bounds.center_x().round();
            if value_normal.value() > 0.5 {
                (
                    center_x,
                    (handle_bounds.center_x()
                        - f32::from(value_fill.handle_spacing)
                        + f32::from(value_fill.border_width)
                        - center_x)
                        .floor(),
                )
            } else {
                let x = (handle_bounds.center_x()
                    + f32::from(value_fill.handle_spacing)
                    - f32::from(value_fill.border_width))
                .round();
                (x, center_x - x)
            }
        }
    };

    let (y, height) = if let Some(height) = value_fill.height {
        let height = f32::from(height);
        (
            (bounds.y
                + ((bounds.height - height) / 2.0)
                + f32::from(value_fill.v_offset))
            .round(),
            height,
        )
    } else {
        (bounds.y + f32::from(value_fill.v_offset), bounds.height)
    };

    Primitive::Quad {
        bounds: Rectangle {
            x,
            y,
            width,
            height,
        },
        background: Background::Color(value_fill.color),
        border_radius: value_fill.border_radius,
        border_width: value_fill.border_width,
        border_color: value_fill.border_color,
    }
}

fn draw_mod_range(
    mod_range: &ModRangeStyle,
    bounds: &Rectangle,
    start_normal: Normal,
    end_normal: Normal,
    active: bool,
) -> Primitive {
    let height = if let Some(height) = mod_range.height {
        f32::from(height)
    } else {
        bounds.height
    };

    let y = match mod_range.placement {
        ModRangePlacement::Center => {
            (bounds.y + ((bounds.height - height) / 2.0) + mod_range.offset.y)
                .round()
        }
        ModRangePlacement::Top => {
            (bounds.y - height + mod_range.offset.y).round()
        }
        ModRangePlacement::Bottom => {
            (bounds.y + bounds.height + mod_range.offset.y).round()
        }
    };

    let back_x = (bounds.x + mod_range.offset.x).round();
    let back_width = bounds.width - (f32::from(mod_range.edge_padding) * 2.0);

    let back = if let Some(back_color) = mod_range.back_color {
        Primitive::Quad {
            bounds: Rectangle {
                x: back_x,
                y,
                width: back_width,
                height,
            },
            background: Background::Color(back_color),
            border_radius: mod_range.border_radius,
            border_width: mod_range.border_width,
            border_color: mod_range.border_color,
        }
    } else {
        Primitive::None
    };

    let range = if active {
        let start_offset_x = (start_normal.value() * back_width).round();
        let end_offset_x = (end_normal.value() * back_width).round();

        if start_offset_x == end_offset_x {
            Primitive::None
        } else {
            let (x, width, color) = if end_offset_x > start_offset_x {
                (
                    back_x + start_offset_x,
                    end_offset_x - start_offset_x,
                    mod_range.filled_color_inv,
                )
            } else {
                (
                    back_x + end_offset_x,
                    start_offset_x - end_offset_x,
                    mod_range.filled_color,
                )
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y,
                    width,
                    height,
                },
                background: Background::Color(color),
                border_radius: mod_range.border_radius,
                border_width: mod_range.border_width,
                border_color: Color::TRANSPARENT,
            }
        }
    } else {
        Primitive::None
    };

    Primitive::Group {
        primitives: vec![back, range],
    }
}

fn draw_handle_layer(
    handle_layer: &HandleLayer,
    handle_bounds: &Rectangle,
) -> Primitive {
    match handle_layer {
        HandleLayer::None => Primitive::None,
        HandleLayer::Rectangle(rectangle_layer) => {
            let width = if let Some(width) = rectangle_layer.width {
                f32::from(width)
            } else {
                handle_bounds.width
            };

            let height = if let Some(height) = rectangle_layer.height {
                f32::from(height)
            } else {
                handle_bounds.height
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + rectangle_layer.offset.x
                        + ((handle_bounds.width - width) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + rectangle_layer.offset.y
                        + ((handle_bounds.height - height) / 2.0))
                        .round(),
                    width,
                    height,
                },
                background: Background::Color(rectangle_layer.color),
                border_radius: rectangle_layer.border_radius,
                border_width: rectangle_layer.border_width,
                border_color: rectangle_layer.border_color,
            }
        }
        HandleLayer::Circle(circle_layer) => {
            let diameter = if let Some(diameter) = circle_layer.diameter {
                f32::from(diameter)
            } else {
                handle_bounds.width
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + circle_layer.offset.x
                        + ((handle_bounds.width - diameter) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + circle_layer.offset.y
                        + ((handle_bounds.height - diameter) / 2.0))
                        .round(),
                    width: diameter,
                    height: diameter,
                },
                background: Background::Color(circle_layer.color),
                border_radius: (diameter / 2.0) as u16,
                border_width: circle_layer.border_width,
                border_color: circle_layer.border_color,
            }
        }
        HandleLayer::Texture(texture_layer) => {
            let width = if let Some(width) = texture_layer.width {
                f32::from(width)
            } else {
                handle_bounds.width
            };

            let height = if let Some(height) = texture_layer.height {
                f32::from(height)
            } else {
                handle_bounds.height
            };

            Primitive::Image {
                handle: texture_layer.image_handle.clone(),
                /// The bounds of the image
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + texture_layer.offset.x
                        + ((handle_bounds.width - width) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + texture_layer.offset.y
                        + ((handle_bounds.height - height) / 2.0))
                        .round(),
                    width,
                    height,
                },
            }
        }
    }
}
