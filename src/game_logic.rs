use crate::case::Case;

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
}
