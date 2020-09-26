//! `iced_graphics` renderer for the [`VSlider`] widget
//!
//! [`VSlider`]: ../native/v_slider/struct.VSlider.html

use crate::core::{ModulationRange, Normal, TextMarkGroup, TickMarkGroup};
use crate::graphics::bar_text_marks;
use crate::native::v_slider;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Point, Rectangle};

pub use crate::native::v_slider::State;
pub use crate::style::v_slider::{
    HandleLayer, ModRangePosition, ModRangeStyle, Rail, Style, StyleSheet,
    TickMarkStyle, ValueFill,
};

/// This is an alias of a `crate::native` [`VSlider`] with an
/// `iced_graphics::Renderer`.
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
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
        mod_range: Option<ModulationRange>,
        tick_marks: Option<&TickMarkGroup>,
        text_marks: Option<&TextMarkGroup>,
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
            if let Some(tick_mark_style) = &tick_mark_style {
                draw_tick_marks(tick_marks, tick_mark_style, &mark_bounds)
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        };

        let text_marks_primitive = if let Some(text_marks) = text_marks {
            if let Some(text_mark_style) = &text_mark_style {
                bar_text_marks::draw_vertical_text_marks(
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

        let (rail, back_border_width) = if let Some(rail_style) = &style.rail {
            draw_rail(rail_style, &bounds)
        } else {
            (Primitive::None, 0)
        };

        let handle_bounds = Rectangle {
            x: bounds.x,
            y: (bounds.y + (mark_bounds.height * normal.inv())).round(),
            width: bounds.width,
            height: handle_height,
        };

        let value_fill = if let Some(value_fill_style) = &style.value_fill {
            draw_value_fill(
                value_fill_style,
                &bounds,
                &handle_bounds,
                back_border_width,
                normal
            )
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
                    handle_bottom,
                    handle_top,
                ],
            },
            mouse::Interaction::default(),
        )
    }
}

fn draw_rail(rail_style: &Rail, bounds: &Rectangle) -> (Primitive, u16) {
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
            (
                Primitive::Group {
                    primitives: vec![left_rail, right_rail],
                },
                0,
            )
        }
        Rail::Texture { texture } => (Primitive::None, 0),
        Rail::Rectangle {
            color,
            border_color,
            border_width,
            border_radius,
        } => (
            Primitive::Quad {
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
            *border_width,
        ),
    }
}

fn draw_value_fill(
    value_fill: &ValueFill,
    bounds: &Rectangle,
    handle_bounds: &Rectangle,
    back_border_width: u16,
    value_normal: Normal,
) -> Primitive {
    match value_fill {
        ValueFill::Unipolar {
            color,
            corner_radius,
            handle_spacing,
            width,
            h_offset,
            from_bottom,
        } => {
            let (x, width) = if let Some(width) = width {
                let width = f32::from(*width);
                (
                    (bounds.x
                        + ((bounds.width - width) / 2.0)
                        + f32::from(*h_offset))
                    .round(),
                    width,
                )
            } else {
                (bounds.x + f32::from(*h_offset), bounds.width)
            };

            let (y, height) = if *from_bottom {
                let y = handle_bounds.y
                    + handle_bounds.height
                    + f32::from(*handle_spacing)
                    - (f32::from(back_border_width) * 2.0);
                (
                    y,
                    bounds.y + bounds.height - y
                )
            } else {
                (
                    bounds.y,
                    handle_bounds.y
                            - f32::from(*handle_spacing)
                            + (f32::from(back_border_width) * 2.0)
                            - bounds.y
                )
            };

            Primitive::Quad {
                bounds: Rectangle { x, y, width, height },
                background: Background::Color(*color),
                border_radius: *corner_radius,
                border_width: back_border_width,
                border_color: Color::TRANSPARENT,
            }
        },
        ValueFill::Bipolar {
            bottom_color,
            top_color,
            corner_radius,
            handle_spacing,
            width,
            h_offset,
        } => {
            if value_normal.value() == 0.5 {
                return Primitive::None;
            }

            let (x, width) = if let Some(width) = width {
                let width = f32::from(*width);
                (
                    (bounds.x
                        + ((bounds.width - width) / 2.0)
                        + f32::from(*h_offset))
                    .round(),
                    width,
                )
            } else {
                (bounds.x + f32::from(*h_offset), bounds.width)
            };

            let center_y = bounds.center_y().round();

            let (y, height, color) = if value_normal.value() > 0.5 {
                let y = handle_bounds.y
                    + handle_bounds.height
                    + f32::from(*handle_spacing)
                    - (f32::from(back_border_width) * 2.0);
                (
                    y,
                    center_y - y,
                    *top_color
                )
            } else {
                (
                    center_y,
                    handle_bounds.y
                            - f32::from(*handle_spacing)
                            + (f32::from(back_border_width) * 2.0)
                            - center_y,
                    *bottom_color
                )
            };

            Primitive::Quad {
                bounds: Rectangle { x, y, width, height },
                background: Background::Color(color),
                border_radius: *corner_radius,
                border_width: back_border_width,
                border_color: Color::TRANSPARENT,
            }
        }
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

fn draw_tick_marks(
    tick_marks: &TickMarkGroup,
    style: &TickMarkStyle,
    bounds: &Rectangle,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let center_x = bounds.center_x();

    if style.center_offset == 0 {
        primitives.reserve_exact(tick_marks.len());

        if tick_marks.has_tier_1() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_1_positions(),
                style.width_tier_1 as f32,
                style.length_scale_tier_1,
                &style.color_tier_1,
                bounds,
                center_x,
            );
        }
        if tick_marks.has_tier_2() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_2_positions(),
                style.width_tier_2 as f32,
                style.length_scale_tier_2,
                &style.color_tier_2,
                bounds,
                center_x,
            );
        }
        if tick_marks.has_tier_3() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_3_positions(),
                style.width_tier_3 as f32,
                style.length_scale_tier_3,
                &style.color_tier_3,
                bounds,
                center_x,
            );
        }
    } else {
        primitives.reserve_exact(tick_marks.len() * 2);

        let center_offset = style.center_offset as f32;

        if tick_marks.has_tier_1() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_1_positions(),
                style.width_tier_1 as f32,
                style.length_scale_tier_1,
                &style.color_tier_1,
                bounds,
                center_x,
                center_offset,
            );
        }
        if tick_marks.has_tier_2() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_2_positions(),
                style.width_tier_2 as f32,
                style.length_scale_tier_2,
                &style.color_tier_2,
                bounds,
                center_x,
                center_offset,
            );
        }
        if tick_marks.has_tier_3() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_3_positions(),
                style.width_tier_3 as f32,
                style.length_scale_tier_3,
                &style.color_tier_3,
                bounds,
                center_x,
                center_offset,
            );
        }
    }

    Primitive::Group { primitives }
}
