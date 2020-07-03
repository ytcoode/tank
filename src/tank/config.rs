use crate::config;
use crate::config::Config;
use std::collections::HashMap;
use std::fmt;
use std::io;

pub struct TankCfgs {
    map: HashMap<u32, TankCfg>,
}

struct TankCfg {
    id: u32,
}

impl TankCfg {
    fn new<C: Config>(c: C) -> TankCfg {
        let id = c.get("id").unwrap_or("0");
        let id = id
            .parse()
            .expect(format!("Failed to parse {} as tank id", id).as_str());
        TankCfg { id }
    }
}

impl fmt::Debug for TankCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TankCfg").field("id", &self.id).finish()
    }
}

pub fn load_cfgs() -> io::Result<TankCfgs> {
    let mut map = HashMap::new();

    config::load("./config/tank.txt")
        .into_iter()
        .map(|c| TankCfg::new(c))
        .for_each(|t| map.insert(t.id, t).expect_none("Duplicate tank id"));

    Ok(TankCfgs { map })
}
