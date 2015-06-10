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
