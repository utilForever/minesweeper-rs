pub struct Minesweeper {
    mines: Vec<bool>,
}

impl Minesweeper {
    pub fn new(
        width: u32,
        height: u32,
    ) -> windows::Result<Self> {
        Ok(Self {
            mines: vec![false; (width * height) as usize],
        })
    }
}
