use crate::native::text_marks;
use crate::style::text_marks::{Placement, Style};

use iced_graphics::Primitive;
use iced_native::{Align, HorizontalAlignment, Rectangle, VerticalAlignment};

fn draw_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    text_marks: &text_marks::Group,
    style: &Style,
    inverse: bool,
    align: VerticalAlignment,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    let start_x = bounds.x + style.offset.x;
    let y = (y + style.offset.y).round();

    if inverse {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x: (start_x + (text_mark.0.scale_inv(bounds.width)))
                        .round(),
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: align,
            });
        }
    } else {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x: (start_x + (text_mark.0.scale(bounds.width))).round(),
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: align,
            });
        }
    }
}

pub fn draw_horizontal_text_marks(
    bounds: &Rectangle,
    text_marks: &text_marks::Group,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let primitives = match style.placement {
        Placement::BothSides { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len() * 2);

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Top,
                );
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Bottom,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Bottom,
                );
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Top,
                );
            }

            primitives
        }
        Placement::LeftOrTop { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Top,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Bottom,
                );
            }

            primitives
        }
        Placement::RightOrBottom { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Bottom,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    VerticalAlignment::Top,
                );
            }

            primitives
        }
        Placement::Center { align } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            match align {
                Align::Start => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        VerticalAlignment::Top,
                    );
                }
                Align::End => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        VerticalAlignment::Bottom,
                    );
                }
                Align::Center => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        VerticalAlignment::Center,
                    );
                }
            }

            primitives
        }
    };

    Primitive::Group { primitives }
}
