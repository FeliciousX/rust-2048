use std::io;
mod app;
mod player;
mod console;
mod board;
mod tiles;

fn main() {

    console::show_menu();

    let name = console::request_name();
    let player1 = player::Player::new(&name);

    // TODO: Place in IO function
    println!("Instruction: ");
    println!("8 to move up");
    println!("2 to move down");
    println!("4 to move left");
    println!("6 to move right");

    let mut instruction = String::new();

    io::stdin().read_line(&mut instruction)
        .ok()
        .expect("Failed to read instruction");

    println!("\nSelected {}", instruction);
    // end fn
}
