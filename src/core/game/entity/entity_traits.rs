use crate::game::Position;

pub trait Entity {
    fn id(&self) -> u64;
    fn pos(&self) -> &Position;
    fn hp(&self) -> u64;
}

pub trait EntityTeam {
    fn team_id(&self) -> u64;
}
