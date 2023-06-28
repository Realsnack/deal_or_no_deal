use crate::case::Case;

mod case;
mod game_logic;
mod menu;

fn main() {
    #[cfg(not(debug_assertions))]
    clear_screen();

    println!("Welcome to Deal or no deal!");
    let name = menu::get_user_input("Please enter your name:");
    println!("Hello, {}!", name);
    println!("");
    menu::press_enter();
    menu::clear_screen();

    let game = game_logic::GameLogic::new();

    let case = choose_case();
}

fn choose_case() -> Case {
    let case: u8;
    loop {
        let input = menu::get_user_input("Please choose a case number between 1 and 26:");
        case = input.trim().parse().expect("Failed to parse input");

        if (case < 1) || (case > 26) {
            println!("Invalid case number!");
            menu::press_enter();
        }
        break;
    }

    println!("You chose case number {}!", case);
    menu::press_enter();
    return Case {
        number: case,
        value: 0,
    };
}
