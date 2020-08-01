use self::misc::MiscCfgs;
use crate::game::scene::SceneCfgs;
use crate::game::tank::TankCfgs;
use ggez::Context;

pub mod misc;

pub struct GameCfgs {
    pub scenes: SceneCfgs,
    pub tanks: TankCfgs,
    pub misc: MiscCfgs,
}

impl GameCfgs {
    pub fn load(ctx: &mut Context) -> GameCfgs {
        let tanks = TankCfgs::load(ctx);
        let scenes = SceneCfgs::load(ctx, &tanks);
        let misc = MiscCfgs::load(ctx);

        GameCfgs {
            scenes,
            tanks,
            misc,
        }
    }
}
