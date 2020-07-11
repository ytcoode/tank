use crate::game::scene::SceneCfgs;
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
        let scenes = SceneCfgs::load();
        let tanks = TankCfgs::load(ctx);
        let misc = MiscCfgs::load(ctx);

        GameCfgs {
            scenes,
            tanks,
            misc,
        }
    }
}
