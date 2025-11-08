use iced::{Center, Color};
use iced::widget::{Button, Column, Container, Row, button, container, scrollable, text};

use crate::bracket;
use crate::data;

// https://colorhunt.co/palette/37353e44444e715a5ad3dad9
const fg_color: iced::Color = iced::Color::from_rgb(211f32, 218f32, 217f32);
const alt_fg_color: iced::Color = iced::Color::from_rgb(113f32, 90f32, 90f32);
const alt_bg_color: iced::Color = iced::Color::from_rgb(68f32, 68f32, 78f32);
const bg_color: iced::Color = iced::Color::from_rgb(55f32, 53f32, 62f32);

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
        self.bracket_view()
    }
    fn bracket_view(&self) -> iced::Element<'_, Message> {
        let mut rounds_container = Row::new().spacing(50);
        for (i, round) in self.bracket.rounds.iter().enumerate() {
            rounds_container = rounds_container.push(self.round_view(i, round));
        }
        scrollable(rounds_container).into()
    }
    fn round_view(&self, round_index: usize, round: &[data::MatchId]) -> iced::Element<'_, Message>{
        let mut round_container = Column::new()
            .padding(iced::padding::top(50f32 * (3 * round_index) as f32))
            .spacing(50f32 * ((4 * round_index) + 3) as f32);
        for mid in round {
            if mid == &data::MatchId(0) {
                round_container = round_container.push(container("").height(250));
                // TODO: better temp
                continue;
            }
            let m = self.bracket.matches[mid].clone();
            round_container = round_container.push(self.match_view(mid, m));
        }
        round_container.into()
    }
    fn match_view(&self, mid: &data::MatchId, m: data::Match) -> Row<'_, Message> {
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
        match_container.into()
    }
}
