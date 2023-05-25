use crate::board::{Board, CellState};

pub fn minimax(board: &Board, maximise: bool) -> i8 {
    if let Some(w) = board.is_winner() {
        return w.to_score();
    }

    let mut scores = Vec::new();

    let mut new_board = board.clone();

    for poss_move in new_board.get_free_squares() {
        new_board.set_square(poss_move.0, poss_move.1, CellState::from_bool(maximise));
        scores.push(minimax(&new_board, !maximise));
    }
    if maximise { *scores.iter().max().unwrap() } else { *scores.iter().min().unwrap() }
}

pub fn find_best_move(board: &Board) -> (usize, usize) {
    let mut max_score = (0, (0, 0));
    let mut new_board = board.clone();
    for moves in board.get_free_squares() {
        new_board.set_square(moves.0, moves.1, CellState::TakenO);
        let val = minimax(&new_board, true);
        if max_score.0 > val {
            max_score.0 = val;
            max_score.1 = moves;
        }
    }
    max_score.1
}