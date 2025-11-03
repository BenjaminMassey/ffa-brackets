use std::collections::{HashMap, HashSet};

use crate::data;

pub struct Bracket {
    pub ids: HashSet<u32>,
    pub desired_size: usize,
    pub players: HashMap<data::PlayerId, data::Player>,
    pub matches: HashMap<data::MatchId, data::Match>,
}
impl Bracket {
    pub fn new(desired_size: usize) -> Self {
        Self {
            ids: HashSet::new(),
            desired_size,
            players: HashMap::new(),
            matches: HashMap::new(),
        }
    }
    pub fn display(&self) {
        for (mid, m) in &self.matches {
            println!("==============");
            for p in &m.players {
                println!("{} [{}]", self.players[p].name, m.states[p]);
            }
            println!("==============\n");
        }
    }
    pub fn new_id(&mut self) -> u32 {
        if self.ids.len() >= std::u32::MAX as usize {
            panic!("Too many items for id gen.");
        }
        loop {
            let x: u32 = rand::random();
            if !self.ids.contains(&x) {
                self.ids.insert(x);
                return x;
            }
        }
    }
    pub fn make_groups(&mut self) {
        let groups = self
            .players
            .keys()
            .map(|k| *k)
            .collect::<Vec<data::PlayerId>>();
        let mut chunked_groups = groups.chunks_exact(self.desired_size);
        for group in chunked_groups.by_ref() {
            self.add_group(group);
        }
        let mut problem_group = chunked_groups.remainder().to_vec();
        self.fill_group(&mut problem_group);
        self.add_group(&problem_group);
    }
    pub fn add_group(&mut self, group: &[data::PlayerId]) {
        let player_results: HashMap<data::PlayerId, data::PlayerResult> = group
            .iter()
            .map(|p| (*p, data::PlayerResult::Unplayed))
            .collect::<Vec<(data::PlayerId, data::PlayerResult)>>()
            .into_iter()
            .collect();
        let id = data::MatchId(self.new_id());
        self.matches.insert(
            id,
            data::Match {
                id,
                resulting_match: None, // TODO: does this make sense?
                players: group.to_vec(),
                states: player_results,
            },
        );
    }
    pub fn fill_group(&mut self, problem_group: &mut Vec<data::PlayerId>) {
        let mut fudged_indices: HashSet<usize> = HashSet::new();
        for _ in 0..(self.desired_size - problem_group.len()) {
            let index_to_fudge;
            loop {
                let i: usize = rand::random_range(0..self.matches.len());
                if !fudged_indices.contains(&i) {
                    fudged_indices.insert(i);
                    index_to_fudge = i;
                    break;
                }
            }
            let key = self
                .matches
                .clone()
                .into_keys()
                .collect::<Vec<data::MatchId>>()[index_to_fudge];
            let stolen_id = self.matches.get_mut(&key).unwrap().players.remove(0);
            problem_group.push(stolen_id);
        }
    }
}
