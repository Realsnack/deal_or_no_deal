use crate::case::Case;

pub struct GameLogic {
    pub players_case: Case,
    pub available_cases: Vec<Case>,
    pub opened_cases: Vec<Case>,
    pub round: u8,
}
