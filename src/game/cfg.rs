use super::tank::TankCfgs;
use ggez::graphics::Image;
use ggez::Context;
use misc::MiscCfgs;

pub mod misc;

pub struct GameCfgs {
    pub tanks: TankCfgs,
    pub misc: MiscCfgs,
}

impl GameCfgs {
    pub fn load(ctx: &mut Context) -> GameCfgs {
        let tanks = TankCfgs::load(ctx);
        let misc = MiscCfgs::load(ctx);
        GameCfgs { tanks, misc }
    }
}
