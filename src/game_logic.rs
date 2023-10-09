use crate::menu;
use crate::game_logic;
use rand::Rng;
use std::collections::HashMap;
use colorize::AnsiColor;

#[derive(Debug)]
pub struct GameState {
    players_case: usize,
    players_case_value: usize,
    available_cases: HashMap<usize, usize>,
    opened_cases: HashMap<usize, usize>,
    round: usize,
}

pub fn create_game() -> GameState {
    let players_case = game_logic::choose_case_number();
    let case_values = generate_case_values();

    let mut available_cases: HashMap<usize, usize> = HashMap::new();


    for i in 0..26 {
        available_cases.insert(i+1, case_values[i]);
    }

    let players_case_value = available_cases.get(&players_case)
        .unwrap()
        .clone();

    available_cases.remove(&players_case);
    let mut opened_cases: HashMap<usize, usize> = HashMap::new();
    opened_cases.insert(players_case, players_case_value);

    GameState {
        players_case,
        players_case_value,
        round: 1,
        available_cases,
        opened_cases
    }
}

pub fn choose_case_number() -> usize {
    let case_number: usize;
    loop {
        let input = menu::get_user_input("Please choose a case number between 1 and 26:");
        let parsed_input = input.trim().parse::<usize>();  // usize is used as an example, use the appropriate type

        match parsed_input {
            Ok(num) => {
                if (num < 1) || (num > 26) {
                    println!("Invalid case number!");
                    menu::press_enter();
                } else {
                    case_number = num;
                    break;
                }
            },
            Err(_) => {
                println!("Please enter a valid number.");
                menu::press_enter();
            },
        }
    }

    case_number
}

pub fn generate_case_values() -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut values_left = vec![
        1, 10, 50, 100, 250, 500, 750, 1_000, 2_000, 3_000, 4_000, 5_000, 7_500, 10_000, 50_000, 100_000, 250_000,
        500_000, 750_000, 1_000_000, 2_000_000, 3_000_000, 4_000_000, 5_000_000, 7_500_000, 10_000_000,
    ];

    for _ in 0..26 {
        let random_index = rng.gen_range(0..values_left.len());
        let random_value = values_left.remove(random_index);
        values.push(random_value);
    }

    values
}

pub fn play_round(game_state: GameState) -> GameState {
    let round_map: HashMap<usize, usize> = HashMap::from([
                                                         (1,6),
                                                         (2,5),
                                                         (3,4),
                                                         (4,3),
                                                         (5,2),
                                                         (6,1),
                                                         (7,1),
                                                         (8,1),
                                                         (9,1),
                                                         (10,2)
    ]);

    let mut available_cases = game_state.available_cases;
    let mut opened_cases = game_state.opened_cases;

    for _ in 1usize..round_map.get(&game_state.round).unwrap().clone()+1 {
        println!();
        let case_to_open = game_logic::choose_case_number();

        if opened_cases.contains_key(&case_to_open) {
            println!("That case has already been opened!");
            menu::press_enter();
            continue;
        }

        let case_value = available_cases.get(&case_to_open)
            .unwrap()
            .clone();

        available_cases.remove(&case_to_open);
        opened_cases.insert(case_to_open, case_value);

        match case_value {
            _ if case_value >= 250_000 => println!("You opened case {} which contained ${}!", case_to_open, case_value.to_string().red()),
            _ if case_value < 250_000 => println!("You opened case {} which contained ${}!", case_to_open, case_value.to_string().green()),
            _ => {}
        };
    }

    GameState {
        players_case: game_state.players_case,
        players_case_value: game_state.players_case_value,
        available_cases,
        opened_cases,
        round: 2
    }
}

