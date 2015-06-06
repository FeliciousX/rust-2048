use std::io;
// TODO: Implement them modularly
//mod app;
//mod board;
//mod tiles;

fn main() {

    // TODO: Place in IO functions
    for  i in 0..42 {
        print!("=");
    }
    print!(" 2048 Game ");
    for i in 0..42 {
        print!("=");
    }

    println!("\n");
    // end fn

    // TODO: Place in IO function
    println!("Please key in your name: ");
    let mut name = String::new();

    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to read name");

    println!("\nWelcome, {}", name);
    // end fn

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
