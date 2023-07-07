use crate::menu;
use rand::Rng;
use std::collections::HashMap;

pub struct GameState {
    players_case: usize,
    players_case_value: usize,
    available_cases: HashMap<usize, usize>,
    opened_cases: HashMap<usize, usize>,
    round: usize,
}

impl GameState {
    pub fn new() -> GameState {
        // First get players case number
        let players_case_number = Self::choose_case_number();
        // Then generate case values and fill available cases map
        let case_values = Self::generate_case_values();
        let mut available_cases: HashMap<usize, usize> = HashMap::new();
        for i in 0..26 {
            available_cases.insert(i + 1, case_values[i]);
        }

        if available_cases.len() != 26 {
            panic!("Available cases map is not 26!");
        }

        let players_case_value = available_cases.get(&players_case_number)
            .unwrap()
            .clone();

        // Then remove players case from available cases map
        available_cases.remove(&players_case_number);

        if available_cases.len() != 25 {
            panic!("Available cases map is not 25!");
        }

        // Then start game
        GameState {
            players_case: players_case_number,
            players_case_value,
            round: 0,
            available_cases,
            opened_cases: HashMap::new(),
        }
    }

    fn choose_case_number() -> usize {
        let mut case_number: usize;
        loop {
            let input = menu::get_user_input("Please choose a case number between 1 and 26:");
            case_number = input.trim().parse().expect("Failed to parse input");

            if (case_number < 1) || (case_number > 26) {
                println!("Invalid case number!");
                menu::press_enter();
                case_number = Self::choose_case_number();
            }
            break;
        }

        case_number
    }

    fn generate_case_values() -> Vec<usize> {
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

    pub fn start_game(&mut self) {
        println!("Starting game...");
    }
}
