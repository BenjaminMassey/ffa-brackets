use iced::widget::{Button, Column, Row, button, text};

use crate::bracket;

#[derive(Default)]
pub struct App {
    pub bracket: bracket::Bracket,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Ping,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Ping => {
                println!("Pong");
            }
        }
    }
    pub fn view(&self) -> Column<'_, Message> {
        let mut root = Column::new().spacing(50);
        for (_id, m) in &self.bracket.matches {
            let mut match_container: Column<'_, Message> = Column::new();
            for pid in &m.players {
                let pname = &self.bracket.players[&pid].name;
                let mut player_container: Row<'_, Message> = Row::new();
                player_container = player_container.push(text(pname));
                let but: Button<'_, Message> = button("!").on_press(Message::Ping);
                player_container = player_container.push(but);
                match_container = match_container.push(player_container);
            }
            root = root.push(match_container);
        }
        root
    }
}
