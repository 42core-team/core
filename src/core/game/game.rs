use std::time::Duration;

use tokio::{net::TcpListener, sync::mpsc};

use crate::game::action::Action;

use super::{
    helper::Target, utils::get_ms, Core, GameConfig, Message, Resource, Spectator, State, Team,
    Unit,
};

#[derive(Debug)]
pub struct Game {
    pub status: u64,
    pub teams: Vec<Team>,
    pub config: GameConfig,
    pub resources: Vec<Resource>,
    pub cores: Vec<Core>,
    pub units: Vec<Unit>,
    targets: Vec<(u64, u64)>,
    pub tick_rate: u128,
    pub last_tick_time: u128,
    pub time_since_last_tick: u128,

    pub spectators: Vec<Spectator>,
    pub required_team_ids: Vec<u64>,
}

impl Game {
    pub fn new(required_team_ids: Vec<u64>) -> Self {
        Game {
            status: 0, // OK
            teams: vec![],
            config: GameConfig::patch_0_1_0(),
            cores: vec![Core::new(1, 2000, 2000), Core::new(2, 4000, 4000)],
            resources: vec![],
            units: vec![],
            targets: vec![],
            tick_rate: 50,
            last_tick_time: get_ms(),
            time_since_last_tick: 0,

            spectators: vec![],
            required_team_ids,
        }
    }

