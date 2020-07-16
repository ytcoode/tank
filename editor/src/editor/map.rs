use crate::editor::style;
use crate::editor::Msg;
use grid::Grid;
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
        let grid = Grid::new(10, 20);

        Map { grid }
    }

    pub fn view(&mut self) -> Element<'_, Msg> {
        let c = Column::new().push(self.grid.view());

        Container::new(c)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}
