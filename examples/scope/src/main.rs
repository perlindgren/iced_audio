// Import iced modules.
use iced::{
    Align, Column, Container, Element, Length, Sandbox, Settings, Text,
};
// Import iced_audio modules.
use iced_audio::{scope, tick_marks, FloatRange, Normal, Scope};

// The message when a parameter widget is moved by the user
#[derive(Debug, Clone)]
pub enum Message {
    XYPadFloat(Normal, Normal),
}

pub fn main() {
    App::run(Settings::default()).unwrap();
}

pub struct App {
    // The ranges handle converting the input/output of a parameter to and from
    // a usable value.
    //
    // There are 4 built-in options available for a range:
    //
    // * FloatRange - a linear range of f32 values
    float_range: FloatRange,

    // The states of the widgets that will control the parameters.
    xy_pad_state: scope::State,

    // A group of tick marks with their size and position.
    center_tick_mark: tick_marks::Group,
    output_text: String,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        // Initialize each range:
        let float_range = FloatRange::default_bipolar();

        App {
            // Add the ranges.
            float_range,

            // Initialize the state of the widgets with a normalized parameter
            // that has a value and a default value.
            xy_pad_state: scope::State::new(
                float_range.default_normal_param(),
                float_range.default_normal_param(),
            ),

            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),

            output_text: "Scope!".into(),
        }
    }

    fn title(&self) -> String {
        format!("Simple ]Scope - Iced Audio")
    }

    fn update(&mut self, event: Message) {
        match event {
            // Retrieve the value by mapping the normalized value of the parameter
            // to the corresponding range.
            //
            // Now do something useful with that value!
            Message::XYPadFloat(normal_x, normal_y) => {
                let value_x = self.float_range.unmap_to_value(normal_x);
                let value_y = self.float_range.unmap_to_value(normal_y);
                self.output_text =
                    format!("XYPadFloat: x: {:.2}, y: {:.2}", value_x, value_y);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Create each parameter widget, passing in the current state of the widget.

        let scope_widget =
            Scope::new(&mut self.xy_pad_state, Message::XYPadFloat);

        // Push the widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(300)
            .max_height(500)
            .spacing(20)
            .padding(20)
            .align_items(Align::Center)
            .push(scope_widget)
            .push(
                Container::new(Text::new(&self.output_text))
                    .width(Length::Fill),
            )
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
