mod case;
mod game_logic;
mod menu;

fn main() {
    #[cfg(not(debug_assertions))]
    menu::clear_screen();

    println!("Welcome to Deal or no deal!");
    let name = menu::get_user_input("Please enter your name:");
    println!("Hello, {}!", name);
    println!("");
    menu::press_enter();
    menu::clear_screen();

    let mut game = game_logic::GameLogic::new();
    game.start_game();

    println!("Thanks for playing!");
}
