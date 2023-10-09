use std::io::Write;

pub fn get_user_input(prompt_text: &str) -> String {
    println!("{}", prompt_text);
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn press_enter() {
    print!("Press enter to continue");
    std::io::stdout().flush().expect("Failed to flush");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

pub fn clear_screen() {
    println!("\x1B[2J");
}

