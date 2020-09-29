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
    ValueFill,
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
        text_marks: Option<&text_marks::TextMarkGroup>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let tick_mark_style = style_sheet.tick_mark_style();
        let text_mark_style = style_sheet.text_mark_style();

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

        let rail = if let Some(rail_style) = &style.rail {
            draw_rail(rail_style, &bounds)
        } else {
            Primitive::None
        };

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

        let handle_bottom = if let Some(handle_layer) = &style.handle_bottom {
            draw_handle_layer(handle_layer, &handle_bounds)
        } else {
            Primitive::None
        };

        let handle_top = if let Some(handle_layer) = &style.handle_top {
            draw_handle_layer(handle_layer, &handle_bounds)
        } else {
            Primitive::None
        };

        (
            Primitive::Group {
                primitives: vec![
                    tick_marks_primitive,
                    text_marks_primitive,
                    rail,
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

fn draw_rail(rail_style: &Rail, bounds: &Rectangle) -> Primitive {
    match rail_style {
        Rail::Classic {
            colors,
            widths,
            edge_padding,
        } => {
            let (left_color, right_color) = colors;
            let left_width = f32::from(widths.0);
            let right_width = f32::from(widths.1);
            let edge_padding = f32::from(*edge_padding);

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
                background: Background::Color(*left_color),
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
                background: Background::Color(*right_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };

            Primitive::Group {
                primitives: vec![left_rail, right_rail],
            }
        }
        Rail::Rectangle {
            color,
            border_color,
            border_width,
            border_radius,
        } => Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            background: Background::Color(*color),
            border_radius: *border_radius,
            border_width: *border_width,
            border_color: *border_color,
        },
        Rail::Texture {
            image_handle,
            width,
            height,
            edge_padding,
            offset,
        } => {
            let width = if let Some(width) = width {
                f32::from(*width)
            } else {
                bounds.width
            };

            let height = if let Some(height) = height {
                f32::from(*height) - (f32::from(*edge_padding) * 2.0)
            } else {
                bounds.height
            };

            Primitive::Image {
                handle: image_handle.clone(),
                /// The bounds of the image
                bounds: Rectangle {
                    x: (bounds.x + offset.x + ((bounds.width - width) / 2.0))
                        .round(),
                    y: (bounds.y
                        + offset.y
                        + f32::from(*edge_padding)
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
    if value_fill.bipolar && value_normal.value() == 0.5 {
        return Primitive::None;
    }

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

    let (y, height) = if value_fill.bipolar {
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
    } else {
        if value_fill.from_bottom {
            let y = (handle_bounds.center_y()
                + f32::from(value_fill.handle_spacing)
                - f32::from(value_fill.border_width))
            .round();
            (
                y,
                bounds.y + bounds.height
                    - f32::from(value_fill.edge_padding)
                    - y,
            )
        } else {
            let y = bounds.y + f32::from(value_fill.edge_padding);
            (
                y,
                (handle_bounds.center_y()
                    - f32::from(value_fill.handle_spacing)
                    + f32::from(value_fill.border_width)
                    - y)
                    .floor(),
            )
        }
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
        HandleLayer::Rectangle {
            color,
            border_color,
            border_width,
            border_radius,
            width,
            height,
            offset,
        } => {
            let width = if let Some(width) = width {
                f32::from(*width)
            } else {
                handle_bounds.width
            };

            let height = if let Some(height) = height {
                f32::from(*height)
            } else {
                handle_bounds.height
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + offset.x
                        + ((handle_bounds.width - width) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + offset.y
                        + ((handle_bounds.height - height) / 2.0))
                        .round(),
                    width,
                    height,
                },
                background: Background::Color(*color),
                border_radius: *border_radius,
                border_width: *border_width,
                border_color: *border_color,
            }
        }
        HandleLayer::Circle {
            color,
            border_color,
            border_width,
            diameter,
            offset,
        } => {
            let diameter = if let Some(diameter) = diameter {
                f32::from(*diameter)
            } else {
                handle_bounds.height
            };

            Primitive::Quad {
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + offset.x
                        + ((handle_bounds.width - diameter) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + offset.y
                        + ((handle_bounds.height - diameter) / 2.0))
                        .round(),
                    width: diameter,
                    height: diameter,
                },
                background: Background::Color(*color),
                border_radius: (diameter / 2.0) as u16,
                border_width: *border_width,
                border_color: *border_color,
            }
        }
        HandleLayer::Texture {
            image_handle,
            width,
            height,
            offset,
        } => {
            let width = if let Some(width) = width {
                f32::from(*width)
            } else {
                handle_bounds.width
            };

            let height = if let Some(height) = height {
                f32::from(*height)
            } else {
                handle_bounds.height
            };

            Primitive::Image {
                handle: image_handle.clone(),
                /// The bounds of the image
                bounds: Rectangle {
                    x: (handle_bounds.x
                        + offset.x
                        + ((handle_bounds.width - width) / 2.0))
                        .round(),
                    y: (handle_bounds.y
                        + offset.y
                        + ((handle_bounds.height - height) / 2.0))
                        .round(),
                    width,
                    height,
                },
            }
        }
    }
}

fn draw_tick_mark_tier_merged(
    primitives: &mut Vec<Primitive>,
    tick_mark_positions: &Vec<Normal>,
    width: f32,
    length_scale: f32,
    color: &Color,
    bounds: &Rectangle,
    center_x: f32,
) {
    let length = (length_scale * bounds.width).round();
    let color = Background::Color(*color);
    let start_y = bounds.y - (width / 2.0);
    let x = (center_x - (length / 2.0)).round();

    for position in tick_mark_positions.iter() {
        let y = (start_y + position.scale_inv(bounds.height)).round();

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x,
                y,
                width: length,
                height: width,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });
    }
}

fn draw_tick_mark_tier(
    primitives: &mut Vec<Primitive>,
    tick_mark_positions: &Vec<Normal>,
    width: f32,
    length_scale: f32,
    color: &Color,
    bounds: &Rectangle,
    center_x: f32,
    center_offset: f32,
) {
    let length = (length_scale * bounds.width).round();
    let half_length = (length / 2.0).round();
    let color = Background::Color(*color);
    let start_y = bounds.y - (width / 2.0);

    let left_x = center_x - center_offset - half_length;
    let right_x = center_x + center_offset;

    for position in tick_mark_positions.iter() {
        let y = (start_y + position.scale_inv(bounds.height)).round();

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x: left_x,
                y,
                width: half_length,
                height: width,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x: right_x,
                y,
                width: half_length,
                height: width,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });
    }
}
