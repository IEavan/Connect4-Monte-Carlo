// Data definitions for the connect4 grid state

#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    Yellow,
    Red
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    owner: Player,
    horizontal: u8,
    vertical: u8,
    rising_diagonal: u8,
    falling_diagonal: u8
}

pub struct GridState {
    grid: [[Option<Tile>; 9]; 6],
    turn: Player,
    winner: Option<Player>
}

pub struct GameMove {
    previous: GridState,
    next: GridState
}

pub struct Game {
    moves: Vec<GameMove>
}

impl GridState {
    pub fn new() -> GridState {
        let empty_grid: [[Option<Tile>; 9]; 6] = Default::default();
        return GridState {grid: empty_grid, turn: Player::Yellow, winner: None};
    }

    pub fn get_moves(&self) -> [Option<GameMove>; 9] {
        let mut move_list: [Option<GameMove>; 9] = Default::default();
        for i in 0..9 {
            let previous = self.clone();
            let mut next = self.clone();
            move_list[i] = match next.insert(i) {
                Ok(_)  => Some(GameMove {previous: previous, next: next}),
                Err(_) => None
            };
        }
        move_list
    }

    pub fn insert(&mut self, column: usize) -> Result<(), &'static str> {
        if let Some(_) = self.grid[0][column] {return Err("Column is full");}
        if let Some(_) = self.winner {return Err("Game is over");}

        let mut free_position = 0;
        for j in (0..6).rev() {
            if let Some(_) = self.grid[j][column] {continue;}
            else {
                free_position = j;
                break;
            }
        }

        let adjacent_tiles = self.get_adjacent_tiles(column, free_position);
        let new_tile: Tile = Tile {owner: self.turn,
            horizontal: GridState::sum_horizontal(adjacent_tiles[3], adjacent_tiles[5]),
            vertical: GridState::sum_vertical(adjacent_tiles[1], adjacent_tiles[7]),
            falling_diagonal: GridState::sum_falling(adjacent_tiles[0], adjacent_tiles[8]),
            rising_diagonal: GridState::sum_rising(adjacent_tiles[2], adjacent_tiles[6])
        };

        if new_tile.horizontal == 4 || new_tile.vertical == 4 ||
           new_tile.falling_diagonal == 4 || new_tile.rising_diagonal == 4 {
            self.winner = Some(self.turn);
        }
        println!("Tile horizontal: {}", new_tile.vertical);

        self.grid[free_position][column] = Some(new_tile);
        self.turn = match self.turn {
            Player::Yellow => Player::Red,
            Player::Red    => Player::Yellow
        };
        Ok(())
    }

    fn sum_horizontal(t1: Option<Tile>, t2: Option<Tile>) -> u8 {
        match (t1, t2) {
            (None, None) => 1,
            (None, Some(t)) => t.horizontal + 1,
            (Some(t), None) => t.horizontal + 1,
            (Some(t1),  Some(t2)) => t1.horizontal + t2.horizontal + 1
        }
    }
    fn sum_vertical(t1: Option<Tile>, t2: Option<Tile>) -> u8 {
        match (t1, t2) {
            (None, None) => 1,
            (None, Some(t)) => t.vertical + 1,
            (Some(t), None) => t.vertical + 1,
            (Some(t1),  Some(t2)) => t1.vertical + t2.vertical + 1
        }
    }
    fn sum_falling(t1: Option<Tile>, t2: Option<Tile>) -> u8 {
        match (t1, t2) {
            (None, None) => 1,
            (None, Some(t)) => t.falling_diagonal + 1,
            (Some(t), None) => t.falling_diagonal + 1,
            (Some(t1),  Some(t2)) => t1.falling_diagonal + t2.falling_diagonal + 1
        }
    }
    fn sum_rising(t1: Option<Tile>, t2: Option<Tile>) -> u8 {
        match (t1, t2) {
            (None, None) => 1,
            (None, Some(t)) => t.rising_diagonal + 1,
            (Some(t), None) => t.rising_diagonal + 1,
            (Some(t1),  Some(t2)) => t1.rising_diagonal + t2.rising_diagonal + 1
        }
    }

    fn get_adjacent_tiles(&self, column: usize, row: usize) -> [Option<Tile>; 9] {
        let mut adjacent_tiles: [Option<Tile>; 9] = Default::default();
        for i in -1..2 {
            for j in -1..2 {
                let tile;
                if row as i8 + i < 0 || row as i8 + i > 5 {
                    tile = None;
                } else if i == 0 && j == 0 {
                    tile = None;
                } else if column as i8 + j < 0 || column as i8 + j > 8 {
                    tile = None;
                } else {
                    tile = self.grid[(row as i8 + i) as usize][(column as i8 + j) as usize];
                }
                adjacent_tiles[(3 * (i + 1) + (j + 1)) as usize] = tile;
            }
        }
        adjacent_tiles
    }
}

