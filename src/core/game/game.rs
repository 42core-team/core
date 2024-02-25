use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use tokio::{net::TcpListener, sync::mpsc};

use crate::game::action::Action;
use crate::game::log::log;
use crate::game::Spectator;

use super::action::Travel;
use super::bridge_con::BridgeCon;
use super::config::GameConfigWithId;
use super::entity::Unit;
use super::helper::Dmg;
use super::{generate, passive_income, Entity, Position};
use super::{helper::Target, utils::get_ms, Core, GameConfig, Message, Resource, State, Team};

#[derive(Debug)]
pub struct Game {
    pub status: u64,
    pub teams: Vec<Team>,
    pub config: GameConfig,
    pub resources: Vec<Resource>,
    pub cores: Vec<Core>,
    pub units: Vec<Unit>,
    pub tick_rate: u128,
    pub last_tick_time: u128,
    pub tick_calculation_time: u128,
    pub time_since_last_tick: u128,
    game_id_counter: Mutex<u64>,

    pub spectators: Vec<Spectator>,
    pub required_team_ids: Vec<u64>,
}

impl Game {
    pub fn new(required_team_ids: Vec<u64>) -> Self {
        let game_config: GameConfig = GameConfig::patch_0_1_0();

        Game {
            status: 0, // OK
            teams: vec![],
            cores: vec![],
            config: game_config,
            resources: vec![],
            units: vec![],
            tick_rate: 50,
            last_tick_time: get_ms(),
            tick_calculation_time: 0,
            time_since_last_tick: 0,
            game_id_counter: Mutex::new(0),

            spectators: vec![],
            required_team_ids,
        }
    }

