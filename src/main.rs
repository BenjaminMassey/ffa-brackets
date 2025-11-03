use std::collections::{HashMap, HashSet};

mod bracket;
mod data;

fn main() {
    // init
    let mut bracket = bracket::Bracket::new(4);

    //players
    let player_names = get_players(); // TODO: input
    for name in player_names {
        let id = data::PlayerId(bracket.new_id());
        bracket.players.insert(id, data::Player { id, name });
    }
    bracket.make_groups();

    // display
    bracket.display();
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
