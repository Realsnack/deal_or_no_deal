use std::io::Write;

use crate::case::Case;

mod case;
mod game_logic;

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

    let game = game_logic::GameLogic::new();

    let case = choose_case();
}

fn choose_case() -> Case {
    let case: u8;
    loop {
        let input = get_user_input("Please choose a case number between 1 and 26:");
        case = input.trim().parse().expect("Failed to parse input");

        if (case < 1) || (case > 26) {
            println!("Invalid case number!");
            press_enter();
        }
        break;
    }

    println!("You chose case number {}!", case);
    press_enter();
    return Case {
        number: case,
        value: 0,
    };
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
