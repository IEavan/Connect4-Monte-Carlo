extern crate rand;

mod grid;
mod mcts;
use grid::*;
use mcts::*;

fn main() {
    let mut test_grid: GridState = GridState::new();
    println!("{}", test_grid);
    loop {
        let move_list = test_grid.get_moves();
        let game_move = get_best_move(move_list, 100);
        test_grid = game_move.next;
        println!("{}", test_grid);
        if let Some(winner) = test_grid.winner {
            println!("{:?} wins", winner);
            break;
        }
    }
}
