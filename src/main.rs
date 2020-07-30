// #![feature(option_unwrap_none)]
#![feature(option_expect_none)]

use self::game::Game;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

mod game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("CrazyTank", "wangyuntao")
        .window_setup(WindowSetup::default().title("Crazy Tank"))
        .window_mode(WindowMode::default().dimensions(1000.0, 600.0))
        .add_resource_path("./assets/resources")
        .build()?;

    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}
