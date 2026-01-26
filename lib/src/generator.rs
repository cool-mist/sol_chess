use std::fmt::Display;

use crate::board::{
    Board, BoardOptions,
    cmove::CMove,
    piece::{Piece, PieceKind},
};

pub trait RandomRange {
    fn gen_range(&self, min: usize, max: usize) -> usize;
}

#[derive(Default)]
pub struct Puzzle {
    pub board: Board,
    pub solutions: Vec<Vec<CMove>>,
    pub solved: bool,
}

pub struct GenerateStats {
    piece_total: u32,
    piece_success: u32,
    total: u32,
    board: Option<Board>,
    solutions: Vec<Vec<CMove>>,
}

pub fn generate_weighted_random(
    num_pieces: u32,
    num_solutions: u32,
    max_moves_per_piece: u32,
    rand: &impl RandomRange,
) -> GenerateStats {
    let candidate_pieces = vec![
        PieceKind::Pawn,
        PieceKind::Pawn,
        PieceKind::Pawn,
        PieceKind::Pawn,
        PieceKind::Bishop,
        PieceKind::Bishop,
        PieceKind::Bishop,
        PieceKind::Bishop,
        PieceKind::Knight,
        PieceKind::Knight,
        PieceKind::Knight,
        PieceKind::Queen,
        PieceKind::Rook,
        PieceKind::Rook,
    ];

    if num_pieces > candidate_pieces.len().try_into().unwrap() {
        panic!(
            "Number of pieces to place on the board should be <= {}",
            candidate_pieces.len()
        );
    }

    let attempts: u32 = 1000;
    let mut overall_stats = GenerateStats::new(0, 0, 0, None, vec![]);
    for _ in 0..attempts {
        let stats = try_generate(
            num_pieces,
            num_solutions,
            max_moves_per_piece,
            rand,
            candidate_pieces.clone(),
        );
        overall_stats.piece_total += stats.piece_total;
        overall_stats.piece_success += stats.piece_success;
        overall_stats.total += stats.total;
        overall_stats.board = stats.board;
        if overall_stats.board.is_some() {
            return overall_stats;
        }
    }

    overall_stats
}

impl GenerateStats {
    fn new(
        piece_total: u32,
        piece_success: u32,
        total: u32,
        board: Option<Board>,
        solutions: Vec<Vec<CMove>>,
    ) -> Self {
        Self {
            piece_total,
            piece_success,
            total,
            board,
            solutions,
        }
    }

    pub fn print_stats(&self) {
        let mut stats = String::new();
        add_stat(&mut stats, "Total attempts", self.total);
        add_stat(&mut stats, "Total pieces placed", self.piece_total);
        add_stat(&mut stats, "Success pieces placed", self.piece_success);

        println!("{}", stats);
    }

    pub fn puzzle(self) -> Option<Puzzle> {
        let Some(board) = self.board else {
            return None;
        };

        let solved = self.solutions.len() > 0;

        Some(Puzzle {
            board,
            solutions: self.solutions,
            solved,
        })
    }
}

fn add_stat<T>(stats: &mut String, name: &str, val: T)
where
    T: Display,
{
    stats.push_str(&format!("{:>30}:{:>6}\n", name, val));
}

fn try_generate(
    num_pieces: u32,
    num_solutions: u32,
    max_moves_per_piece: u32,
    rand: &impl RandomRange,
    mut candidate_pieces: Vec<PieceKind>,
) -> GenerateStats {
    let mut board = Board::create(BoardOptions {
        max_moves_per_piece,
    });

    let mut piece_total = 0;
    let mut piece_success = 0;
    for _ in 0..num_pieces {
        let mut placed = false;
        let empty_squares = board.empty_squares();
        let mut attempts = 15;
        while !placed {
            if attempts == 0 {
                return GenerateStats::new(piece_total, piece_success, 1, None, vec![]);
            }

            attempts -= 1;
            piece_total += 1;

            let index = rand.gen_range(0, candidate_pieces.len());
            let piece_kind = candidate_pieces[index];
            let square_index = rand.gen_range(0, empty_squares.len());
            let mut random_square = empty_squares[square_index].clone();
            random_square.piece = Some(Piece::new(piece_kind));
            board.set(random_square.clone());
            let puzzle = board.solve();
            if puzzle.solutions.len() > 0 {
                placed = true;
                piece_success += 1;
                candidate_pieces.remove(index);
                continue;
            }

            random_square.piece = None;
            board.set(random_square);
        }
    }

    let puzzle = board.solve();
    if puzzle.solutions.len() > num_solutions as usize {
        GenerateStats::new(piece_total, piece_success, 1, None, vec![])
    } else {
        GenerateStats::new(
            piece_total,
            piece_success,
            1,
            Some(puzzle.board),
            puzzle.solutions,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::board::BoardState;

    use super::*;

    use rand::Rng;

    struct TestRandom;
    impl RandomRange for TestRandom {
        fn gen_range(&self, min: usize, max: usize) -> usize {
            rand::rng().random_range(min..max)
        }
    }

    #[test]
    fn generator_smoke() {
        for _ in 0..10 {
            let gen_stats = generate_weighted_random(5, 5, 10, &TestRandom);
            let board = gen_stats.board.expect("No puzzle was generated");
            assert_eq!(board.game_state, BoardState::InProgress);

            let puzzle = board.solve();
            assert!(puzzle.solutions.len() <= 5);
            assert!(puzzle.solutions.len() >= 1);
        }
    }
}
