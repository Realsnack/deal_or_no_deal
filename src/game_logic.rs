use crate::{case::Case, menu};

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

        self.players_case = Case {
            number: case,
            value: 0,
        };
    }
    
    pub fn start_game() {
        // Generate values for all cases
        let mut case_value_vector = Self::generate_case_values();
    }

    fn generate_case_values() -> Vec<usize> {
        let mut values: Vec<usize> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut values_left = vec![
            1, 10, 100, 500, 1000, 5000, 10000, 25000, 50000, 75000, 100000, 200000, 300000,
            400000, 500000, 750000, 1000000,
        ];
    
        for _ in 0..26 {
            let index = rng.gen_range(0, values_left.len());
            values.push(values_left[index]);
            values_left.remove(index);
        }
    
        values
    }
}
