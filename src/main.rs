mod bracket;
mod data;
mod gui;

fn main() -> iced::Result {
    iced::run("FFA Brackets", gui::App::update, gui::App::view)
}
