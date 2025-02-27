use crate::board::{
    cmove::CMove,
    {Board, BoardState},
};

pub struct Solver {
    pub board: Board,
    moves: Vec<CMove>,
}

impl Solver {
    pub fn new(board: Board) -> Solver {
        Solver {
            board,
            moves: vec![],
        }
    }

    fn clone(&self, m: CMove) -> Self {
        let mut moves = self.moves.clone();
        let mut board = self.board.clone();
        moves.push(m.clone());
        board.make_move(m);
        Solver { board, moves }
    }

    pub fn solve(&self) -> Vec<Vec<CMove>> {
        let mut solutions = Vec::new();
        if let BoardState::Won = self.board.game_state {
            solutions.push(self.moves.clone());
            return solutions;
        }

        let BoardState::InProgress = self.board.game_state else {
            return solutions;
        };

        self.board.legal_moves.iter().for_each(|m| {
            let solver = self.clone(m.clone());
            let more_solutions = solver.solve();
            solutions.extend(more_solutions);
        });

        solutions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{square::Square, Board};

    macro_rules! sq {
        ($sq:literal) => {
            Square::parse($sq)
        };
    }

    #[test]
    fn solver_smoke() {
        let mut board = Board::new();
        // . R . .
        // R . . P
        // B . B N
        // P . N .

        board.set(sq!("Pa1"));
        board.set(sq!("Ba2"));
        board.set(sq!("Ra3"));
        board.set(sq!("Rb4"));
        board.set(sq!("Nc1"));
        board.set(sq!("Bc2"));
        board.set(sq!("Nd2"));
        board.set(sq!("Pd3"));

        let solver = Solver::new(board.clone());
        let solutions = solver.solve();

        for solution in solutions {
            let mut board = board.clone();
            solution
                .into_iter()
                .for_each(|m| assert!(board.make_move(m).is_some()));
            assert_eq!(BoardState::Won, board.game_state);
        }
    }

    #[test]
    fn solver_smoke_no_solution() {
        // . R . .
        // R . . .
        // B . B N
        // P . N .

        let mut board = Board::new();
        board.set(sq!("Pa1"));
        board.set(sq!("Ba2"));
        board.set(sq!("Ra3"));
        board.set(sq!("Rb4"));
        board.set(sq!("Nc1"));
        board.set(sq!("Bc2"));
        board.set(sq!("Nd2"));

        let solver = Solver::new(board.clone());
        let solutions = solver.solve();

        assert_eq!(0, solutions.len());
    }
}