impl fmt::Display for GridState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{:?}'s Turn", self.turn));
        for row in self.grid.iter() {
            try!(write!(f, "["));
            for grid_cell in row.iter() {
                let cell_char = match grid_cell {
                    None => " _",
                    Some(t) => match t.owner {
                        Player::Yellow => " Y",
                        Player::Red => " R",
                    }
                };
                try!(write!(f, "{}", cell_char));
            }
            try!(writeln!(f, " ]"));
        }
        write!(f, "")
    }
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{:?}'s Turn                {:?}'s Turn",
                      self.previous.turn, self.next.turn));

        for (i, (row_prev, row_next)) in self.previous.grid.iter()
                                    .zip(self.next.grid.iter())
                                    .enumerate() {
            try!(write!(f, "["));
            for grid_cell in row_prev.iter() {
                let cell_char = match grid_cell {
                    None => " _",
                    Some(t) => match t.owner {
                        Player::Yellow => " Y",
                        Player::Red => " R",
                    }
                };
                try!(write!(f, "{}", cell_char));
            }
            try!(write!(f, " ]"));

            if i == 2 {
                try!(write!(f, " --> "));
            } else {
                try!(write!(f, "     "));
            }

            try!(write!(f, "["));
            for grid_cell in row_next.iter() {
                let cell_char = match grid_cell {
                    None => " _",
                    Some(t) => match t.owner {
                        Player::Yellow => " Y",
                        Player::Red => " R",
                    }
                };
                try!(write!(f, "{}", cell_char));
            }
            try!(writeln!(f, " ]"));
        }
        write!(f, "")
    }
}

impl Clone for GridState {
    fn clone(&self) -> GridState {
        let mut new_grid = GridState::new();
        new_grid.turn = self.turn;
        for i in 0..6 {
            for j in 0..9 {
                new_grid.grid[i][j] = self.grid[i][j];
            }
        }
        new_grid
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_row_win() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(8).unwrap();
        grid.insert(1).unwrap();
        grid.insert(7).unwrap();
        grid.insert(2).unwrap();
        grid.insert(6).unwrap();
        grid.insert(3).unwrap();
        assert_eq!(grid.winner.unwrap(), Player::Yellow);
    }
    
    #[test]
    fn test_cannot_insert_after_win() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(8).unwrap();
        grid.insert(1).unwrap();
        grid.insert(7).unwrap();
        grid.insert(2).unwrap();
        grid.insert(6).unwrap();
        grid.insert(3).unwrap();
        assert!(grid.insert(5).is_err());
    }

    #[test]
    fn test_detect_column_win() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(8).unwrap();
        grid.insert(0).unwrap();
        grid.insert(7).unwrap();
        grid.insert(0).unwrap();
        grid.insert(6).unwrap();
        grid.insert(0).unwrap();
        assert_eq!(grid.winner.unwrap(), Player::Yellow);
    }

    #[test]
    fn test_no_early_winner() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(8).unwrap();
        grid.insert(0).unwrap();
        grid.insert(7).unwrap();
        grid.insert(0).unwrap();
        grid.insert(6).unwrap();
        assert_eq!(grid.winner, None);
    }
}
