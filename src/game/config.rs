use crate::game::scene::{map::MapCfgs, SceneCfgs};
use crate::game::tank::TankCfgs;
use ggez::graphics::Image;
use ggez::Context;
use misc::MiscCfgs;

pub mod misc;

pub struct GameCfgs {
    pub scenes: SceneCfgs,
    pub tanks: TankCfgs,
    pub misc: MiscCfgs,
}

impl GameCfgs {
    pub fn load(ctx: &mut Context) -> GameCfgs {
        let maps = MapCfgs::load();
        let scenes = SceneCfgs::load(&maps);
        let tanks = TankCfgs::load(ctx);
        let misc = MiscCfgs::load(ctx);

        GameCfgs {
            scenes,
            tanks,
            misc,
        }
    }
}
