#[derive(Clone)]
pub enum MineState {
    Empty,
    Flag,
    Question,
    Revaled,
}

pub struct Minesweeper {
    mines: Vec<bool>,
    num_mines: i32,
    mine_states: Vec<MineState>,
}

impl Minesweeper {
    pub fn new(width: u32, height: u32, num_mines: i32) {
        let board_size = (width * height) as usize;

        let mut result = Self {
            mines: vec![false; board_size],
            num_mines: num_mines,
            mine_states: vec![MineState::Empty; board_size],
        };

        result.start();
    }

    fn start(&mut self) {
        for mine_state in self.mine_states.iter_mut() {
            *mine_state = MineState::Empty
        }
    }
}
