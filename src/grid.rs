// Data definitions for the connect4 grid state

#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Yellow,
    Red
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    owner: Player
}

#[derive(PartialEq)]
pub struct GridState {
    grid: [[Option<Tile>; 9]; 6],
    pub turn: Player,
    pub winner: Option<Player>
}

pub struct GameMove {
    pub previous: GridState,
    pub next: GridState
}

pub struct Game {
    moves: Vec<GameMove>
}

impl GridState {
    pub fn new() -> GridState {
        let empty_grid: [[Option<Tile>; 9]; 6] = Default::default();
        return GridState {grid: empty_grid, turn: Player::Yellow, winner: None};
    }

    pub fn get_moves(&self) -> Vec<GameMove> {
        let mut move_list: Vec<GameMove> = Vec::new();
        for i in 0..9 {
            let previous = self.clone();
            let mut next = self.clone();
            if let Ok(_) = next.insert(i) {
                move_list.push(GameMove {previous: previous, next: next});
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

        let new_tile: Tile = Tile {owner: self.turn};

        if GridState::check_winner(&self, free_position as i32, column as i32) {
            self.winner = Some(self.turn);
        }

        self.grid[free_position][column] = Some(new_tile);
        self.turn = match self.turn {
            Player::Yellow => Player::Red,
            Player::Red    => Player::Yellow
        };
        Ok(())
    }

    fn check_winner(&self, new_row: i32, new_column: i32) -> bool {
        //TODO: Candidate for macros
        let last_player = self.turn;

        // Check Horizontal
        let mut row_count = 0;
        for i in 0..3 {
            if new_column + i + 1 >= 0 && new_column + i + 1 < 9  { 
                if let Some(x) = self.grid[new_row as usize][(new_column + i + 1) as usize] {
                    if x.owner == last_player {row_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        for i in 0..3 {
            if new_column - i - 1 >= 0 && new_column - i - 1 < 9  { 
                if let Some(x) = self.grid[new_row as usize][(new_column - i - 1) as usize] {
                    if x.owner == last_player {row_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        
        // Check Vertical
        let mut vert_count = 0;
        for i in 0..3 {
            if new_row + i + 1 >= 0 && new_row + i + 1 < 6  { 
                if let Some(x) = self.grid[(new_row + i + 1) as usize][(new_column) as usize] {
                    if x.owner == last_player {vert_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        for i in 0..3 {
            if new_row - i - 1 >= 0 && new_row - i - 1 < 6  { 
                if let Some(x) = self.grid[(new_row - i - 1) as usize][(new_column) as usize] {
                    if x.owner == last_player {vert_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        
        // Check Rising Diagonal
        let mut rising_count = 0;
        for i in 0..3 {
            if new_column + i + 1 >= 0 && new_column + i + 1 < 9  && 
               new_row - i - 1 >= 0 && new_row - i - 1 < 6  { 
                if let Some(x) = self.grid[(new_row - i - 1) as usize][(new_column + i + 1) as usize] {
                    if x.owner == last_player {rising_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        for i in 0..3 {
            if new_column - i - 1 >= 0 && new_column - i - 1 < 9  && 
               new_row + i + 1 >= 0 && new_row + i + 1 < 6  { 
                if let Some(x) = self.grid[(new_row + i + 1) as usize][(new_column - i - 1) as usize] {
                    if x.owner == last_player {rising_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        
        // Check Falling Diagonal
        let mut falling_count = 0;
        for i in 0..3 {
            if new_column + i + 1 >= 0 && new_column + i + 1 < 9  && 
               new_row + i + 1 >= 0 && new_row + i + 1 < 6  { 
                if let Some(x) = self.grid[(new_row + i + 1) as usize][(new_column + i + 1) as usize] {
                    if x.owner == last_player {falling_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }
        for i in 0..3 {
            if new_column - i - 1 >= 0 && new_column - i - 1 < 9  && 
               new_row - i - 1 >= 0 && new_row - i - 1 < 6  { 
                if let Some(x) = self.grid[(new_row - i - 1) as usize][(new_column - i - 1) as usize] {
                    if x.owner == last_player {falling_count += 1;} else {break;}
                } else {break;}
            } else {
                break;
            }
        }

        //println!("Hor: {} | Vert: {} | Rising: {} | Falling: {}", row_count, vert_count, rising_count, falling_count);
        row_count >= 3 || vert_count >= 3 || rising_count >= 3 || falling_count >= 3
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

impl fmt::Debug for GridState {
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

/*
impl PartialEq for GridState {
    fn eq(&self, other: &GridState) -> bool {
        for (row1, row2) in self.grid.iter().zip(other.grid.iter()) {
            for (val1, val2) in row1.iter().zip(row2.iter()) {
                let is_same = match (val1, val2) {
                    (Some(x), Some(y)) => x == y,
                    (None, Some(_)) => false,
                    (Some(_), None) => false,
                    (None, None) => true
                };
                if !is_same {return false;}
            }
        }
        self.turn == other.turn &&
        self.winner == other.winner
    }
}
*/

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
        new_grid.winner = self.winner;
        for i in 0..6 {
            for j in 0..9 {
                new_grid.grid[i][j] = self.grid[i][j];
            }
        }
        new_grid
    }
}

impl GameMove {
    pub fn new() -> GameMove {
        GameMove { 
            previous: GridState::new(),
            next: GridState::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_row_win() {
        let mut grid = GridState::new();
        grid.insert(1).unwrap();
        grid.insert(8).unwrap();
        grid.insert(2).unwrap();
        grid.insert(7).unwrap();
        grid.insert(3).unwrap();
        grid.insert(6).unwrap();
        grid.insert(0).unwrap();
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
    fn test_detect_rising_win() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(1).unwrap();
        grid.insert(1).unwrap();
        grid.insert(2).unwrap();
        grid.insert(3).unwrap();
        grid.insert(2).unwrap();
        grid.insert(2).unwrap();
        grid.insert(3).unwrap();
        grid.insert(4).unwrap();
        grid.insert(3).unwrap();
        grid.insert(3).unwrap();
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
    
    #[test]
    fn test_no_moves_after_win() {
        let mut grid = GridState::new();
        grid.insert(0).unwrap();
        grid.insert(8).unwrap();
        grid.insert(0).unwrap();
        grid.insert(7).unwrap();
        grid.insert(0).unwrap();
        grid.insert(6).unwrap();
        grid.insert(0).unwrap();
        assert_eq!(grid.get_moves().len(), 0);
    }
}
