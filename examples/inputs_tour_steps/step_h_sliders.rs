use iced::{Column, Element, Length, Row, Text};
use iced_native::image;

use iced_audio::{
    h_slider, text_marks, tick_marks, FloatRange, FreqRange, HSlider, IntRange,
    LogDBRange,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum HSlidersID {
    Float,
    Int,
    DB,
    Freq,
    RectStyle,
    BipolarRectStyle,
    TextureStyle,
}

#[derive(Debug, Clone)]
pub enum Message {
    HSliderMoved(HSlidersID),
}

pub struct HSliderStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    h_slider_float_state: h_slider::State<HSlidersID>,
    h_slider_int_state: h_slider::State<HSlidersID>,
    h_slider_db_state: h_slider::State<HSlidersID>,
    h_slider_freq_state: h_slider::State<HSlidersID>,
    h_slider_rect_state: h_slider::State<HSlidersID>,
    h_slider_rect_bp_state: h_slider::State<HSlidersID>,
    h_slider_texture_state: h_slider::State<HSlidersID>,

    h_slider_texture_handle: image::Handle,

    float_tick_marks: tick_marks::Group,
    int_tick_marks: tick_marks::Group,
    db_tick_marks: tick_marks::Group,
    freq_tick_marks: tick_marks::Group,

    float_text_marks: text_marks::Group,
    int_text_marks: text_marks::Group,
    db_text_marks: text_marks::Group,
    freq_text_marks: text_marks::Group,

    output_text: String,
}

