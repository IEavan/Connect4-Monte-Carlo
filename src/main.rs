mod grid;
use grid::*;

fn main() {
    let mut test_grid: GridState = GridState::new();
    test_grid.insert(4).unwrap();
    println!("{}", test_grid);
    let moves = test_grid.get_moves();
    for game_move in moves.iter() {
        match game_move {
            Some(game_move) => println!("{}", game_move),
            None => println!("Nothing")
        }
    }

}
