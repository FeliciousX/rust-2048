pub fn show_menu() {
    for i in 0..42 {
        print!("=");
    }
    print!(" 2048 Game ");
    for i in 0..42 {
        print!("=");
    }

    println!("\n");
}

pub fn request_name() -> String {
    use std::io;

    println!("Please key in your name: ");
    let mut name = String::new();

    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to read name");

    println!("\nWelcome, {}", name);

    name.to_string()
}

pub fn get_instruction() -> String {
    use std::io;

    show_instruction();

    let mut instruction = String::new();

    io::stdin().read_line(&mut instruction)
        .ok()
        .expect("Failed to read instruction");

    instruction.to_string()
}

fn show_instruction() {
    println!("Instruction: ");
    println!("8 to move up");
    println!("2 to move down");
    println!("4 to move left");
    println!("6 to move right");
}
