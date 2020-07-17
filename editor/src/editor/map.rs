use self::grid::Grid;
use crate::editor::style;
use crate::editor::Msg;
use iced::Column;
use iced::Container;
use iced::Element;
use iced::Length;

mod grid;

pub struct Map {
    grid: Grid,
}

impl Map {
    pub fn new() -> Map {
        let grid = Grid::new(3, 3);

        Map { grid }
    }

    pub fn view(&mut self) -> Element<'_, Msg> {
        let c = Column::new().push(self.grid.view());

        // TODO controls

        Container::new(c)
            // .width(Length::Fill)
            // .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}
