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
        let mut problem_group = chunked_groups.remainder().to_vec();
        for group in chunked_groups.by_ref() {
            self.add_group(group, if problem_group.is_empty() { 0 } else { 1 });
        }
        if !problem_group.is_empty() {
            self.fill_group(&mut problem_group);
            self.add_group(&problem_group, 0);
        }
        self.connect_matches();
    }
    pub fn connect_matches(&mut self) {
        for (ri, round) in self.rounds.iter().enumerate() {
            let unconnected = round
                .iter()
                .filter(|mid| mid != &&data::MatchId(0) && self.matches[mid].connection.is_none())
                .collect::<Vec<&data::MatchId>>();
            let mut pairs = unconnected.chunks_exact(2);
            for pair in pairs.by_ref() {
                (*self.matches.get_mut(&pair[0]).unwrap()).connection = Some(pair[1].clone());
                (*self.matches.get_mut(&pair[1]).unwrap()).connection = Some(pair[0].clone());
            }
            let extra = pairs.remainder().to_vec();
            if !extra.is_empty() &&
                ri + 1 < self.rounds.len() &&
                !&self.rounds[ri + 1].is_empty() &&
                self.matches[&self.rounds[ri + 1][0]].connection.is_none()
            {
                (*self.matches.get_mut(&extra[0]).unwrap()).connection = Some(self.rounds[ri + 1][0].clone());
                (*self.matches.get_mut(&self.rounds[ri + 1][0]).unwrap()).connection = Some(extra[0].clone());
            }
        }
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
        self.add_to_round(&id, None, round);
        // TODO: this round business is all hogwash and I hate it and it's unfinished
        //self.connect_matches();
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
    pub fn add_to_round(&mut self, mid: &data::MatchId, cid: Option<&data::MatchId>, round: usize) {
        while self.rounds.len() <= round {
            self.rounds.push(vec![]);
        }
        if let Some(cid) = cid {
            let (rounds_i, round_i) = self.get_indices_for_round_insert(mid, cid);
            while self.rounds[rounds_i].len() <= round_i {
                self.rounds.get_mut(rounds_i).unwrap().push(data::MatchId(0));
            }
            self.rounds.get_mut(rounds_i).unwrap()[round_i] = *mid;
        } else {
            self.rounds.get_mut(round).unwrap().push(*mid);
        }
    }
    fn get_indices_for_round_insert(&self, mid1: &data::MatchId, mid2: &data::MatchId) -> (usize, usize) {
        let mut m1 = self.matches[mid1].clone();
        let mut m2 = self.matches[mid2].clone();
        if m1.round > m2.round {
            let temp = m1.clone();
            m1 = m2.clone();
            m2 = temp.clone();
        }

        let m1_index = {
            let mut res = 0;
            for (i, mid) in self.rounds[m1.round].iter().enumerate() {
                if mid == mid1 {
                    res = i;
                    break;
                }
            }
            res
        };
        let m2_index = {
            let mut res = 0;
            for (i, mid) in self.rounds[m2.round].iter().enumerate() {
                if mid == mid2 {
                    res = i;
                    break;
                }
            }
            res
        };

        if m1.round == m2.round {
            (m1.round + 1, m1_index)
        } else {
            (m2.round + 1, m1_index)
        }
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
            self.connect_matches();
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
