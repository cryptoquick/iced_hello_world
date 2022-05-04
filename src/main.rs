use iced::{
    alignment::{Horizontal, Vertical},
    executor, keyboard, window, Application, Color, Column, Command, Container, Element, Length,
    Row, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};

pub fn main() -> iced::Result {
    HelloWorld::run(Settings {
        window: window::Settings {
            always_on_top: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    DebugToggled,
}

struct HelloWorld {
    debug: bool,
}

impl Application for HelloWorld {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (HelloWorld { debug: false }, Command::none())
    }

    fn title(&self) -> String {
        "Iced Hello World Example".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        match event {
            Message::DebugToggled => {
                self.debug = !self.debug;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::KeyCode;

        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if let KeyCode::F12 = key_code {
                        Some(Message::DebugToggled)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        let HelloWorld { debug, .. } = self;

        let row = Row::new().push(
            Text::new("Hello World!")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        );

        let debug_state = if debug == &mut true { "off" } else { "on" };
        let debug_enabled = Row::new().push(
            Text::new(format!("Toggle debug mode {} with F12.", debug_state))
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        );

        let column = Column::new().push(row).push(debug_enabled);
        let content: Element<_> = column.into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}