    pub async fn init(mut self) {
        let (team_sender, mut team_receiver) = mpsc::channel::<BridgeCon>(100);
        let (spectator_sender, mut spectator_receiver) = mpsc::channel::<BridgeCon>(100);

        Self::open(team_sender, spectator_sender);

        loop {
            if let Ok(bridge_con) = team_receiver.try_recv() {
                let team = Team::new(&self, bridge_con);
                log::info(&format!("Team received: {:?}", team.con.id));
                if self.required_team_ids.contains(&team.con.id)
                    && !self
                        .teams
                        .iter()
                        .any(|iter_team| iter_team.con.id == team.con.id)
                {
                    log::info(&format!("Team id {:?} accepted", team.con.id));
                    self.teams.push(team);
                    log::info(&format!("Teams: {:?}", self.teams.len()));
                    log::info(&format!("Required: {:?}", self.required_team_ids.len()));
                } else {
                    log::error(&format!("Did not accept Team id {:?}", team.con.id));
                }
            }
            if let Ok(bridge_con) = spectator_receiver.try_recv() {
                let spectator = Spectator::new(bridge_con);
                log::info("Spectator received");
                self.spectators.push(spectator);
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
            if self.teams.len() == self.required_team_ids.len() {
                break;
            }
        }
        self.start(spectator_receiver).await;
    }

    pub fn open(team_sender: mpsc::Sender<BridgeCon>, spectator_sender: mpsc::Sender<BridgeCon>) {
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                let mut bridge_con = BridgeCon::new(stream);

                if let Some(message) = bridge_con.receiver.as_mut().unwrap().recv().await {
                    match message {
                        Message::Login(login) => {
                            bridge_con.id = login.id;
                            if login.id == 42 {
                                let _ = spectator_sender.send(bridge_con).await;
                            } else {
                                let _ = team_sender.send(bridge_con).await;
                            }
                        }
                        _ => {
                            log::error("First message is not a login message");
                        }
                    }
                }
            }
        });
    }

    pub async fn start(&mut self, mut spectator_receiver: mpsc::Receiver<BridgeCon>) {
        GameConfig::fill_team_config(&mut self.config, &self.teams);
        self.cores = generate::cores(self);
        self.resources = generate::resources(self);

        for team in self.teams.iter_mut() {
            match team
                .con
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_game_config(
                    &GameConfigWithId::from_game_config(&self.config, team.id),
                ))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    log::error("Error sending config to team");
                }
            }
        }
        for spectator in self.spectators.iter_mut() {
            match spectator
                .con
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_game_config(
                    &GameConfigWithId::from_game_config(&self.config, 42),
                ))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    log::error("Error sending config to spectator");
                }
            }
        }

        loop {
            if let Ok(bridge_con) = spectator_receiver.try_recv() {
                let spectator = Spectator::new(bridge_con);
                log::info("Spectator received");
                self.spectators.push(spectator);
                self.spectators
                    .last_mut()
                    .unwrap()
                    .con
                    .sender
                    .as_mut()
                    .unwrap()
                    .send(Message::from_game_config(
                        &GameConfigWithId::from_game_config(&self.config, 42),
                    ))
                    .await
                    .unwrap();
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
            if team.con.is_disconnected() {
                log::info(&format!("Team {:?} disconnected", team.id));
                return true;
            }
        }
        self.wait_till_next_tick().await;
        log::info(&format!(
            "Tick: {:?}, {:?}",
            self.time_since_last_tick, self.tick_calculation_time
        ));

        let mut team_actions: Vec<(u64, Action)> = vec![];

        for team in self.teams.iter_mut() {
            while let Ok(message) = team.con.receiver.as_mut().unwrap().try_recv() {
                match message {
                    Message::VecAction(actions) => {
                        log::action(&format!("TEAM {:?}: {:?}", team.id, actions));
                        for action in actions {
                            team_actions.push((team.id, action));
                        }
                    }
                    _ => {
                        log::error(&format!("TEAM {:?} received unknown message", team.id));
                    }
                }
            }
        }
        self.update(team_actions);
        if self.check_game_over() {
            return true;
        }

        passive_income::grant_passive_income(self);

        self.send_state().await;
        false
    }

    async fn send_state(&mut self) {
        let state = State::from_game(self);
        log::state(&serde_json::to_string(&state).unwrap());

        for team in self.teams.iter_mut() {
            match team
                .con
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_state(&state))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    log::error(&format!("Error sending state to team {:?}", team.id));
                }
            }
        }
        for spectator in self.spectators.iter_mut() {
            match spectator
                .con
                .sender
                .as_mut()
                .unwrap()
                .send(Message::from_state(&state))
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    log::error("Error sending state to spectator");
                }
            }
        }
    }

    pub async fn wait_till_next_tick(&mut self) {
        let new_tick_start_time = self.last_tick_time + self.tick_rate;
        self.tick_calculation_time = get_ms() - self.last_tick_time;

        if new_tick_start_time > get_ms() {
            tokio::time::sleep(Duration::from_millis(
                (new_tick_start_time - get_ms()) as u64,
            ))
            .await;
        }

        let current_millis = get_ms();
        self.time_since_last_tick = current_millis - self.last_tick_time;
        self.last_tick_time = current_millis;
    }

    pub fn check_game_over(&self) -> bool {
        self.cores.len() <= 1
    }

    pub fn generate_u64_id(&self) -> u64 {
        let mut counter = self.game_id_counter.lock().unwrap();
        *counter += 1;
        *counter
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
        log::changes(&format!(
            "Try to create unit of type {:?} for team with id {:?}",
            type_id, team_id
        ));

        // check if core exists
        let team_core = self.cores.iter().find(|core| core.team_id == team_id);
        if team_core.is_none() {
            log::error(&format!("Core of team with id {:?} not found", team_id));
            return;
        }

        // check if team has enough balance and subtract amount
        let unit_config = GameConfig::get_unit_config_by_type_id(&self.config, type_id);
        if unit_config.is_none() {
            log::error(&format!("Unit type with id {:?} not found", type_id));
            return;
        }
        let unit_cost = unit_config.unwrap().cost;
        let team = self.get_team_by_id_mut(team_id);
        match team {
            None => {
                log::error(&format!("Team with id {:?} not found", team_id));
                return;
            }
            Some(team) => {
                if team.balance < unit_cost {
                    log::error(&format!(
                        "Team with id {:?} has not enough balance",
                        team_id
                    ));
                    return;
                }
                team.balance -= unit_cost;
            }
        }

        let team_core = self
            .cores
            .iter()
            .find(|core| core.team_id == team_id)
            .unwrap();
        let unit = Unit::new(self, team_id, type_id, team_core.pos.clone());
        match unit {
            Some(unit) => {
                self.units.push(unit);
            }
            None => {
                log::error("Unit could not be created");
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
    pub fn handel_attack(&mut self, attacker_id: u64, target_id: u64, team_id: u64) {
        log::changes(&format!(
            "handel_attack_action: {:?} -> {:?} from team with id {:?}",
            attacker_id, target_id, team_id
        ));
        let target = self.get_target_by_id(target_id);
        if target.is_none() {
            log::error("Target not found");
            return;
        }
        let attacker = self.units.iter_mut().find(|unit| unit.id == attacker_id);
        if attacker.is_none() {
            log::error("Attacker not found");
            return;
        }
        attacker.unwrap().attack(target.unwrap());
    }

    pub fn deal_dmg(&mut self) {
        let mut dmg_to_deal: Vec<Dmg> = vec![];
        self.units.clone().iter().for_each(|unit| {
            if unit.target_id.is_none() {
                return;
            }
            let target = self.get_target_by_id(unit.target_id.unwrap());
            if target.is_none() {
                return;
            }
            let target = target.unwrap();

            let dmg = unit.calc_dmg(&self.config, &target, self.time_since_last_tick);
            if dmg > 0 {
                dmg_to_deal.push(Dmg::new(unit.id, target.id(), dmg));
            }
        });

        let mut ids_to_remove: Vec<u64> = vec![];
        let mut balance_to_add: HashMap<u64, u64> = HashMap::new();
        dmg_to_deal.iter().for_each(|dmg| {
            if let Some(unit) = self.units.iter_mut().find(|unit| unit.id == dmg.target_id) {
                if unit.deal_dmg(dmg.amount) {
                    ids_to_remove.push(unit.id);
                }
            }

            if let Some(resource) = self
                .resources
                .iter_mut()
                .find(|resource| resource.id == dmg.target_id)
            {
                let balance = resource.balance_from_dmg(&self.config, dmg.amount);
                balance_to_add
                    .entry(dmg.attacker_id)
                    .and_modify(|e| *e += balance)
                    .or_insert(balance);

                if resource.deal_dmg(dmg.amount) {
                    ids_to_remove.push(resource.id);
                }
            }

            if let Some(core) = self.cores.iter_mut().find(|core| core.id == dmg.target_id) {
                if core.deal_dmg(dmg.amount) {
                    ids_to_remove.push(core.id);
                }
            }
        });

        self.units.iter().for_each(|unit| {
            if let Some(balance) = balance_to_add.get(&unit.id) {
                if let Some(team) = self.teams.iter_mut().find(|team| team.id == unit.team_id) {
                    team.balance += *balance;
                }
            }
        });

        self.units.retain(|unit| !ids_to_remove.contains(&unit.id));
        self.resources
            .retain(|resource| !ids_to_remove.contains(&resource.id));
        self.cores.retain(|core| !ids_to_remove.contains(&core.id));
    }

    pub fn get_target_by_id(&self, id: u64) -> Option<Target> {
        let unit = self.units.iter().find(|unit: &&Unit| unit.id == id);
        if unit.is_some() {
            return Some(Target::Unit(unit.unwrap().clone()));
        }

        let resource = self
            .resources
            .iter()
            .find(|resource: &&Resource| resource.id == id);
        if resource.is_some() {
            return Some(Target::Resource(resource.unwrap().clone()));
        }

        let core = self.cores.iter().find(|core: &&Core| core.id == id);
        if core.is_some() {
            return Some(Target::Core(core.unwrap().clone()));
        }
        None
    }

    ///
    /// Handel the travel action
    ///
    /// Security:
    /// - check if unit exists
    /// - check if action is for the right team
    ///
    pub fn handel_travel(&mut self, team_id: u64, travel: Travel) {
        log::changes(&format!("Travel: {:?}", travel));
        let unit = self
            .units
            .iter_mut()
            .find(|unit: &&mut Unit| unit.id == travel.id);
        if unit.is_none() {
            log::error(&format!("Unit with id {:?} not found", travel.id));
            return;
        }
        let unit = unit.unwrap();
        if unit.team_id != team_id {
            log::error(&format!(
                "Team id {:?} for travel action for Unit id {:?} does not match",
                team_id, unit.id
            ));
        }
        unit.travel(&self.config, travel);
    }

    pub fn handel_travel_update(&mut self) {
        self.units.iter_mut().for_each(|unit| {
            unit.update_position(self.time_since_last_tick, &self.config);
        });
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
                    self.handel_attack(attack.attacker_id, attack.target_id, team_id);
                }
                Action::Travel(travel) => {
                    self.handel_travel(team_id, travel);
                }
            }
        }

        self.handel_travel_update();
        self.deal_dmg();
    }

    pub fn create_fake_unit(&mut self, team_id: u64, type_id: u64, pos: Position) {
        let unit = Unit::new(self, team_id, type_id, pos);
        match unit {
            Some(unit) => {
                self.units.push(unit);
            }
            None => {
                log::error("Unit could not be created");
            }
        }
    }

    pub fn create_fake_resource(&mut self, pos: Position) {
        let resource = Resource::new(self, 1, pos, 100);
        self.resources.push(resource);
    }

    pub fn create_fake_core(&mut self, team_id: u64, pos: Position, hp: u64) {
        let core = Core::new(self, team_id, pos, hp);
        self.cores.push(core);
    }
}
