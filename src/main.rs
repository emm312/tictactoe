
use std::io::{stdout, Write};

use tictactoe::ai::find_best_move;
use tictactoe::board::{Board, CellState};

use tictactoe::*;

use ansi_escapes::{CursorUp, EraseLine, CursorDown};

fn main() {
    let mut board = Board::new();
    let state = CellState::TakenX;
    loop {
        println!("{}", board);
        print!("{}", EraseLine);
        if let Some(n) = board.is_winner() {
            println!("{}", CursorDown(7));
            println!("{}", n);
            break;
        }
        print!("Your move: ");
        stdout().flush().unwrap();
        let input = input().trim().to_lowercase().chars().collect::<Vec<char>>();
        print!("{}", CursorUp(7));
        if input.len() != 2 { println!("Invalid input."); continue; }
        let x;
        let y;
        match input[0] {
            'a' => { y = 0; }
            'b' => { y = 1; }
            'c' => { y = 2; },
            _ => { println!("Invalid input."); continue; }
        }
        x = input[1].to_string().parse::<usize>().unwrap().clamp(1, 3)-1;
        board.set_square(x, y, state);
        let best_move = find_best_move(&board);
        board.set_square(best_move.1, best_move.0, CellState::TakenO);
    }
}
