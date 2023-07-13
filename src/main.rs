mod game_logic;
mod menu;

fn main() {
    #[cfg(not(debug_assertions))]
    menu::clear_screen();

    println!("Welcome to Deal or no deal!");
    let name = menu::get_user_input("Please enter your name:");
    println!("Hello, {}!", name);
    println!();
    menu::press_enter();
    menu::clear_screen();

    let game_state = game_logic::create_game();

    println!("{:?}", game_state);

    let game_state = game_logic::play_round(game_state, 1);

    println!("Thanks for playing!");
}
