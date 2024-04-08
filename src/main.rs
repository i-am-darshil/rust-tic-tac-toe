mod game;
use crate::game::*;
use simple_user_input::*;

fn main() {
    println!("Hello, world, welcome to tic tac toe! Player X begins...");
    println!("Use the following format: row column. E.g: 2 1");
    let size = 3;
    let mut game = Game::new(size);

    while game.is_game_over() == false {
      print!("Player {:?} turn, ", game.get_current_player());
      let input = get_input("Enter row column:");
      let (row, col) = Game::get_row_col_from_input(input);
      
      if row.is_none() || col.is_none() {
        println!("Invalid input. Try again.");
        continue;
      }
      game.make_move(row.unwrap(), col.unwrap());
      game.display();
    }

    println!("Winner is: {:?}", game.get_winner());

}

mod simple_user_input {
  use std::io;
  pub fn get_input(prompt: &str) -> String{
      println!("{}",prompt);
      let mut input = String::new();
      match io::stdin().read_line(&mut input) {
          Ok(_goes_into_input_above) => {},
          Err(_no_updates_is_fine) => {},
      }
      input.trim().to_string()
  }
}
