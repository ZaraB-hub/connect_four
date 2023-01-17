

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub grid: Vec<Vec<char>>,
    pub player1: Vec<usize>,
    pub player2: Vec<usize>,
    pub current_player: char,
    pub max_turns: usize,
    pub moves: usize,
    
}
