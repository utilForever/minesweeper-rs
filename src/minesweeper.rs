#[derive(Clone)]
pub enum MineState {
    Empty,
    Flag,
    Question,
    Revaled,
}

pub struct Minesweeper {
    mines: Vec<bool>,
    mine_states: Vec<MineState>,
}

impl Minesweeper {
    pub fn new(
        width: u32,
        height: u32,
    ) -> windows::Result<Self> {
        let board_size = (width * height) as usize;

        Ok(Self {
            mines: vec![false; board_size],
            mine_states: vec![MineState::Empty; board_size],
        })
    }
}
