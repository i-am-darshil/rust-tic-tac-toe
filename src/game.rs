mod player;
mod board;

use crate::game::player::*;
use crate::game::board::*;

pub struct Game {
  board: Board,
  current_player: PlayerType,
  is_game_over: bool,
  num_moves: usize,
  winner: PlayerType,
}

impl Game {
  pub fn new(size: usize) -> Game{
    Game {
      board: Board::new(size),
      current_player: PlayerType::X,
      is_game_over: false,
      num_moves: 0,
      winner: PlayerType::None,
    }
  }

  pub fn is_game_over(&self) -> bool { 
    self.is_game_over
  }

  pub fn get_winner(&self) -> PlayerType {
    self.winner
  }

  pub fn make_move(&mut self, row: usize, col: usize) {

    if self.board.is_empty_cell(row, col) {
      self.board.set_cell(row, col, self.current_player.clone());
      self.is_game_over = self.board.check_if_winner(row, col, self.current_player.clone());
      if self.is_game_over == true {
        self.winner = self.current_player.clone();
      }
      self.num_moves += 1;
      self.is_game_over = self.is_game_over || self.num_moves >= self.board.get_num_cells()
    }

    if self.current_player == PlayerType::X {
      self.current_player = PlayerType::O;
    } else {
      self.current_player = PlayerType::X;
    }
  }

  pub fn get_row_col_from_input(input: String) -> (Option<usize>, Option<usize>) {
    let row_col: Vec<&str> = input.split_whitespace().collect();
    if row_col.len() != 2 {
      return (None, None);
    }
    let row = row_col[0].parse::<usize>().unwrap();
    let col = row_col[1].parse::<usize>().unwrap();
    (Some(row), Some(col))
  }

  pub fn display(&self) {
    println!("================================================================");
    self.board.print_cells();
    println!("================================================================");
  }

  pub fn get_current_player(&self) -> PlayerType {
    self.current_player.clone()
  }
  
}