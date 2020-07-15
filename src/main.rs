#![feature(option_unwrap_none)]
#![feature(option_expect_none)]

use ggez::event;
use ggez::ContextBuilder;

pub mod deps; // The 'pub' keyword is to suppress the dead code warnings
mod game;
pub mod util; // Same as above

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("CrazyTank", "wangyuntao")
        .add_resource_path("./assets/resources")
        .build()?;

    let mut game = game::Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}
