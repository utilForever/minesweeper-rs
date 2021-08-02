use crate::gui::GUI;

use rand::Rng;

#[derive(Clone)]
pub enum MineState {
    Empty,
    Flag,
    Question,
    Revaled,
}

pub struct Minesweeper {
    width: u32,
    height: u32,

    mines: Vec<bool>,
    num_mines: i32,
    mine_states: Vec<MineState>,

    gui: GUI,
}

impl Minesweeper {
    pub fn new(width: u32, height: u32, num_mines: i32, gui: &GUI) -> Self {
        let board_size = (width * height) as usize;

        let mut result = Self {
            width: width,
            height: height,

            mines: vec![false; board_size],
            num_mines: num_mines,
            mine_states: vec![MineState::Empty; board_size],

            gui: gui.clone(),
        };

        result.start();

        result
    }

    fn start(&mut self) {
        for mine_state in self.mine_states.iter_mut() {
            *mine_state = MineState::Empty
        }

        self.generate_mines();
    }

    fn generate_mines(&mut self) {
        for mine in self.mines.iter_mut() {
            *mine = false
        }

        let mut rng = rand::thread_rng();
        let side = rand::distributions::Uniform::new(0, self.mines.len());

        for _i in 0..self.num_mines {
            let mut idx: usize;

            while {
                idx = rng.sample(side);
                self.mines[idx]
            } {}

            self.mines[idx] = true;
        }
    }

    fn is_mine(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            false
        } else {
            let idx = (y as u32 * self.height + x as u32) as usize;
            self.mines[idx]
        }
    }

    fn get_count_neighbor_mines(&self, x: u32, y: u32) -> u32 {
        let mut count: u32 = 0;

        // North West
        if self.is_mine(x as i32 - 1, y as i32 - 1) {
            count += 1;
        }
        // North
        if self.is_mine(x as i32, y as i32 - 1) {
            count += 1;
        }
        // North East
        if self.is_mine(x as i32 + 1, y as i32 - 1) {
            count += 1;
        }
        // East
        if self.is_mine(x as i32 + 1, y as i32) {
            count += 1;
        }
        // South East
        if self.is_mine(x as i32 + 1, y as i32 + 1) {
            count += 1;
        }
        // South
        if self.is_mine(x as i32, y as i32 + 1) {
            count += 1;
        }
        // South West
        if self.is_mine(x as i32 - 1, y as i32 + 1) {
            count += 1;
        }
        // West
        if self.is_mine(x as i32 - 1, y as i32) {
            count += 1;
        }

        count
    }

    pub fn show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.height + x) as usize;
                print!("{}", if self.mines[idx] { "*" } else { "." });
            }
            println!();
        }
    }

    pub fn show_with_mine_count(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.height + x) as usize;
                print!(
                    "{}",
                    if self.mines[idx] {
                        "*".to_string()
                    } else {
                        self.get_count_neighbor_mines(x, y).to_string()
                    }
                );
            }
            println!();
        }
    }
}
