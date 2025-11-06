use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use crate::data;

pub struct Bracket {
    pub ids: HashSet<u32>,
    pub desired_size: usize,
    pub players: HashMap<data::PlayerId, data::Player>,
    pub matches: HashMap<data::MatchId, data::Match>,
    pub rounds: Vec<Vec<data::MatchId>>,
}
impl Default for Bracket {
    fn default() -> Self {
        let mut bracket = Self {
            ids: HashSet::new(),
            desired_size: 4, // TODO: configurable
            players: HashMap::new(),
            matches: HashMap::new(),
            rounds: vec![],
        };
        bracket.make_players();
        bracket.make_groups();
        bracket.connect_matches();
        bracket
    }
}
impl Bracket {
    pub fn new_id(&mut self) -> u32 {
        if self.ids.len() >= std::u32::MAX as usize {
            panic!("Too many items for id gen.");
        }
        loop {
            let x: u32 = rand::random();
            if !self.ids.contains(&x) && x != 0 {
                self.ids.insert(x);
                return x;
            }
        }
    }
    pub fn make_players(&mut self) {
        let mut player_names = self.get_players(); // TODO: input
        player_names.shuffle(&mut rand::rng());
        for name in player_names {
            let id = data::PlayerId(self.new_id());
            self.players.insert(id, data::Player { id, name });
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
            self.add_group(group, 0);
        }
        let mut problem_group = chunked_groups.remainder().to_vec();
        self.fill_group(&mut problem_group);
        self.add_group(&problem_group, 0);
    }
    pub fn connect_matches(&mut self) {
        for round in &self.rounds {
            let unconnected = round
                .iter()
                .filter(|mid| mid != &&data::MatchId(0) && self.matches[mid].connection.is_none())
                .collect::<Vec<&data::MatchId>>();
            let mut pairs = unconnected.chunks_exact(2);
            for pair in pairs.by_ref() {
                (*self.matches.get_mut(&pair[0]).unwrap()).connection = Some(pair[1].clone());
                (*self.matches.get_mut(&pair[1]).unwrap()).connection = Some(pair[0].clone());
            }
        }
        // TODO: handle potential remainder match (cross-round D:)
    }
    pub fn add_group(&mut self, group: &[data::PlayerId], round: usize) {
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
                connection: None,
                players: group.to_vec(),
                states: player_results,
                finished: false,
                round,
            },
        );
        self.add_to_round(&id, round);
        self.connect_matches();
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
    pub fn add_to_round(&mut self, mid: &data::MatchId, round: usize) {
        while self.rounds.len() <= round {
            self.rounds.push(vec![]);
        }
        let index = self.get_index_for_round_insert(mid, round);
        if let Some(i) = index {
            while self.rounds[round].len() <= i {
                self.rounds.get_mut(round).unwrap().push(data::MatchId(0));
            }
            self.rounds.get_mut(round).unwrap()[i] = *mid;
        } else {
            self.rounds.get_mut(round).unwrap().push(*mid);
        }
    }
    fn get_index_for_round_insert(&self, mid: &data::MatchId, round: usize) -> Option<usize> {
        if round > 0 {
            for i in 0..self.rounds[round - 1].len() {
                let entry = &self.rounds[round - 1][i];
                for p0 in &self.matches[entry].players {
                    for p1 in &self.matches[mid].players {
                        if p0 == p1 {
                            return Some((i as f32 / 2f32).floor() as usize);
                        }
                    }
                }
            }
        }
        None
    }
    pub fn finish(&mut self, mid: &data::MatchId) {
        for pid in self.matches[mid].clone().players.iter().as_ref() {
            if self.matches[mid].states[pid] == data::PlayerResult::Unplayed {
                *self
                    .matches
                    .get_mut(mid)
                    .unwrap()
                    .states
                    .get_mut(pid)
                    .unwrap() = data::PlayerResult::Lost;
            }
        }
        (*self.matches.get_mut(mid).unwrap()).finished = true;
        if let Some(cmid) = &self.matches[mid].connection
            && self.matches[cmid].finished
        {
            let mut next_players = self.matches[mid].winners();
            next_players.append(&mut self.matches[cmid].winners());
            let next_round = std::cmp::max(self.matches[mid].round, self.matches[cmid].round) + 1;
            self.add_group(&next_players, next_round);
        }
        // TODO: handle new match creation based on connection field
    }
    // TODO: real input system
    fn get_players(&self) -> Vec<String> {
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
}
