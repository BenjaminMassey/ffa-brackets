use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayerId(pub u32);
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Clone)]
pub enum PlayerResult {
    Unplayed,
    Pass,
    Fail,
}
impl std::fmt::Display for PlayerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            PlayerResult::Unplayed => "U",
            PlayerResult::Pass => "P",
            PlayerResult::Fail => "F",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MatchId(pub u32);
#[derive(Clone)]
pub struct Match {
    pub id: MatchId,
    pub resulting_match: Option<MatchId>,
    pub players: Vec<PlayerId>,
    pub states: HashMap<PlayerId, PlayerResult>,
}
