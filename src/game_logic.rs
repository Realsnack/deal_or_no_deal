use crate::{case::Case, menu};
use rand::Rng;

pub struct GameLogic {
    pub players_case: Case,
    pub available_cases: Vec<Case>,
    pub opened_cases: Vec<Case>,
    pub round: u8,
}

impl GameLogic {
    pub fn new() -> GameLogic {
        GameLogic {
            players_case: Case {
                number: 0,
                value: 0,
            },
            available_cases: Vec::new(),
            opened_cases: Vec::new(),
            round: 0,
        }
    }

    pub fn choose_case(&mut self) {
        let case: usize;
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

        self.players_case = Case {
            number: case,
            value: 0,
        };
    }

    pub fn start_game(&mut self) {
        // Generate values for all cases
        self.choose_case();
        
        let case_value_vector = Self::generate_case_values();
        println!("{:?}", case_value_vector);

        self.players_case.value = case_value_vector[self.players_case.number - 1];
        #[cfg(not(debug_assertions))]
        println!("Case value: {}", self.players_case.value);
    }

    fn generate_case_values() -> Vec<usize> {
        let mut values: Vec<usize> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut values_left = vec![
            0_01, 1, 5, 10, 25, 50, 75, 100, 200, 300, 400, 500, 750, 1_000, 5_000, 10_000, 25_000,
            50_000, 75_000, 100_000, 200_000, 300_000, 400_000, 500_000, 750_000, 1_000_000,
        ];

        for _ in 0..26 {
            let random_index = rng.gen_range(0..values_left.len());
            let random_value = values_left.remove(random_index);
            values.push(random_value);
        }

        values
    }
}
