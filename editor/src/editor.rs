use crate::style;
use iced::{executor, Application, Command, Container, Element, Length, Settings, Text};

pub fn start() {
    Editor::run(Settings::default());
}

struct Editor;

#[derive(Debug)]
enum Msg {
    A,
}

impl Application for Editor {
    type Executor = executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Editor, Command::none())
    }

    fn title(&self) -> String {
        String::from("Map Editor")
    }

    fn update(&mut self, _message: Msg) -> Command<Msg> {
        Command::none()
    }

    fn view(&mut self) -> Element<Msg> {
        let t = Text::new("Hello, world!");

        Container::new(t)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}
