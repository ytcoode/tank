use iced::{executor, Application, Command, Element, Settings};
use map::Map;

mod map;
mod style;

pub fn start() {
    Editor::run(Settings::default());
}

struct Editor {
    map: Map,
}

impl Editor {
    fn new() -> Editor {
        Editor { map: Map::new() }
    }
}

#[derive(Debug)]
pub enum Msg {
    A,
}

impl Application for Editor {
    type Executor = executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Editor::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Game Editor")
    }

    fn update(&mut self, _message: Msg) -> Command<Msg> {
        Command::none()
    }

    fn view(&mut self) -> Element<Msg> {
        self.map.view()
    }
}
