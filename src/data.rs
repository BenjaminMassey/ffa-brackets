use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerId(pub u32);
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerResult {
    Unplayed,
    Won,
    Lost,
}
impl std::fmt::Display for PlayerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            PlayerResult::Unplayed => "U",
            PlayerResult::Won => "W",
            PlayerResult::Lost => "L",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatchId(pub u32);
#[derive(Clone)]
pub struct Match {
    pub id: MatchId,
    pub connection: Option<MatchId>,
    pub players: Vec<PlayerId>,
    pub states: HashMap<PlayerId, PlayerResult>,
    pub finished: bool,
}
