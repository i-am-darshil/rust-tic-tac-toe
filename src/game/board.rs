use crate::game::player::*;

pub struct Board {
  board: Vec<Vec<PlayerType>>,
  size: usize,
}

impl Board {
  pub fn new(size: usize) -> Board {
    Board {
      board: vec![vec![PlayerType::None; size]; size],
      size: size,
    }
  }

  pub fn check_if_winner(&self, row: usize, col:usize, current_player: PlayerType) -> bool {
    // check row
    let mut winner = true;
    for i in 0..self.size {
      if self.board[row][i] != current_player {
        winner = false;
        break;
      }
    }
    if winner == true { return true; }

    // check col
    let mut winner = true;
    for i in 0..self.size {
      if self.board[i][col] != current_player {
        winner = false;
        break;
      }
    }
    if winner == true { return true; }

    // check primary diagonal
    let mut winner = true;
    for i in 0..self.size {
      if self.board[i][i] != current_player {
        winner = false;
        break;
      }
    }

    if winner == true { return true; }

    // check secondary diagonal
    let mut winner = true;
    for i in 0..self.size {
      if self.board[i][self.size - 1 - i] != current_player {
        winner = false;
        break;
      }
    }

    return winner;
  }

  pub fn set_cell(&mut self, row: usize, col:usize, value: PlayerType) {
    self.board[row][col] = value;
  }

  pub fn is_valid_cell(&self, row: usize, col:usize) -> bool {
    return row < self.size && col < self.size;
  }

  pub fn is_empty_cell(&self, row: usize, col:usize) -> bool {
    return self.is_valid_cell(row, col) && self.board[row][col] == PlayerType::None;
  }

  pub fn get_num_cells(&self) -> usize { self.size * self.size }

  pub fn print_cells(&self) {
    for row in 0..self.size {
      for col in 0..self.size {
        if self.board[row][col] == PlayerType::None {
          print!("{:?} ", self.board[row][col]);
        } else {
          print!("  {:?}  ", self.board[row][col]);
        }
      }
      println!();
    }
  }
}