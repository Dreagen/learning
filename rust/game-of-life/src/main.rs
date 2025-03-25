use core::time;
use rand::prelude::*;
use std::{
    collections::HashMap,
    io::Write,
    thread::{self},
};

fn main() {
    let mut board = Board::new(250, 60);

    board.print();
    for _ in 0..1000 {
        board.update();

        thread::sleep(time::Duration::from_millis(10));
    }
}

const DARK_GRAY: &str = "\x1B[30m";
const LIGHT_BLUE: &str = "\x1B[94m";
const WHITE: &str = "\x1B[0m";

struct Board {
    generation: u32,
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    previous_states: HashMap<Position, State>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut board = Board {
            generation: 0,
            width,
            height,
            grid: Vec::new(),
            previous_states: HashMap::new(),
        };

        for h in 0..height {
            let mut row = Vec::new();

            for w in 0..width {
                let cell = Cell::new(h, w);
                row.push(cell);
            }

            board.grid.push(row);
        }

        board
    }

    fn get_cell_at_pos(&self, pos: Position) -> Option<&Cell> {
        match self.grid.iter().nth(pos.y) {
            Some(row) => row.iter().nth(pos.x),
            None => None,
        }
    }

    fn update(&mut self) {
        self.generation += 1;
        move_cursor(0, 0);
        println!("{}Generation: {}", WHITE, self.generation);
        std::io::stdout().flush().unwrap(); // Make sure the text appears immediately
        let mut new_states = HashMap::new();
        self.previous_states = HashMap::new();

        for cells_in_row in &self.grid {
            for cell in cells_in_row {
                self.previous_states.insert(cell.pos.clone(), cell.state);
            }
        }

        for cells_in_row in &self.grid {
            for cell in cells_in_row {
                new_states.insert(cell.pos.clone(), cell.get_next_state(self));
            }
        }

        for new_state in new_states {
            let (position, state) = new_state;
            let previous_state = self.previous_states.get(&position).unwrap();
            if previous_state != &state {
                move_cursor(position.x, position.y + 1);
                self.grid[position.y][position.x].state = state;
                print!("{}", self.grid[position.y][position.x].print());
            }
        }

        move_cursor(0, self.height);
    }

    fn print(&self) {
        clear_console();
        println!("{}Generation: {}", WHITE, self.generation);
        for cells_in_row in &self.grid {
            for cell in cells_in_row {
                print!("{}", cell.print());
            }

            println!();
        }
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn move_cursor(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}

struct Cell {
    state: State,
    pos: Position,
}

impl Cell {
    fn print(&self) -> String {
        match self.state {
            State::Alive => format!("{}o{}", LIGHT_BLUE, DARK_GRAY),
            State::Dead => format!("{}o", DARK_GRAY),
        }
    }

    fn get_next_state(&self, board: &Board) -> State {
        let mut live_neighbours = Vec::new();

        let top_left = if self.pos.x > 0 && self.pos.y > 0 {
            Some(Position {
                x: self.pos.x - 1,
                y: self.pos.y - 1,
            })
        } else {
            None
        };

        let top = if self.pos.y > 0 {
            Some(Position {
                x: self.pos.x,
                y: self.pos.y - 1,
            })
        } else {
            None
        };

        let top_right = if self.pos.x + 1 < board.width && self.pos.y > 0 {
            Some(Position {
                x: self.pos.x + 1,
                y: self.pos.y - 1,
            })
        } else {
            None
        };

        let right = if self.pos.x + 1 < board.width {
            Some(Position {
                x: self.pos.x + 1,
                y: self.pos.y,
            })
        } else {
            None
        };

        let bottom_right = if self.pos.x + 1 < board.width && self.pos.y + 1 < board.height {
            Some(Position {
                x: self.pos.x + 1,
                y: self.pos.y + 1,
            })
        } else {
            None
        };

        let bottom = if self.pos.y + 1 < board.height {
            Some(Position {
                x: self.pos.x,
                y: self.pos.y + 1,
            })
        } else {
            None
        };

        let bottom_left = if self.pos.x > 0 && self.pos.y + 1 < board.height {
            Some(Position {
                x: self.pos.x - 1,
                y: self.pos.y + 1,
            })
        } else {
            None
        };

        let left = if self.pos.x > 0 {
            Some(Position {
                x: self.pos.x - 1,
                y: self.pos.y,
            })
        } else {
            None
        };

        let neighbouring_cell_positions = [
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            left,
        ];

        for pos in neighbouring_cell_positions {
            if pos.is_some() {
                if let Some(cell) = board.get_cell_at_pos(pos.unwrap()) {
                    if cell.state == State::Alive {
                        live_neighbours.push(cell);
                    }
                };
            }
        }

        match live_neighbours.len() {
            0 => State::Dead,
            1 => State::Dead,
            2 if self.state == State::Alive => State::Alive,
            3 => State::Alive,
            _ => State::Dead,
        }
    }

    fn new(h: usize, w: usize) -> Self {
        let state = if rand::rng().random_bool(0.5) {
            State::Alive
        } else {
            State::Dead
        };

        Self {
            state,
            pos: Position { x: w, y: h },
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    Alive,
    Dead,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}
