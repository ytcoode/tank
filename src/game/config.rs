use crate::tank;
use ggez::Context;

pub struct GameCfgs {
    _tanks: tank::TankCfgs,
}

pub fn load_cfgs(ctx: &mut Context) -> GameCfgs {
    let _tanks = tank::load_cfgs(ctx);
    GameCfgs { _tanks }
}
