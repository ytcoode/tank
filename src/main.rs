#![feature(option_unwrap_none)]
#![feature(option_expect_none)]

use ggez::event;
use ggez::ContextBuilder;

mod game;
mod map;
mod position;
mod tank;

mod deps; // The 'pub' keyword is to suppress the dead code warnings
mod util; // Same as above

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("CrazyTank", "CrazyBunny")
        .add_resource_path("./resources")
        .build()?;

    let mut state = game::GameState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)?;

    Ok(())
}
