use std::io::Write;

mod game_logic;
mod case;

const CLEAR_SCREEN: &str = "\x1B[2J";

fn main() {
    #[cfg(not(debug_assertions))]
    clear_screen();

    println!("Welcome to Deal or no deal!");
    let name = get_user_input("Please enter your name:");
    println!("Hello, {}!", name);
    println!("");
    press_enter();
    clear_screen();

    let case = get_user_input("Please choose a case number between 1 and 26:");
    println!("You chose case {}!", case);
    press_enter();
}

fn get_user_input(prompt_text: &str) -> String {
    println!("{}", prompt_text);
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn press_enter() {
    print!("Press enter to continue");
    std::io::stdout().flush().expect("Failed to flush");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

fn clear_screen() {
    println!("{}", CLEAR_SCREEN);
}