impl Default for HSliderStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = LogDBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            int_range,
            db_range,
            freq_range,

            // initialize the state of the HSlider widget
            h_slider_float_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::Float),
            ),

            h_slider_int_state: h_slider::State::new(
                int_range.create_param_default(HSlidersID::Int),
            ),

            h_slider_db_state: h_slider::State::new(
                db_range.create_param_default(HSlidersID::DB),
            ),

            h_slider_freq_state: h_slider::State::new(freq_range.create_param(
                HSlidersID::Freq,
                1000.0,
                1000.0,
            )),

            h_slider_rect_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::RectStyle),
            ),

            h_slider_rect_bp_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::BipolarRectStyle),
            ),

            h_slider_texture_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::TextureStyle),
            ),

            h_slider_texture_handle: format!(
                "{}/examples/images/iced_h_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: tick_marks::Group::subdivided(
                1,
                1,
                1,
                Some(tick_marks::Tier::Two),
            ),

            int_tick_marks: tick_marks::Group::evenly_spaced(
                6,
                tick_marks::Tier::Two,
            ),

            db_tick_marks: vec![
                (db_range.to_normal(0.0), tick_marks::Tier::One),
                (db_range.to_normal(1.0), tick_marks::Tier::Two),
                (db_range.to_normal(3.0), tick_marks::Tier::Two),
                (db_range.to_normal(6.0), tick_marks::Tier::Two),
                (db_range.to_normal(12.0), tick_marks::Tier::Two),
                (db_range.to_normal(-1.0), tick_marks::Tier::Two),
                (db_range.to_normal(-3.0), tick_marks::Tier::Two),
                (db_range.to_normal(-6.0), tick_marks::Tier::Two),
                (db_range.to_normal(-12.0), tick_marks::Tier::Two),
            ]
            .into(),

            freq_tick_marks: vec![
                (freq_range.to_normal(20.0), tick_marks::Tier::Two),
                (freq_range.to_normal(50.0), tick_marks::Tier::Two),
                (freq_range.to_normal(100.0), tick_marks::Tier::One),
                (freq_range.to_normal(200.0), tick_marks::Tier::Two),
                (freq_range.to_normal(400.0), tick_marks::Tier::Two),
                (freq_range.to_normal(1000.0), tick_marks::Tier::One),
                (freq_range.to_normal(2000.0), tick_marks::Tier::Two),
                (freq_range.to_normal(5000.0), tick_marks::Tier::Two),
                (freq_range.to_normal(10000.0), tick_marks::Tier::One),
                (freq_range.to_normal(20000.0), tick_marks::Tier::Two),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center(
                "-1", "+1", "0",
            ),
            int_text_marks: text_marks::Group::evenly_spaced(&[
                "A", "B", "C", "D", "E", "F",
            ]),
            db_text_marks: text_marks::Group::min_max_and_center(
                "-12", "+12", "0",
            ),
            freq_text_marks: vec![
                (freq_range.to_normal(100.0), "100"),
                (freq_range.to_normal(1000.0), "1k"),
                (freq_range.to_normal(10000.0), "10k"),
            ].into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl HSliderStep {
    pub fn title(&self) -> &str {
        "Horizontal Sliders"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::HSliderMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    HSlidersID::Float => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_float_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::Int => {
                        // Integer parameters must be snapped for the widget to
                        // "step" when moved.
                        self.int_range.snap_normal(
                            &mut self.h_slider_int_state.param.normal,
                        );

                        self.output_text = crate::info_text_i32(
                            id,
                            self.int_range
                                .to_value(self.h_slider_int_state.param.normal),
                        );
                    }
                    HSlidersID::DB => {
                        self.output_text = crate::info_text_db(
                            id,
                            self.db_range
                                .to_value(self.h_slider_db_state.param.normal),
                        );
                    }
                    HSlidersID::Freq => {
                        self.output_text = crate::info_text_freq(
                            id,
                            self.freq_range.to_value(
                                self.h_slider_freq_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::RectStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_rect_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::BipolarRectStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_rect_bp_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::TextureStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_texture_state.param.normal,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float =
            HSlider::new(&mut self.h_slider_float_state, Message::HSliderMoved)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks);

        let h_slider_int =
            HSlider::new(&mut self.h_slider_int_state, Message::HSliderMoved)
                .tick_marks(&self.int_tick_marks)
                .text_marks(&self.int_text_marks);

        let h_slider_db =
            HSlider::new(&mut self.h_slider_db_state, Message::HSliderMoved)
                .tick_marks(&self.db_tick_marks)
                .text_marks(&self.db_text_marks);

        let h_slider_freq =
            HSlider::new(&mut self.h_slider_freq_state, Message::HSliderMoved)
                .tick_marks(&self.freq_tick_marks)
                .text_marks(&self.freq_text_marks);

        let h_slider_rect =
            HSlider::new(&mut self.h_slider_rect_state, Message::HSliderMoved)
                .height(Length::from(Length::Units(24)))
                .style(style::HSliderRectStyle);

        let h_slider_rect_bp = HSlider::new(
            &mut self.h_slider_rect_bp_state,
            Message::HSliderMoved,
        )
        .height(Length::from(Length::Units(24)))
        .style(style::HSliderRectBipolarStyle);

        let h_slider_texture = HSlider::new(
            &mut self.h_slider_texture_state,
            Message::HSliderMoved,
        )
        .tick_marks(&self.float_tick_marks)
        .text_marks(&self.float_text_marks)
        // the height of the texture
        .height(Length::from(Length::Units(20)))
        .style(style::HSliderTextureStyle(
            // clone the handle to the loaded texture
            self.h_slider_texture_handle.clone(),
        ));

        // push the widgets into rows
        let h_slider_row = Row::new()
            .spacing(16)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Float Range"))
                    .push(h_slider_float)
                    .push(Text::new("Log DB Range"))
                    .push(h_slider_db)
                    .push(Text::new("Custom Style"))
                    .push(h_slider_rect)
                    .push(Text::new("Custom Texture Style"))
                    .push(h_slider_texture),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Int Range"))
                    .push(h_slider_int)
                    .push(Text::new("Freq Range"))
                    .push(h_slider_freq)
                    .push(Text::new("Custom Bipolar Style"))
                    .push(h_slider_rect_bp),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(h_slider_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Horizontal Sliders (HSlider)")
            .push(content)
            .into()
    }
}
