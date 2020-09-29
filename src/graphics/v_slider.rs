//! Display an interactive vertical slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::{ModulationRange, Normal};
use crate::graphics::{
    text_marks, text_marks_render, tick_marks, tick_marks_render,
};
use crate::native::v_slider;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Point, Rectangle};

pub use crate::native::v_slider::State;
pub use crate::style::v_slider::{
    HandleLayer, ModRangePlacement, ModRangeStyle, Rail, Style, StyleSheet,
    ValueFill, ClassicRail, RectangleRail, TextureRail, ValueFillMode,
    RectangleLayer, CircleLayer, TextureLayer,
};

/// A vertical slider GUI widget that controls a [`Param`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`VSlider`]: struct.VSlider.html
pub type VSlider<'a, Message, ID, Backend> =
    v_slider::VSlider<'a, Message, Renderer<Backend>, ID>;

impl<B: Backend> v_slider::Renderer for Renderer<B> {
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
        let handle_height = f32::from(style.handle_height);

        let mark_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y + (handle_height / 2.0).round(),
            width: bounds.width,
            height: bounds.height - handle_height,
        };

        let tick_marks_primitive = if let Some(tick_marks) = tick_marks {
            if let Some((tick_mark_style, placement)) = &tick_mark_style {
                tick_marks_render::draw_vertical_tick_marks(
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
                text_marks_render::draw_vertical_text_marks(
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
            x: bounds.x,
            y: (bounds.y + (mark_bounds.height * normal.inv())).round(),
            width: bounds.width,
            height: handle_height,
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

        let handle_bottom = draw_handle_layer(&style.handle_bottom, &handle_bounds);
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
        Rail::None => {
            Primitive::None
        }
        Rail::Classic(classic_rail) => {
            let (left_color, right_color) = classic_rail.colors;
            let left_width = f32::from(classic_rail.widths.0);
            let right_width = f32::from(classic_rail.widths.1);
            let edge_padding = f32::from(classic_rail.edge_padding);

            let y = bounds.y + edge_padding;
            let height = bounds.height - (edge_padding * 2.0);

            let center_x = bounds.x + (bounds.width / 2.0);
            let start_x =
                (center_x - ((left_width + right_width) / 2.0)).round();

            let left_rail = Primitive::Quad {
                bounds: Rectangle {
                    x: start_x,
                    y,
                    width: left_width,
                    height,
                },
                background: Background::Color(left_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };
            let right_rail = Primitive::Quad {
                bounds: Rectangle {
                    x: start_x + left_width,
                    y,
                    width: right_width,
                    height,
                },
                background: Background::Color(right_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };

            Primitive::Group {
                primitives: vec![left_rail, right_rail],
            }
        }
        Rail::Rectangle(rectangle_rail) => {
            let width = if let Some(width) = rectangle_rail.width {
                f32::from(width)
            } else {
                bounds.width
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: (bounds.x + ((bounds.width - width) / 2.0)).round(),
                    y: bounds.y + f32::from(rectangle_rail.edge_padding),
                    width,
                    height: bounds.height - (f32::from(rectangle_rail.edge_padding) * 2.0),
                },
                background: Background::Color(rectangle_rail.color),
                border_radius: rectangle_rail.border_radius,
                border_width: rectangle_rail.border_width,
                border_color: rectangle_rail.border_color,
            }
        },
        Rail::Texture(texture_rail) => {
            let width = if let Some(width) = texture_rail.width {
                f32::from(width)
            } else {
                bounds.width
            };

            let height = if let Some(height) = texture_rail.height {
                f32::from(height) - (f32::from(texture_rail.edge_padding) * 2.0)
            } else {
                bounds.height
            };

            Primitive::Image {
                handle: texture_rail.image_handle.clone(),
                /// The bounds of the image
                bounds: Rectangle {
                    x: (bounds.x + texture_rail.offset.x + ((bounds.width - width) / 2.0))
                        .round(),
                    y: (bounds.y
                        + texture_rail.offset.y
                        + f32::from(texture_rail.edge_padding)
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
    let (y, height) = match value_fill.fill_mode {
        ValueFillMode::FromBottom { padding } => {
            if value_normal.value() == 0.0 {
                return Primitive::None;
            }

            let y = (handle_bounds.center_y()
                + f32::from(value_fill.handle_spacing)
                - f32::from(value_fill.border_width))
            .round();
            (
                y,
                bounds.y + bounds.height
                    - f32::from(padding)
                    - y,
            )
        }
        ValueFillMode::FromTop { padding } => {
            if value_normal.value() == 1.0 {
                return Primitive::None;
            }

            let y = bounds.y + f32::from(padding);
            (
                y,
                (handle_bounds.center_y()
                    - f32::from(value_fill.handle_spacing)
                    + f32::from(value_fill.border_width)
                    - y)
                    .floor(),
            )
        }
        ValueFillMode::FromCenter => {
            if value_normal.value() == 0.5 {
                return Primitive::None;
            }

            let center_y = bounds.center_y().round();
            if value_normal.value() > 0.5 {
                let y = (handle_bounds.center_y()
                    + f32::from(value_fill.handle_spacing)
                    - f32::from(value_fill.border_width))
                .round();
                (y, center_y - y)
            } else {
                (
                    center_y,
                    (handle_bounds.center_y()
                        - f32::from(value_fill.handle_spacing)
                        + f32::from(value_fill.border_width)
                        - center_y)
                        .floor(),
                )
            }
        }
    };

    let (x, width) = if let Some(width) = value_fill.width {
        let width = f32::from(width);
        (
            (bounds.x
                + ((bounds.width - width) / 2.0)
                + f32::from(value_fill.h_offset))
            .round(),
            width,
        )
    } else {
        (bounds.x + f32::from(value_fill.h_offset), bounds.width)
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
    let width = if let Some(width) = mod_range.width {
        f32::from(width)
    } else {
        bounds.width
    };

    let x = match mod_range.placement {
        ModRangePlacement::Center => {
            (bounds.x + ((bounds.width - width) / 2.0) + mod_range.offset.x)
                .round()
        }
        ModRangePlacement::Left => {
            (bounds.x - width + mod_range.offset.x).round()
        }
        ModRangePlacement::Right => {
            (bounds.x + bounds.width + mod_range.offset.x).round()
        }
    };

    let back_y = (bounds.y + mod_range.offset.y).round();
    let back_height = bounds.height - (f32::from(mod_range.edge_padding) * 2.0);

    let back = if let Some(back_color) = mod_range.back_color {
        Primitive::Quad {
            bounds: Rectangle {
                x,
                y: back_y,
                width,
                height: back_height,
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
        let start_offset_y = (start_normal.inv() * back_height).round();
        let end_offset_y = (end_normal.inv() * back_height).round();

        if start_offset_y == end_offset_y {
            Primitive::None
        } else {
            let (y, height, color) = if end_offset_y > start_offset_y {
                (
                    back_y + start_offset_y,
                    end_offset_y - start_offset_y,
                    mod_range.filled_color_inv,
                )
            } else {
                (
                    back_y + end_offset_y,
                    start_offset_y - end_offset_y,
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
        HandleLayer::None => {
            Primitive::None
        }
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
                handle_bounds.height
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
