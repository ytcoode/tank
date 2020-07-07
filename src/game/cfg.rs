use crate::tank;
use ggez::Context;

pub struct GameCfgs {
    pub tanks: tank::TankCfgs,
}

pub fn load(ctx: &mut Context) -> GameCfgs {
    let tanks = tank::load_cfgs(ctx);
    GameCfgs { tanks }
}
