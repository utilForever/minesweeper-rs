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
}

impl Minesweeper {
    pub fn new(width: u32, height: u32, num_mines: i32) {
        let board_size = (width * height) as usize;

        let mut result = Self {
            width: width,
            height: height,

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

    pub fn show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.height + x) as usize;
                print!("{}", if self.mines[idx] { "*" } else { "." });
            }
            println!();
        }
    }
}