    pub async fn init(mut self) {
        let (team_sender, mut team_receiver) = mpsc::channel::<Team>(100);
        let (spectator_sender, mut spectator_receiver) = mpsc::channel::<Spectator>(100);

        Self::open(team_sender, spectator_sender);

        loop {
            if let Ok(team) = team_receiver.try_recv() {
                if self.required_team_ids.contains(&team.id)
                    && !self.teams.iter().any(|iter_team| iter_team.id == team.id)
                {
                    self.teams.push(team);
                    println!("Team received");
                } else {
                    println!("Team not allowed");
                }
            }
            if let Ok(spectator) = spectator_receiver.try_recv() {
                println!("Spectator received");
                self.spectators.push(spectator);
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
            println!("Teams: {:?}", self.teams.len());
            println!("Required: {:?}", self.required_team_ids.len());
            if self.teams.len() == self.required_team_ids.len() {
                break;
            }
        }
        self.start(spectator_receiver).await;
    }

    pub fn open(team_sender: mpsc::Sender<Team>, spectator_sender: mpsc::Sender<Spectator>) {
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                let mut team = Team::from_tcp_stream(stream);

                if let Some(message) = team.receiver.as_mut().unwrap().recv().await {
                    match message {
                        Message::Login(login) => {
                            if login.id == 42 {
                                let _ = spectator_sender.send(Spectator::from_team(team)).await;
                            } else {
                                team.id = login.id;
                                let _ = team_sender.send(team).await;
                            }
                        }
                        _ => {
                            println!("Error: First message is not a login message");
                        }
                    }
                }
            }
        });
    }

    pub async fn start(&mut self, mut spectator_receiver: mpsc::Receiver<Spectator>) {
        for team_index in 0..self.teams.len() {
            let team = &mut self.teams[team_index];
            match team
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_game_config(&GameConfig::patch_0_1_0()))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    println!("Error sending config to team");
                }
            }
        }

        loop {
            if let Ok(spectator) = spectator_receiver.try_recv() {
                println!("Spectator received");
                self.spectators.push(spectator);
            }

            if self.tick().await {
                break;
            }
        }
        self.status = 2; // END
        self.send_state().await;
    }

    async fn tick(&mut self) -> bool {
        for team in self.teams.iter_mut() {
            if team.is_disconnected() {
                println!("Team {:?} disconnected", team.id);
                return true;
            }
        }
        println!("------ Tick ------");
        self.wait_till_next_tick().await;

        let mut team_actions: Vec<(u64, Action)> = vec![];

        for team_index in 0..self.teams.len() {
            let team = &mut self.teams[team_index];
            while let Ok(message) = team.receiver.as_mut().unwrap().try_recv() {
                match message {
                    Message::VecAction(actions) => {
                        println!("TEAM send action: {:?}", actions);
                        for action in actions {
                            team_actions.push((team.id, action));
                        }
                    }
                    _ => {
                        println!("TEAM received unknown message");
                    }
                }
            }
        }
        self.update(team_actions);
        self.send_state().await;
        false
    }

    async fn send_state(&mut self) {
        let state = State::from_game(self);
        for team in self.teams.iter_mut() {
            let state = state.clone();
            match team
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_state(&state))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    println!("Error sending state to team");
                }
            }
        }
        for spectator in self.spectators.iter_mut() {
            let state = state.clone();
            match spectator
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_state(&state))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    println!("Error sending state to spectator");
                }
            }
        }
    }

    pub async fn wait_till_next_tick(&mut self) {
        let min_ms_per_tick: u128 = self.tick_rate;

        loop {
            // This is so that it always takes 1ms steps minimum
            if get_ms() <= self.last_tick_time {
                tokio::time::sleep(Duration::from_millis(1)).await;
                continue;
            }

            self.time_since_last_tick = get_ms() - self.last_tick_time;

            if self.time_since_last_tick > min_ms_per_tick {
                self.last_tick_time = self.last_tick_time + self.time_since_last_tick;
                break;
            }

            tokio::time::sleep(Duration::from_millis(((min_ms_per_tick / 2) + 1) as u64)).await;
        }
    }

    pub fn generate_u64_id() -> u64 {
        static mut COUNTER: u64 = 1;

        unsafe {
            COUNTER += 1;
            COUNTER
        }
    }

    pub fn get_team_by_id(&self, id: u64) -> Option<&Team> {
        for team in self.teams.iter() {
            if team.id == id {
                return Some(team);
            }
        }

        None
    }

    pub fn get_team_by_id_mut(&mut self, team_id: u64) -> Option<&mut Team> {
        self.teams.iter_mut().find(|team| team.id == team_id)
    }

    pub fn get_unit_by_id(&self, id: u64) -> Option<&Unit> {
        for unit in self.units.iter() {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn get_unit_by_id_mut(&mut self, id: u64) -> Option<&mut Unit> {
        self.units.iter_mut().find(|unit| unit.id == id)
    }

    pub fn get_resource_by_id(&self, id: u64) -> Option<&Resource> {
        for resource in self.resources.iter() {
            if resource.id == id {
                return Some(resource);
            }
        }
        None
    }

    pub fn get_resource_by_id_mut(&mut self, id: u64) -> Option<&mut Resource> {
        self.resources.iter_mut().find(|resource| resource.id == id)
    }

    pub fn get_core_by_id(&self, id: u64) -> Option<&Core> {
        for core in self.cores.iter() {
            if core.id == id {
                return Some(core);
            }
        }
        None
    }

    pub fn get_core_by_id_mut(&mut self, id: u64) -> Option<&mut Core> {
        self.cores.iter_mut().find(|core| core.id == id)
    }

    pub fn get_core_by_team_id(&self, team_id: u64) -> Option<&Core> {
        for core in self.cores.iter() {
            println!("Core: {:?}", core);
            if core.team_id == team_id {
                return Some(core);
            }
        }
        None
    }

    ///
    /// Function to create a new unit
    ///
    /// Security:
    /// - check if team exists
    /// - check if unit type exists
    /// - check if team has enough balance
    ///
    /// Features:
    /// - create unit
    /// - reduce team balance
    ///
    pub fn create_unit(&mut self, team_id: u64, type_id: u64) {
        println!(
            "Create unit of type {:?} for team with id {:?}",
            type_id, team_id
        );
        let team_core = self.get_core_by_team_id(team_id);
        if team_core.is_none() {
            println!("Core of team with id {:?} not found", team_id);
            return;
        }
        let team_core = team_core.unwrap();
        let unit = Unit::new(self, team_id, type_id, team_core.x, team_core.y);
        match unit {
            Some(unit) => {
                let team_balance = self.get_team_by_id(team_id).unwrap().balance;
                let unit_cost = GameConfig::get_unit_config_by_type_id(type_id)
                    .unwrap()
                    .cost;
                if team_balance < unit_cost {
                    println!("Team with id {:?} has not enough balance", team_id);
                    return;
                }
                let team = self.get_team_by_id_mut(team_id);
                match team {
                    Some(team) => {
                        team.balance -= unit_cost;
                    }
                    None => {
                        println!("Team with id {:?} not found", team_id);
                        return;
                    }
                }
                self.units.push(unit);
            }
            None => {
                println!("Unit could not be created");
            }
        }
    }

    ///
    /// Handel the attack action
    ///
    /// Security:
    /// - check if attacker exists
    /// - check if target exists
    /// - check if attacker is in own team
    ///
    /// if target is equal to attacker:
    /// - remove target from targets
    ///
    pub fn handel_attack_action(&mut self, attacker_id: u64, target_id: u64, team_id: u64) {
        println!("Attack: {:?} -> {:?}", attacker_id, target_id);
        let attacker = self.units.iter().find(|unit| unit.id == attacker_id);
        let target = self.units.iter().find(|unit| unit.id == target_id);
        match (attacker, target) {
            (Some(attacker), Some(_)) => {
                if attacker.team_id == team_id {
                    if attacker_id == target_id {
                        self.targets.retain(|target| target.0 != attacker_id);
                    } else if target_id != team_id {
                        self.targets.push((attacker_id, target_id));
                    }
                }
            }
            _ => {
                println!("Attacker or target not found");
            }
        }
    }

    ///
    /// Find a target by id
    ///
    /// Security:
    /// - check if target exists
    ///
    /// Features:
    /// - return target in the following types:
    /// 	- Unit
    /// 	- Resource
    /// 	- Core
    /// 	- None
    ///
    pub fn get_target_by_id(&self, id: u64) -> Target {
        let unit = self.units.iter().find(|unit| unit.id == id);
        let resource = self.resources.iter().find(|resource| resource.id == id);
        let core = self.cores.iter().find(|core| core.id == id);
        match (unit, resource, core) {
            (Some(unit), _, _) => Target::Unit(unit.clone()),
            (_, Some(resource), _) => Target::Resource(resource.clone()),
            (_, _, Some(core)) => Target::Core(core.clone()),
            _ => Target::None,
        }
    }

    pub fn get_dist(&self, x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
        let xdif;
        let ydif;
        if x1 > x2 {
            xdif = x1 - x2;
        } else {
            xdif = x2 - x1;
        }
        if y1 > y2 {
            ydif = y1 - y2;
        } else {
            ydif = y2 - y1;
        }
        (((xdif).pow(2) + (ydif).pow(2)) as f64).sqrt() as u64
    }

    pub fn is_target_in_range(&self, attacker_id: u64, target: &Target) -> bool {
        if let Some(attacker) = self
            .units
            .iter()
            .find(|unit| unit.id == attacker_id)
            .cloned()
        {
            match target {
                Target::Unit(target) => {
                    let dist = self.get_dist(attacker.x, attacker.y, target.x, target.y);
                    let max_range = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                        .map(|config| config.max_range)
                        .unwrap_or_default();
                    return dist <= max_range;
                }
                Target::Resource(target) => {
                    let dist = self.get_dist(attacker.x, attacker.y, target.x, target.y);
                    let max_range = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                        .map(|config| config.max_range)
                        .unwrap_or_default();
                    return dist <= max_range;
                }
                Target::Core(target) => {
                    let dist = self.get_dist(attacker.x, attacker.y, target.x, target.y);
                    let max_range = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                        .map(|config| config.max_range)
                        .unwrap_or_default();
                    return dist <= max_range;
                }
                Target::None => {
                    return false;
                }
            }
        }
        false
    }

    ///
    /// Fulfill the attack action
    ///
    /// Security:
    /// - check if attacker exists
    /// - check if target exists
    ///
    /// Features:
    /// - attack target
    /// - calculate damage per tick
    ///
    /// Get the damage of the attacker based on the type of the target from the config
    ///
    pub fn attack(&mut self, attacker_id: u64, target_id: u64) {
        println!("Attack: {:?} -> {:?}", attacker_id, target_id);
        let attacker = self
            .units
            .iter()
            .find(|unit| unit.id == attacker_id)
            .cloned();
        let target = self.get_target_by_id(target_id);
        match (attacker, target) {
            (Some(attacker), target @ Target::Unit(_))
            | (Some(attacker), target @ Target::Resource(_))
            | (Some(attacker), target @ Target::Core(_)) => {
                if self.is_target_in_range(attacker_id, &target) {
                    match target {
                        Target::Unit(unit) => {
                            let damage = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                                .unwrap()
                                .dmg_unit;
                            self.get_unit_by_id_mut(unit.id).unwrap().hp -=
                                (damage / (1000 / self.tick_rate as u64)) as u64;
                            if self.get_unit_by_id_mut(unit.id).unwrap().hp <= 0 {
                                self.units.retain(|unit| unit.id != target_id);
                            }
                        }
                        Target::Resource(resource) => {
                            let damage = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                                .unwrap()
                                .dmg_resource;
                            self.get_resource_by_id_mut(resource.id).unwrap().hp -=
                                (damage / (1000 / self.tick_rate as u64)) as u64;
                            if self.get_resource_by_id_mut(resource.id).unwrap().hp <= 0 {
                                self.resources.retain(|resource| resource.id != target_id);
                            }
                        }
                        Target::Core(core) => {
                            let damage = GameConfig::get_unit_config_by_type_id(attacker.type_id)
                                .unwrap()
                                .dmg_core;
                            self.get_core_by_id_mut(core.id).unwrap().hp -=
                                (damage / (1000 / self.tick_rate as u64)) as u64;
                            if self.get_core_by_id_mut(core.id).unwrap().hp <= 0 {
                                self.cores.retain(|core| core.id != target_id);
                            }
                        }
                        _ => {
                            // Handle other cases if needed
                        }
                    }
                } else {
                    println!("Target not in range");
                }
            }
            _ => {
                println!("Attacker or target not found");
            }
        }
    }

    ///
    /// Handel the update of the game
    ///
    /// a valid json to send with netcat is:
    /// [{"Create":{"type_id":3}},{"Travel":{"id":1,"x":2,"y":3}},{"Attack":{"attacker_id":1,"target_id":2}}]
    /// [{"Create":{"type_id":1}}]
    /// [{"Attack":{"attacker_id":6,"target_id":6}}]
    ///
    /// {"actions":[{"Create":{"type_id":0}}]}
    /// {"actions":[{"Create":{"type_id":0}},{"Travel":{"id":1,"x":2,"y":3}},{"Attack":{"attacker_id":1,"target_id":2}}]}
    /// {"id": 10}
    /// {"id": 20}
    /// {"id": 42}
    ///
    /// To uns netcat:
    /// ```sh
    /// nc localhost 4242
    /// ```
    /// then paste the json and press enter
    ///
    /// You need at least two netcat instances to start a game
    ///
    pub fn update(&mut self, team_actions: Vec<(u64, Action)>) {
        for (team_id, action) in team_actions {
            match action {
                Action::Create(create) => {
                    self.create_unit(team_id, create.type_id);
                }
                Action::Attack(attack) => {
                    self.handel_attack_action(attack.attacker_id, attack.target_id, team_id);
                }
                Action::Travel(travel) => {
                    println!("Travel: {:?}", travel);
                }
            }
        }
        let targets: Vec<_> = self.targets.iter().cloned().collect();
        for (attacker_id, target_id) in targets {
            let attacker = self
                .units
                .iter()
                .find(|unit| unit.id == attacker_id)
                .cloned();
            let target = self.get_target_by_id(target_id.clone());
            match (attacker, target) {
                (Some(attacker), Target::Unit(target)) => {
                    if attacker.team_id != target.team_id {
                        self.attack(attacker.id, target.id);
                    }
                }
                (Some(attacker), Target::Resource(target)) => {
                    self.attack(attacker.id, target.id);
                }
                (Some(attacker), Target::Core(target)) => {
                    if attacker.team_id != target.team_id {
                        self.attack(attacker.id, target.id);
                    }
                }
                _ => {
                    println!("Attacker or target not found");
                }
            }
        }
    }

    pub fn create_fake_unit(&mut self, team_id: u64, type_id: u64, x: u64, y: u64) {
        let unit = Unit::new(self, team_id, type_id, x, y);
        match unit {
            Some(unit) => {
                self.units.push(unit);
            }
            None => {
                println!("Unit could not be created");
            }
        }
    }

    pub fn create_fake_resource(&mut self, x: u64, y: u64) {
        let resource = Resource::new(0, 100, x, y, 100);
        self.resources.push(resource);
    }

    pub fn create_fake_core(&mut self, team_id: u64, x: u64, y: u64) {
        let core = Core::new(team_id, x, y);
        self.cores.push(core);
    }
}
