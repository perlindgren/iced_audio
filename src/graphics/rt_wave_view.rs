//! `iced_graphics` renderer for the [`RtWaveView`] widget
//!
//! [`RtWaveView`]: ../native/rt_wave_view/struct.RtWaveView.html

use crate::native::rt_wave_view;
use iced_graphics::canvas::{Fill, Frame, LineCap, LineJoin, Path, Stroke};
use iced_graphics::{Backend, Primitive, Renderer, Size};
use iced_native::{mouse, Background, Color, Point, Rectangle, Vector};

pub use crate::native::rt_wave_view::{PlotPoint, Plot, State, Detector, Animator, peak_detector};
pub use crate::style::rt_wave_view::{Style, StyleSheet};

/// This is an alias of a `crate::native` [`RtWaveView`] with an
/// `iced_graphics::Renderer`.
///
/// [`RtWaveView`]: ../../native/rt_wave_view/struct.RtWaveView.html
pub type RtWaveView<'a, Backend> = rt_wave_view::RtWaveView<'a, Renderer<Backend>>;

impl<B: Backend> rt_wave_view::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        style_sheet: &Self::Style,
        left_plot: Option<&[Plot]>,
        right_plot: Option<&[Plot]>,
        is_dual: bool,
    ) -> Self::Output {
        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let style = style_sheet.style();

        let border_width = style.back_border_width as f32;
        let twice_border_width = border_width * 2.0;

        let back = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: bounds_y,
                width: bounds_width,
                height: bounds_height,
            },
            background: Background::Color(style.back_color),
            border_radius: 0,
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        };

        if is_dual {
            let div_line_width = style.div_line_width as f32;

            let plot_x = bounds_x + border_width;
            let plot_width = bounds_width - twice_border_width;

            let plot_height =
                ((bounds_height - twice_border_width - div_line_width) / 2.0)
                    .floor();

            let left_plot_y = bounds_y + border_width;
            let right_plot_y = left_plot_y + plot_height + div_line_width;

            let left_plot_primitive = if let Some(left_plot) = left_plot {
                draw_plot(
                    plot_x,
                    left_plot_y,
                    plot_width,
                    plot_height,
                    left_plot,
                    style.left_plot_color,
                )
            } else {
                Primitive::None
            };

            let right_plot_primitive = if let Some(right_plot) = right_plot {
                draw_plot(
                    plot_x,
                    right_plot_y,
                    plot_width,
                    plot_height,
                    right_plot,
                    style.right_plot_color,
                )
            } else {
                Primitive::None
            };

            let (left_center_line, right_center_line) =
                if let Some(color) = style.center_line_color {
                    let center_line_width = style.center_line_width as f32;
                    let center_offset = (plot_height - center_line_width) / 2.0;

                    (
                        Primitive::Quad {
                            bounds: Rectangle {
                                x: plot_x,
                                y: (left_plot_y + center_offset).round(),
                                width: plot_width,
                                height: center_line_width,
                            },
                            background: Background::Color(color),
                            border_radius: 0,
                            border_width: 0,
                            border_color: Color::TRANSPARENT,
                        },
                        Primitive::Quad {
                            bounds: Rectangle {
                                x: plot_x,
                                y: (right_plot_y + center_offset).round(),
                                width: plot_width,
                                height: center_line_width,
                            },
                            background: Background::Color(color),
                            border_radius: 0,
                            border_width: 0,
                            border_color: Color::TRANSPARENT,
                        },
                    )
                } else {
                    (Primitive::None, Primitive::None)
                };

            let div_line = Primitive::Quad {
                bounds: Rectangle {
                    x: plot_x,
                    y: (left_plot_y + plot_height).round(),
                    width: plot_width,
                    height: style.div_line_width as f32,
                },
                background: Background::Color(style.div_line_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            };

            (
                Primitive::Group {
                    primitives: vec![
                        back,
                        div_line,
                        left_center_line,
                        right_center_line,
                        left_plot_primitive,
                        right_plot_primitive,
                    ],
                },
                mouse::Interaction::default(),
            )
        } else {
            let plot_x = bounds_x + border_width;
            let plot_y = bounds_y + border_width;
            let plot_width = bounds_width - twice_border_width;
            let plot_height = bounds_height - twice_border_width;

            let left_plot_primitive = if let Some(left_plot) = left_plot {
                draw_plot(
                    plot_x,
                    plot_y,
                    plot_width,
                    plot_height,
                    left_plot,
                    style.left_plot_color,
                )
            } else {
                Primitive::None
            };

            let left_center_line = if let Some(color) = style.center_line_color
            {
                let center_line_width = style.center_line_width as f32;

                Primitive::Quad {
                    bounds: Rectangle {
                        x: plot_x,
                        y: (plot_y + ((plot_height - center_line_width) / 2.0))
                            .round(),
                        width: plot_width,
                        height: center_line_width,
                    },
                    background: Background::Color(color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            };

            (
                Primitive::Group {
                    primitives: vec![
                        back,
                        left_center_line,
                        left_plot_primitive,
                    ],
                },
                mouse::Interaction::default(),
            )
        }
    }
}

fn draw_plot(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    plot: &[PlotPoint],
    plot_color: Color,
) -> Primitive {
    let half_height = bounds_height / 2.0;

    let mut frame = Frame::new(Size::new(bounds_width, bounds_height));

    if plot.len() > 0 {
        let mut x: f32 = 0.0;

        let x_delta = (bounds_width / plot.len() as f32).round();

        for point in plot.iter() {
            frame.fill_rectangle(
                Point::new(x, half_height - (point.max * half_height)),
                Size::new(x_delta, (point.max - point.min) * half_height),
                plot_color,
            );

            x = (x + x_delta).round();
        }
    }

    Primitive::Translate {
        translation: Vector::new(bounds_x.round(), bounds_y),
        content: Box::new(frame.into_geometry().into_primitive()),
    }
}
