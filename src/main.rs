use std::io;
mod app;
mod player;
mod console;
mod settings;
mod board;
mod tiles;

fn main() {

    console::show_menu();

    let name = console::request_name();
    let player1 = player::Player::new(&name);

    let settings = settings::Settings::new(4, 4);

    app::load();
}
