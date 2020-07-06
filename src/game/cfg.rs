use crate::tank;
use ggez::Context;

pub struct GameCfgs {
    tanks: tank::TankCfgs,
}

pub fn load_cfgs(ctx: &mut Context) -> GameCfgs {
    let tanks = tank::load_cfgs(ctx);
    GameCfgs { tanks }
}
