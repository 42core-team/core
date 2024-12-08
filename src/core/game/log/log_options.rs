#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum LogOptions {
    State,
    Error,
    Info,
    Changes,
    Action,
}
