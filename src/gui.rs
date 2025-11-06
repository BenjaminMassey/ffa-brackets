use iced::Center;
use iced::widget::{Button, Column, Row, button, container, scrollable, text};

use crate::bracket;
use crate::data;

#[derive(Default)]
pub struct App {
    pub bracket: bracket::Bracket,
}

#[derive(Debug, Clone)]
pub enum Message {
    Result(data::MatchId, data::PlayerId, data::PlayerResult),
    Finish(data::MatchId),
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Result(mid, pid, pr) => {
                *self
                    .bracket
                    .matches
                    .get_mut(&mid)
                    .unwrap()
                    .states
                    .get_mut(&pid)
                    .unwrap() = pr;
            }
            Message::Finish(mid) => {
                self.bracket.finish(&mid);
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let mut root = Row::new().spacing(50);
        for (i, round) in self.bracket.rounds.iter().enumerate() {
            let mut round_container = Column::new()
                .padding(iced::padding::top(50f32 * (3 * i) as f32))
                .spacing(50f32 * ((4 * i) + 3) as f32);
            for mid in round {
                if mid == &data::MatchId(0) {
                    round_container = round_container.push(container("").height(250));
                    // TODO: better temp
                    continue;
                }
                let m = self.bracket.matches[mid].clone();
                let mut match_container: Row<'_, Message> = Row::new().spacing(40).align_y(Center);
                let mut player_list: Column<'_, Message> = Column::new().spacing(10);
                for pid in &m.players {
                    let pname = &self.bracket.players[&pid].name;
                    let ptext = format!("{} [{}]", pname, m.states[&pid]);
                    let mut player_container: Row<'_, Message> = Row::new().spacing(20);
                    player_container = player_container.push(text(ptext).size(28));
                    if m.states[&pid] == data::PlayerResult::Unplayed {
                        let win_button: Button<'_, Message> = button("W")
                            .on_press(Message::Result(*mid, *pid, data::PlayerResult::Won));
                        player_container = player_container.push(win_button);
                        let lose_button: Button<'_, Message> = button("L")
                            .on_press(Message::Result(*mid, *pid, data::PlayerResult::Lost));
                        player_container = player_container.push(lose_button);
                    }
                    player_list = player_list.push(player_container);
                }
                match_container = match_container.push(player_list);
                if !m.finished {
                    let finish_button: Button<'_, Message> =
                        button("Finish").on_press(Message::Finish(*mid));
                    match_container = match_container.push(finish_button);
                }
                round_container = round_container.push(match_container);
            }
            root = root.push(round_container);
        }
        scrollable(root).into()
    }
}
