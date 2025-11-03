use std::collections::{HashMap, HashSet};

fn main() {
    // init
    let mut ids: HashSet<u32> = HashSet::new();
    let mut bracket = Bracket {
        desired_size: 4, // TODO: input
        players: HashMap::new(),
        matches: HashMap::new(),
    };

    //players
    let player_names = get_players(); // TODO: input
    for name in player_names {
        let id = PlayerId(new_id(&mut ids));
        bracket.players.insert(id, Player { id, name });
    }

    // matches
    let groups = bracket
        .players
        .keys()
        .map(|k| *k)
        .collect::<Vec<PlayerId>>();
    let mut chunked_groups = groups.chunks_exact(bracket.desired_size);
    for group in chunked_groups.by_ref() {
        bracket.add_group(group, &mut ids);
    }
    let mut problem_group = chunked_groups.remainder().to_vec();
    bracket.fill_group(&mut problem_group);
    bracket.add_group(&problem_group, &mut ids);

    // display
    bracket.display();
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
    desired_size: usize,
    players: HashMap<PlayerId, Player>,
    matches: HashMap<MatchId, Match>,
}
impl Bracket {
    fn display(&self) {
        for (mid, m) in &self.matches {
            println!("==============");
            for p in &m.players {
                println!("{} [{}]", self.players[p].name, m.states[p]);
            }
            println!("==============\n");
        }
    }
    fn add_group(&mut self, group: &[PlayerId], ids: &mut HashSet<u32>) {
        let player_results: HashMap<PlayerId, PlayerResult> = group
            .iter()
            .map(|p| (*p, PlayerResult::Unplayed))
            .collect::<Vec<(PlayerId, PlayerResult)>>()
            .into_iter()
            .collect();
        let id = MatchId(new_id(ids));
        self.matches.insert(
            id,
            Match {
                id,
                resulting_match: None, // TODO: does this make sense?
                players: group.to_vec(),
                states: player_results,
            },
        );
    }
    fn fill_group(&mut self, problem_group: &mut Vec<PlayerId>) {
        let mut fudged_indices: HashSet<usize> = HashSet::new();
        for _ in 0..(self.desired_size - problem_group.len()) {
            let mut index_to_fudge: usize = 0;
            loop {
                let i: usize = rand::random_range(0..self.matches.len());
                if !fudged_indices.contains(&i) {
                    fudged_indices.insert(i);
                    index_to_fudge = i;
                    break;
                }
            }
            let key = self.matches.clone().into_keys().collect::<Vec<MatchId>>()[index_to_fudge];
            let stolen_id = self.matches.get_mut(&key).unwrap().players.remove(0);
            problem_group.push(stolen_id);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PlayerId(u32);
struct Player {
    id: PlayerId,
    name: String,
}

#[derive(Clone)]
enum PlayerResult {
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
struct MatchId(u32);
#[derive(Clone)]
struct Match {
    id: MatchId,
    resulting_match: Option<MatchId>,
    players: Vec<PlayerId>,
    states: HashMap<PlayerId, PlayerResult>,
}

// TODO: real input system
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
