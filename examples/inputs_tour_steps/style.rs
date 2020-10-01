use iced::{button, image, Align, Background, Color, Point, Size, Vector};
use iced_audio::{
    h_slider, knob, mod_range_input, ramp, text_marks, tick_marks, v_slider,
    xy_pad, Normal,
};

pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => BUTTON_PRIMARY_COLOR,
                Button::Secondary => BUTTON_SECONDARY_COLOR,
            })),
            border_radius: 12,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}

pub const BUTTON_PRIMARY_COLOR: Color = Color::from_rgb(
    0x32 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0xC8 as f32 / 255.0,
);

pub const BUTTON_SECONDARY_COLOR: Color = Color::from_rgb(
    0x62 as f32 / 255.0,
    0x69 as f32 / 255.0,
    0x73 as f32 / 255.0,
);

pub const EMPTY_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
pub const BORDER_COLOR: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0x33 as f32 / 255.0,
    0x3C as f32 / 255.0,
);
pub const FILLED_COLOR: Color = Color::from_rgb(
    0x29 as f32 / 255.0,
    0x66 as f32 / 255.0,
    0xA3 as f32 / 255.0,
);
pub const FILLED_HOVER_COLOR: Color = Color::from_rgb(
    0x33 as f32 / 255.0,
    0x70 as f32 / 255.0,
    0xAD as f32 / 255.0,
);
pub const BP_FILLED_COLOR: Color = Color::from_rgb(0.0, 0.605, 0.0);
pub const BP_FILLED_HOVER_COLOR: Color = Color::from_rgb(0.0, 0.64, 0.0);
pub const BP_HANDLE_COLOR: Color = Color::from_rgb(0.0, 0.9, 0.0);
pub const BP_HANDLE_HOVER_COLOR: Color = Color::from_rgb(0.0, 0.95, 0.0);
pub const CENTER_HANDLE_COLOR: Color = Color::from_rgb(0.7, 0.7, 0.7);
pub const CENTER_HANDLE_HOVER_COLOR: Color = Color::from_rgb(0.8, 0.8, 0.8);
pub const HANDLE_COLOR: Color = Color::from_rgb(
    0x75 as f32 / 255.0,
    0xC2 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const HANDLE_HOVER_COLOR: Color = Color::from_rgb(
    0x7A as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const KNOB_COLOR: Color = Color::from_rgb(
    0x56 as f32 / 255.0,
    0x59 as f32 / 255.0,
    0x62 as f32 / 255.0,
);
pub const KNOB_BORDER_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
pub const KNOB_ARC_COLOR: Color = Color::from_rgb(
    0x3D as f32 / 255.0,
    0x9E as f32 / 255.0,
    0xE9 as f32 / 255.0,
);
pub const KNOB_ARC_RIGHT_COLOR: Color = Color::from_rgb(0.0, 0.77, 0.0);
pub const KNOB_ARC_EMPTY_COLOR: Color = Color::from_rgb(0.85, 0.85, 0.85);

// Custom style for the Rect HSlider

pub struct HSliderRectStyle;
impl HSliderRectStyle {
    fn active_value_fill() -> h_slider::ValueFill {
        h_slider::ValueFill {
            color: FILLED_COLOR,
            border_width: 1,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            handle_spacing: 2,
            height: None,
            fill_mode: h_slider::ValueFillMode::FromLeft { padding: 0 },
            v_offset: 0,
        }
    }
    fn active_handle_bottom() -> h_slider::RectangleLayer {
        h_slider::RectangleLayer {
            color: HANDLE_COLOR,
            border_color: Color::TRANSPARENT,
            border_width: 1,
            border_radius: 2,
            width: None,
            height: None,
            offset: Point::ORIGIN,
        }
    }
}
impl h_slider::StyleSheet for HSliderRectStyle {
    fn active(&self, _value: Normal) -> h_slider::Style {
        h_slider::Style {
            rail: h_slider::Rail::Rectangle(h_slider::RectangleRail {
                color: EMPTY_COLOR,
                border_color: BORDER_COLOR,
                border_width: 1,
                border_radius: 2,
                height: None,
                edge_padding: 0,
            }),
            value_fill: Some(Self::active_value_fill()),
            handle_width: 5,
            handle_bottom: h_slider::HandleLayer::Rectangle(
                Self::active_handle_bottom(),
            ),
            handle_top: h_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> h_slider::Style {
        let active = self.active(value);
        h_slider::Style {
            handle_width: 6,
            value_fill: Some(h_slider::ValueFill {
                color: FILLED_HOVER_COLOR,
                handle_spacing: 3,
                ..Self::active_value_fill()
            }),
            handle_bottom: h_slider::HandleLayer::Rectangle(
                h_slider::RectangleLayer {
                    color: HANDLE_HOVER_COLOR,
                    ..Self::active_handle_bottom()
                },
            ),
            ..active
        }
    }

    fn dragging(&self, value: Normal) -> h_slider::Style {
        self.hovered(value)
    }

    fn mod_range_style(&self) -> Option<h_slider::ModRangeStyle> {
        Some(h_slider::ModRangeStyle {
            back_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
            border_width: 0,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            filled_color: KNOB_ARC_RIGHT_COLOR,
            filled_color_inv: KNOB_ARC_COLOR,
            height: Some(3),
            placement: h_slider::ModRangePlacement::Bottom,
            edge_padding: 1,
            offset: Point { x: 0.0, y: 2.0 },
        })
    }
}

// Custom style for the Rect VSlider

pub struct VSliderRectStyle;
impl VSliderRectStyle {
    fn active_value_fill() -> v_slider::ValueFill {
        v_slider::ValueFill {
            color: FILLED_COLOR,
            border_width: 1,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            handle_spacing: 2,
            width: None,
            fill_mode: v_slider::ValueFillMode::FromBottom { padding: 0 },
            h_offset: 0,
        }
    }
    fn active_handle_bottom() -> v_slider::RectangleLayer {
        v_slider::RectangleLayer {
            color: HANDLE_COLOR,
            border_color: Color::TRANSPARENT,
            border_width: 1,
            border_radius: 2,
            width: None,
            height: None,
            offset: Point::ORIGIN,
        }
    }
}
impl v_slider::StyleSheet for VSliderRectStyle {
    fn active(&self, _value: Normal) -> v_slider::Style {
        v_slider::Style {
            rail: v_slider::Rail::Rectangle(v_slider::RectangleRail {
                color: EMPTY_COLOR,
                border_color: BORDER_COLOR,
                border_width: 1,
                border_radius: 2,
                width: None,
                edge_padding: 0,
            }),
            value_fill: Some(Self::active_value_fill()),
            handle_height: 5,
            handle_bottom: v_slider::HandleLayer::Rectangle(
                Self::active_handle_bottom(),
            ),
            handle_top: v_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> v_slider::Style {
        let active = self.active(value);
        v_slider::Style {
            handle_height: 6,
            value_fill: Some(v_slider::ValueFill {
                color: FILLED_HOVER_COLOR,
                handle_spacing: 3,
                ..Self::active_value_fill()
            }),
            handle_bottom: v_slider::HandleLayer::Rectangle(
                v_slider::RectangleLayer {
                    color: HANDLE_HOVER_COLOR,
                    ..Self::active_handle_bottom()
                },
            ),
            ..active
        }
    }

    fn dragging(&self, value: Normal) -> v_slider::Style {
        self.hovered(value)
    }

    fn mod_range_style(&self) -> Option<v_slider::ModRangeStyle> {
        Some(v_slider::ModRangeStyle {
            back_color: None,
            border_width: 1,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            filled_color: Color {
                r: 0.0,
                g: 0.7,
                b: 0.0,
                a: 0.3,
            },
            filled_color_inv: Color {
                r: 0.0,
                g: 0.7,
                b: 0.0,
                a: 0.3,
            },
            width: None,
            placement: v_slider::ModRangePlacement::Center,
            edge_padding: 0,
            offset: Point::ORIGIN,
        })
    }
}

// Custom style for the Rect Bipolar HSlider

pub struct HSliderRectBipolarStyle;
impl HSliderRectBipolarStyle {
    fn active_value_fill_positive() -> h_slider::ValueFill {
        h_slider::ValueFill {
            color: FILLED_COLOR,
            border_width: 1,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            handle_spacing: 2,
            height: None,
            fill_mode: h_slider::ValueFillMode::FromCenter,
            v_offset: 0,
        }
    }

    fn active_handle_center() -> h_slider::RectangleLayer {
        h_slider::RectangleLayer {
            color: CENTER_HANDLE_COLOR,
            border_color: Color::TRANSPARENT,
            border_width: 1,
            border_radius: 2,
            width: None,
            height: None,
            offset: Point::ORIGIN,
        }
    }
}
impl h_slider::StyleSheet for HSliderRectBipolarStyle {
    fn active(&self, value: Normal) -> h_slider::Style {
        h_slider::Style {
            rail: h_slider::Rail::Rectangle(h_slider::RectangleRail {
                color: EMPTY_COLOR,
                border_color: BORDER_COLOR,
                border_width: 1,
                border_radius: 2,
                height: None,
                edge_padding: 0,
            }),
            value_fill: Some(if value > Normal::center() {
                Self::active_value_fill_positive()
            } else {
                h_slider::ValueFill {
                    color: BP_FILLED_COLOR,
                    ..Self::active_value_fill_positive()
                }
            }),
            handle_width: 5,
            handle_bottom: h_slider::HandleLayer::Rectangle(
                if value == Normal::center() {
                    Self::active_handle_center()
                } else if value > Normal::center() {
                    h_slider::RectangleLayer {
                        color: HANDLE_COLOR,
                        ..Self::active_handle_center()
                    }
                } else {
                    h_slider::RectangleLayer {
                        color: BP_HANDLE_COLOR,
                        ..Self::active_handle_center()
                    }
                },
            ),
            handle_top: h_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> h_slider::Style {
        let active = self.active(value);
        h_slider::Style {
            value_fill: Some(if value > Normal::center() {
                h_slider::ValueFill {
                    color: FILLED_HOVER_COLOR,
                    handle_spacing: 3,
                    ..Self::active_value_fill_positive()
                }
            } else {
                h_slider::ValueFill {
                    color: BP_FILLED_HOVER_COLOR,
                    handle_spacing: 3,
                    ..Self::active_value_fill_positive()
                }
            }),
            handle_width: 6,
            handle_bottom: h_slider::HandleLayer::Rectangle(
                if value == Normal::center() {
                    h_slider::RectangleLayer {
                        color: CENTER_HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                } else if value > Normal::center() {
                    h_slider::RectangleLayer {
                        color: HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                } else {
                    h_slider::RectangleLayer {
                        color: BP_HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                },
            ),
            ..active
        }
    }

    fn dragging(&self, value: Normal) -> h_slider::Style {
        self.hovered(value)
    }
}

// Custom style for the Rect Bipolar VSlider

pub struct VSliderRectBipolarStyle;
impl VSliderRectBipolarStyle {
    fn active_value_fill_positive() -> v_slider::ValueFill {
        v_slider::ValueFill {
            color: FILLED_COLOR,
            border_width: 1,
            border_radius: 2,
            border_color: Color::TRANSPARENT,
            handle_spacing: 2,
            width: None,
            fill_mode: v_slider::ValueFillMode::FromCenter,
            h_offset: 0,
        }
    }

    fn active_handle_center() -> v_slider::RectangleLayer {
        v_slider::RectangleLayer {
            color: CENTER_HANDLE_COLOR,
            border_color: Color::TRANSPARENT,
            border_width: 1,
            border_radius: 2,
            width: None,
            height: None,
            offset: Point::ORIGIN,
        }
    }
}
impl v_slider::StyleSheet for VSliderRectBipolarStyle {
    fn active(&self, value: Normal) -> v_slider::Style {
        v_slider::Style {
            rail: v_slider::Rail::Rectangle(v_slider::RectangleRail {
                color: EMPTY_COLOR,
                border_color: BORDER_COLOR,
                border_width: 1,
                border_radius: 2,
                width: None,
                edge_padding: 0,
            }),
            value_fill: Some(if value > Normal::center() {
                Self::active_value_fill_positive()
            } else {
                v_slider::ValueFill {
                    color: BP_FILLED_COLOR,
                    ..Self::active_value_fill_positive()
                }
            }),
            handle_height: 5,
            handle_bottom: v_slider::HandleLayer::Rectangle(
                if value == Normal::center() {
                    Self::active_handle_center()
                } else if value > Normal::center() {
                    v_slider::RectangleLayer {
                        color: HANDLE_COLOR,
                        ..Self::active_handle_center()
                    }
                } else {
                    v_slider::RectangleLayer {
                        color: BP_HANDLE_COLOR,
                        ..Self::active_handle_center()
                    }
                },
            ),
            handle_top: v_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> v_slider::Style {
        let active = self.active(value);
        v_slider::Style {
            value_fill: Some(if value > Normal::center() {
                v_slider::ValueFill {
                    color: FILLED_HOVER_COLOR,
                    handle_spacing: 3,
                    ..Self::active_value_fill_positive()
                }
            } else {
                v_slider::ValueFill {
                    color: BP_FILLED_HOVER_COLOR,
                    handle_spacing: 3,
                    ..Self::active_value_fill_positive()
                }
            }),
            handle_height: 6,
            handle_bottom: v_slider::HandleLayer::Rectangle(
                if value == Normal::center() {
                    v_slider::RectangleLayer {
                        color: CENTER_HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                } else if value > Normal::center() {
                    v_slider::RectangleLayer {
                        color: HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                } else {
                    v_slider::RectangleLayer {
                        color: BP_HANDLE_HOVER_COLOR,
                        ..Self::active_handle_center()
                    }
                },
            ),
            ..active
        }
    }

    fn dragging(&self, value: Normal) -> v_slider::Style {
        self.hovered(value)
    }
}

// Custom style for the Texture HSlider

pub struct HSliderTextureStyle {
    pub handle: image::Handle,
    pub size: Size<u16>,
    pub offset: Point,
}
impl h_slider::StyleSheet for HSliderTextureStyle {
    fn active(&self, _value: Normal) -> h_slider::Style {
        h_slider::Style {
            rail: h_slider::Rail::Classic(h_slider::ClassicRail {
                colors: (
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.9,
                    },
                    Color {
                        r: 0.36,
                        g: 0.36,
                        b: 0.36,
                        a: 0.75,
                    },
                ),
                widths: (1, 2),
                edge_padding: 12,
            }),
            value_fill: None,
            handle_width: self.size.width,
            handle_bottom: h_slider::HandleLayer::Texture(
                h_slider::TextureLayer {
                    image_handle: self.handle.clone(),
                    width: Some(self.size.width),
                    height: None,
                    offset: self.offset,
                },
            ),
            handle_top: h_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> h_slider::Style {
        self.active(value)
    }

    fn dragging(&self, value: Normal) -> h_slider::Style {
        self.active(value)
    }

    fn tick_marks_style(
        &self,
    ) -> Option<(tick_marks::Style, tick_marks::Placement)> {
        Some((
            tick_marks::Style {
                tier_1: Some(tick_marks::Shape::Line {
                    length: 10,
                    width: 2,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
                tier_2: Some(tick_marks::Shape::Line {
                    length: 8,
                    width: 2,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
                tier_3: Some(tick_marks::Shape::Line {
                    length: 7,
                    width: 1,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
            },
            tick_marks::Placement::CenterSplit {
                fill_length: false,
                gap: 11,
            },
        ))
    }

    fn text_marks_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            text_size: 12,
            font: Default::default(),
            bounds_width: 30,
            bounds_height: 14,
            placement: text_marks::Placement::Center {
                align: Align::Start,
            },
            offset: Point { x: 0.0, y: 19.0 },
        })
    }
}

// Custom style for the Texture VSlider

pub struct VSliderTextureStyle {
    pub handle: image::Handle,
    pub size: Size<u16>,
    pub offset: Point,
}
impl v_slider::StyleSheet for VSliderTextureStyle {
    fn active(&self, _value: Normal) -> v_slider::Style {
        v_slider::Style {
            rail: v_slider::Rail::Classic(v_slider::ClassicRail {
                colors: (
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.9,
                    },
                    Color {
                        r: 0.36,
                        g: 0.36,
                        b: 0.36,
                        a: 0.75,
                    },
                ),
                widths: (1, 2),
                edge_padding: 12,
            }),
            value_fill: None,
            handle_height: self.size.height,
            handle_bottom: v_slider::HandleLayer::Texture(
                v_slider::TextureLayer {
                    image_handle: self.handle.clone(),
                    width: Some(self.size.width),
                    height: None,
                    offset: self.offset,
                },
            ),
            handle_top: v_slider::HandleLayer::None,
        }
    }

    fn hovered(&self, value: Normal) -> v_slider::Style {
        self.active(value)
    }

    fn dragging(&self, value: Normal) -> v_slider::Style {
        self.active(value)
    }

    fn tick_marks_style(
        &self,
    ) -> Option<(tick_marks::Style, tick_marks::Placement)> {
        Some((
            tick_marks::Style {
                tier_1: Some(tick_marks::Shape::Line {
                    length: 10,
                    width: 2,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
                tier_2: Some(tick_marks::Shape::Line {
                    length: 8,
                    width: 2,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
                tier_3: Some(tick_marks::Shape::Line {
                    length: 7,
                    width: 1,
                    color: Color {
                        r: 0.56,
                        g: 0.56,
                        b: 0.56,
                        a: 0.75,
                    },
                }),
            },
            tick_marks::Placement::CenterSplit {
                fill_length: false,
                gap: 11,
            },
        ))
    }

    fn text_marks_style(&self) -> Option<text_marks::Style> {
        Some(text_marks::Style {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            text_size: 12,
            font: Default::default(),
            bounds_width: 30,
            bounds_height: 14,
            placement: text_marks::Placement::Center { align: Align::End },
            offset: Point { x: -19.0, y: 0.0 },
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleCircle;
impl knob::StyleSheet for KnobCustomStyleCircle {
    fn active(&self) -> knob::Style {
        knob::Style::ClassicCircle(knob::ClassicCircleStyle {
            color: KNOB_COLOR,
            border_width: 3,
            border_color: KNOB_BORDER_COLOR,
            notch_color: HANDLE_COLOR,
            notch_border_width: 1,
            notch_border_color: FILLED_COLOR,
            notch_scale: 0.21.into(),
            notch_offset: 0.21.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        let active = self.active();
        if let knob::Style::ClassicCircle(active) = self.active() {
            knob::Style::ClassicCircle(knob::ClassicCircleStyle {
                notch_color: HANDLE_HOVER_COLOR,
                notch_border_color: FILLED_HOVER_COLOR,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn value_ring_style(&self) -> Option<knob::ValueRingStyle> {
        Some(knob::ValueRingStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: None,
        })
    }

    fn mod_range_ring_style(&self) -> Option<knob::ModRangeRingStyle> {
        Some(knob::ModRangeRingStyle {
            width: 3.0,
            offset: 6.0,
            empty_color: Some(KNOB_ARC_EMPTY_COLOR),
            filled_color: KNOB_ARC_RIGHT_COLOR,
            filled_inverse_color: KNOB_ARC_RIGHT_COLOR,
        })
    }

    fn text_mark_style(&self) -> Option<knob::TextMarkStyle> {
        Some(knob::TextMarkStyle {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            offset: 15.0,
            text_size: 11,
            font: Default::default(),
            bounds_width: 20,
            bounds_height: 20,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleLine;
impl knob::StyleSheet for KnobCustomStyleLine {
    fn active(&self) -> knob::Style {
        knob::Style::ClassicLine(knob::ClassicLineStyle {
            color: KNOB_COLOR,
            border_width: 0,
            border_color: KNOB_BORDER_COLOR,
            notch_color: Color::from_rgb(0.0, 0.82, 0.0),
            notch_width: 3.5,
            notch_scale: 0.35.into(),
            notch_offset: 0.21.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn value_ring_style(&self) -> Option<knob::ValueRingStyle> {
        Some(knob::ValueRingStyle {
            width: 2.5,
            offset: 2.0,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: Some(KNOB_ARC_RIGHT_COLOR),
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomArc;
impl knob::StyleSheet for KnobCustomArc {
    fn active(&self) -> knob::Style {
        knob::Style::Arc(knob::ArcStyle {
            width: 3.15,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            filled_color: KNOB_ARC_COLOR,
            notch: Some(knob::ArcNotch {
                width: 3.15,
                length_scale: 0.55.into(),
                color: KNOB_ARC_COLOR,
            }),
        })
    }

    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn angle_range(&self) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }

    fn mod_range_ring_style(&self) -> Option<knob::ModRangeRingStyle> {
        Some(knob::ModRangeRingStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: Some(KNOB_ARC_EMPTY_COLOR),
            filled_color: KNOB_ARC_COLOR,
            filled_inverse_color: KNOB_ARC_RIGHT_COLOR,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomArcBipolar;
impl knob::StyleSheet for KnobCustomArcBipolar {
    fn active(&self) -> knob::Style {
        knob::Style::ArcBipolar(knob::ArcBipolarStyle {
            width: 3.15,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: KNOB_ARC_RIGHT_COLOR,
            notch: Some(knob::ArcBipolarNotch {
                width: 3.15,
                length_scale: 0.55.into(),
                center_color: EMPTY_COLOR,
                left_color: KNOB_ARC_COLOR,
                right_color: KNOB_ARC_RIGHT_COLOR,
            }),
        })
    }

    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn angle_range(&self) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }
}

// Custom style for the ModRangeInput

pub struct ModRangeInputCustom;

impl mod_range_input::StyleSheet for ModRangeInputCustom {
    fn active(&self) -> mod_range_input::Style {
        mod_range_input::Style::Circle(mod_range_input::CircleStyle {
            color: KNOB_ARC_RIGHT_COLOR,
            border_width: 2,
            border_color: Color::from_rgb(0.0, 0.6, 0.0),
        })
    }

    fn hovered(&self) -> mod_range_input::Style {
        let active = self.active();
        if let mod_range_input::Style::Circle(active) = self.active() {
            mod_range_input::Style::Circle(mod_range_input::CircleStyle {
                border_width: 1,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> mod_range_input::Style {
        self.hovered()
    }
}

// Custom style for the Texture VSlider

pub struct XYPadCustomStyle;
impl xy_pad::StyleSheet for XYPadCustomStyle {
    fn active(&self) -> xy_pad::Style {
        xy_pad::Style {
            rail_width: 1,
            h_rail_color: HANDLE_COLOR,
            v_rail_color: HANDLE_COLOR,
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_COLOR,
                size: 10,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            back_color: EMPTY_COLOR,
            border_width: 1,
            border_color: Color::BLACK,
            center_line_width: 1,
            center_line_color: [0.0, 0.0, 0.0, 0.4].into(),
        }
    }

    fn hovered(&self) -> xy_pad::Style {
        let active = self.active();

        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_HOVER_COLOR,
                size: 12,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            ..active
        }
    }

    fn dragging(&self) -> xy_pad::Style {
        let active = self.active();

        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_HOVER_COLOR,
                size: 10,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            ..active
        }
    }
}

// Custom style for the Texture VSlider

pub struct RampCustomStyle;
impl ramp::StyleSheet for RampCustomStyle {
    fn active(&self) -> ramp::Style {
        ramp::Style {
            back_color: KNOB_COLOR,
            back_border_width: 2,
            back_border_color: KNOB_BORDER_COLOR,
            line_width: 2.0,
            line_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            line_up_color: Color::from_rgb(0.0, 0.9, 0.0),
            line_down_color: HANDLE_COLOR,
        }
    }

    fn hovered(&self) -> ramp::Style {
        let active = self.active();

        ramp::Style {
            line_center_color: Color::from_rgb(0.8, 0.8, 0.8),
            line_up_color: Color::from_rgb(0.0, 1.0, 0.0),
            line_down_color: Color::from_rgb(
                0x8A as f32 / 255.0,
                0xD7 as f32 / 255.0,
                0xFF as f32 / 255.0,
            ),
            ..active
        }
    }

    fn dragging(&self) -> ramp::Style {
        self.hovered()
    }
}
