use crate::tank;
use std::io;

pub struct GameCfgs {
    tanks: tank::TankCfgs,
}

pub fn load_cfgs() -> io::Result<GameCfgs> {
    let tanks = tank::load_cfgs()?;
    Ok(GameCfgs { tanks })
}
