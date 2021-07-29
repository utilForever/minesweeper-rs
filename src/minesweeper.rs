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
    pub fn new(
        width: u32,
        height: u32,
        num_mines: i32,
    ) -> windows::Result<Self> {
        let board_size = (width * height) as usize;

        Ok(Self {
            mines: vec![false; board_size],
            num_mines: num_mines,
            mine_states: vec![MineState::Empty; board_size],
        })
    }
}
