pub struct GameConfig {
    height: u64,
    width: u64,
    idle_income: u64,
    core_hp: u64,
    units: Vec<UnitConfig>,
}

impl GameConfig {
    pub fn patch_0_1_0() -> Self {
        GameConfig {
            height: 10000,
            width: 10000,
            idle_income: 25,
            core_hp: 5000,
            units: vec![
                UnitConfig {
                    name: String::from("Warrior"),
                    type_id: 1,
                    cost: 100,
                    hp: 3000,
                    dmg_core: 2000,
                    dmg_unit: 1500,
                    dmg_resource: 500,
                    max_range: 1000,
                    min_range: 0,
                    speed: 1000,
                },
                UnitConfig {
                    name: String::from("Worker"),
                    type_id: 2,
                    cost: 50,
                    hp: 1500,
                    dmg_core: 500,
                    dmg_unit: 250,
                    dmg_resource: 2000,
                    max_range: 200,
                    min_range: 0,
                    speed: 2000,
                },
            ],
        }
    }
}
pub struct UnitConfig {
    name: String,
    type_id: u64,
    cost: u64,
    hp: u64,
    dmg_core: u64,
    dmg_unit: u64,
    dmg_resource: u64,
    max_range: u64,
    min_range: u64,
    speed: u64,
}
