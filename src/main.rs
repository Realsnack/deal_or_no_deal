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

    let mut game = game_logic::GameLogic::new();
    game.choose_case();
    game.start_game();
    
}
