use std::collections::{HashMap, HashSet};

fn main() {
    // parameters
    let desired_size: usize = 4; // TODO: input

    // init
    let mut ids: HashSet<u32> = HashSet::new();
    let mut bracket = Bracket {
        players: vec![],
        matches: vec![],
    };

    //players
    let player_names = get_players(); // TODO: input
    for name in player_names {
        bracket.players.push(Player {
            id: PlayerId(new_id(&mut ids)),
            name,
        });
    }

    // matches
    for group in bracket.players.chunks(desired_size) {
        let player_ids: Vec<PlayerId> = group.iter().map(|p| p.id).collect();
        let player_results: HashMap<PlayerId, PlayerResult> = player_ids
            .iter()
            .map(|p| (*p, PlayerResult::Unplayed))
            .collect::<Vec<(PlayerId, PlayerResult)>>()
            .into_iter()
            .collect();
        bracket.matches.push(Match {
            id: MatchId(new_id(&mut ids)),
            resulting_match: None, // TODO: does this make sense?
            players: player_ids,
            states: player_results,
        });
    }

    // display
    // TODO:
}

fn new_id(ids: &mut HashSet<u32>) -> u32 {
    if ids.len() >= std::u32::MAX as usize {
        panic!("Too many items for id gen.");
    }
    loop {
        let x: u32 = rand::random();
        if !ids.contains(&x) {
            ids.insert(x);
            return x;
        }
    }
}

struct Bracket {
    players: Vec<Player>,
    matches: Vec<Match>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PlayerId(u32);
struct Player {
    id: PlayerId,
    name: String,
}

enum PlayerResult {
    Unplayed,
    Pass,
    Fail,
}

#[derive(Copy, Clone)]
struct MatchId(u32);
struct Match {
    id: MatchId,
    resulting_match: Option<MatchId>, // None = final
    players: Vec<PlayerId>,
    states: HashMap<PlayerId, PlayerResult>,
}

fn get_players() -> Vec<String> {
    vec![
        "Thomas".to_owned(),
        "Syd".to_owned(),
        "Ben".to_owned(),
        "Atharv".to_owned(),
        "Tilford".to_owned(),
        "Lucas".to_owned(),
        "Ethan".to_owned(),
        "Natalie".to_owned(),
        "A".to_owned(),
        "B".to_owned(),
        "C".to_owned(),
        "D".to_owned(),
        "E".to_owned(),
        "F".to_owned(),
        "G".to_owned(),
        "H".to_owned(),
        "I".to_owned(),
        "J".to_owned(),
    ]
}
