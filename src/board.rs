use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    TakenX,
    TakenO,
    Empty,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, " "),
            CellState::TakenO => write!(f, "O"),
            CellState::TakenX => write!(f, "X"),
        }
    }
}

impl CellState {
    pub fn invert(&self) -> CellState {
        match self {
            CellState::TakenO => CellState::TakenX,
            CellState::TakenX => CellState::TakenO,
            _ => *self
        }
    }

    pub fn to_winner(&self) -> Winner {
        match self {
            Self::TakenO => Winner::PlayerO,
            Self::TakenX => Winner::PlayerX,
            _ => Winner::NoWinner
        }
    }

    pub fn from_bool(val: bool) -> CellState {
        if val { CellState::TakenX } else { CellState::TakenO }
    }
}

pub enum Winner {
    PlayerX,
    PlayerO,
    NoWinner
}

impl Winner {
    pub fn to_score(&self) -> i8 {
        match self {
            Winner::NoWinner => 0,
            Winner::PlayerO => -10,
            Winner::PlayerX => 10
        }
    }
}

impl Display for Winner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Winner::PlayerX => write!(f, "X Wins!"),
            Winner::PlayerO => write!(f, "O Wins!"),
            _ => Ok(()),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<CellState>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a {} | {} | {}\n  ---------\nb {} | {} | {}\n  ---------\nc {} | {} | {}\n  1   2   3",
            self.board[0][0],
            self.board[1][0],
            self.board[2][0],
            self.board[0][1],
            self.board[1][1],
            self.board[2][1],
            self.board[0][2],
            self.board[1][2],
            self.board[2][2]
        )
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![CellState::Empty; 3]; 3]
        }
    }

    pub fn set_square(&mut self, x: usize, y: usize, state: CellState) {
        self.board[x][y] = state;
    }

    pub fn is_winner(&self) -> Option<Winner> {
        // check diagonals
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != CellState::Empty {
            return Some(self.board[0][0].to_winner());
        }
        if self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2] && self.board[2][0] != CellState::Empty {
            return Some(self.board[2][0].to_winner());
        }

        // horizontals
        for i in 0..=2 {
            if self.board[2][i] == self.board[1][i] && self.board[1][i] == self.board[0][i] && self.board[2][i] != CellState::Empty {
                return Some(self.board[2][i].to_winner());
            }
        }

        // verticals
        for i in 0..=2 {
            if self.board[i][2] == self.board[i][1] && self.board[i][1] == self.board[i][0] && self.board[i][0] != CellState::Empty {
                return Some(self.board[i][2].to_winner());
            }
        }

        // draw
        let mut is_draw = true;
        'outer: for i in 0..=2 {
            for j in 0..=2 {
                if self.board[i][j] == CellState::Empty {
                    is_draw = false;
                    break 'outer;
                }
            }
        }
        if is_draw {
            return Some(Winner::NoWinner);
        }
        None
    }

    pub fn get_square(&self, x: usize, y: usize) -> CellState {
        self.board[x][y]
    }

    pub fn get_free_squares(&self) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for i in 0..=2 {
            for j in 0..=2 {
                if self.board[i][j] == CellState::Empty {
                    ret.push((i, j));
                }
            }
        }
        ret
    }
}
