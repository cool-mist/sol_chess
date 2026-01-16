pub mod cmove;
mod constants;
pub mod errors;
pub mod piece;
pub mod square;

use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    mem,
};

use cmove::CMove;
use constants::BOARD_SIZE;
use errors::SError;
use piece::Piece;
use square::{Square, SquarePair};

use crate::util;

#[derive(Clone)]
pub struct Board {
    pub cells: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub legal_moves: HashSet<CMove>,
    pub game_state: BoardState,
    pieces_remaining: u8,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BoardState {
    NotStarted,
    InProgress,
    Lost,
    Won,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
            legal_moves: HashSet::new(),
            pieces_remaining: 0,
            game_state: BoardState::NotStarted,
        }
    }

    pub fn from_id(board_id: &str) -> Result<Self, SError> {
        let mut board_id_bytes = [0; 8];
        board_id_bytes.copy_from_slice(board_id.as_bytes());
        let mut working_bytes_slice = [0; 6];
        util::b64_decode_48(&board_id_bytes, &mut working_bytes_slice);

        let mut working_bytes = [0; 8];
        working_bytes[2..].copy_from_slice(&working_bytes_slice);
        let mut working = u64::from_be_bytes(working_bytes);

        let mut board = Board::new();
        let mask = 0b111;
        for i in (0..BOARD_SIZE).rev() {
            for j in (0..BOARD_SIZE).rev() {
                let piece = Board::get_piece_from_encoding((working & mask) as u8);
                working = working >> 3;
                let piece = piece?;
                board.set(Square::new(i, j, piece));
            }
        }
        Ok(board)
    }

    pub fn from_string(board_string: String) -> Result<Self, SError> {
        if board_string.chars().count() != 16 {
            return Err(SError::InvalidBoard);
        }

        let mut board = Board::new();
        let mut chars = board_string.chars();
        for r in 0..BOARD_SIZE {
            for f in 0..BOARD_SIZE {
                let c = chars.next().unwrap();
                let piece = match c {
                    'K' => Piece::King,
                    'Q' => Piece::Queen,
                    'B' => Piece::Bishop,
                    'N' => Piece::Knight,
                    'R' => Piece::Rook,
                    'P' => Piece::Pawn,
                    '.' => continue,
                    _ => return Err(SError::InvalidBoard),
                };

                let square = Square::new(f, r, Some(piece));
                board.set(square);
            }
        }
        Ok(board)
    }

    pub fn set(&mut self, square: Square) -> Option<Piece> {
        let new_is_occuppied = square.piece.is_some();
        let existing = mem::replace(&mut self.cells[square.file][square.rank], square.piece);

        // If placing a piece on a blank, increment piece count
        if existing.is_none() && new_is_occuppied {
            self.pieces_remaining += 1;
        }

        // If placing a blank on a piece, decrement piece count
        if existing.is_some() && !new_is_occuppied {
            self.pieces_remaining -= 1;
        }

        self.board_state_changed();
        existing
    }

    pub fn make_move(&mut self, mv: CMove) -> Option<CMove> {
        if !self.legal_moves.contains(&mv) {
            println!("Invalid move - {}", mv.notation());
            println!("Legal moves - ");
            for m in &self.legal_moves {
                println!("{}", m.notation());
            }
            return None;
        }

        let from_piece = mem::replace(&mut self.cells[mv.from.file][mv.from.rank], None);
        self.cells[mv.to.file][mv.to.rank] = from_piece;

        self.pieces_remaining -= 1;
        self.board_state_changed();
        Some(mv)
    }

    pub fn empty_squares(&self) -> Vec<Square> {
        let mut empty_squares = Vec::new();
        for file in 0..BOARD_SIZE {
            for rank in 0..BOARD_SIZE {
                if self.cells[file][rank].is_none() {
                    empty_squares.push(Square::new(file, rank, None));
                }
            }
        }
        empty_squares
    }

    pub fn pretty_print(&self) {
        println!("{}", self.print(true));
        // println!("{:^40}\n", format!("id: {:#018x}", self.id()));
        println!("{:^40}\n", format!("id: {}", self.id()));
    }

    pub fn id(&self) -> String {
        let mut res: u64 = 0;

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                res = res << 3;
                let byte = Board::get_piece_encoding(self.cells[i][j]);
                res = res | byte as u64
            }
        }

        let mut id_bytes = [0; 6];
        id_bytes.copy_from_slice(&res.to_be_bytes()[2..]);
        util::b64_encode_48(&id_bytes)
    }

    fn print(&self, pretty: bool) -> String {
        let mut board_string = String::new();
        for rank in 0..BOARD_SIZE {
            let mut row = String::new();
            for file in 0..BOARD_SIZE {
                let piece = self.cells[file][rank];
                row.push_str(&get_square_for_display(&piece, pretty));
            }

            if pretty {
                board_string.push_str(&format!("{:^40}\n", row));
            } else {
                board_string.push_str(&row);
            }

            board_string.push('\n');
        }

        board_string
    }

    fn calc_legal_moves(&mut self) {
        self.legal_moves = self
            .all_possible_move_pairs()
            .into_iter()
            .filter(SquarePair::is_different)
            .filter_map(|pair| self.is_legal_move(pair))
            .collect()
    }

    fn is_legal_move(&self, pair: SquarePair) -> Option<CMove> {
        // The below block is just to make the compiler happy. Start will always
        // have a piece
        let Some(piece) = pair.start.piece else {
            return None;
        };

        let legal = match piece {
            Piece::King => self.is_king_legal(&pair),
            Piece::Queen => self.is_queen_legal(&pair),
            Piece::Bishop => self.is_bishop_legal(&pair),
            Piece::Knight => self.is_knight_legal(&pair),
            Piece::Rook => self.is_rook_legal(&pair),
            Piece::Pawn => self.is_pawn_legal(&pair),
        };

        if legal {
            return Some(CMove::new(pair.start, pair.end));
        }

        None
    }

    fn is_king_legal(&self, pair: &SquarePair) -> bool {
        pair.dx <= 1 && pair.dy <= 1
    }

    fn is_queen_legal(&self, pair: &SquarePair) -> bool {
        self.is_path_free(pair)
    }

    fn is_bishop_legal(&self, pair: &SquarePair) -> bool {
        pair.dx == pair.dy && self.is_path_free(pair)
    }

    fn is_knight_legal(&self, pair: &SquarePair) -> bool {
        (pair.dx == 2 && pair.dy == 1) || (pair.dx == 1 && pair.dy == 2)
    }

    fn is_rook_legal(&self, pair: &SquarePair) -> bool {
        if pair.dx != 0 && pair.dy != 0 {
            return false;
        }

        self.is_path_free(pair)
    }

    fn is_pawn_legal(&self, pair: &SquarePair) -> bool {
        pair.dx == 1 && pair.dy == 1 && pair.y_dir == -1
    }

    fn is_path_free(&self, pair: &SquarePair) -> bool {
        // There is no straight line or diagonal to get through
        if pair.dx != pair.dy && pair.dx != 0 && pair.dy != 0 {
            return false;
        }

        let x_inc = pair.x_dir;
        let y_inc = pair.y_dir;
        let mut x: i8 = pair.start.file.try_into().unwrap();
        let mut y: i8 = pair.start.rank.try_into().unwrap();

        loop {
            x = x + x_inc;
            y = y + y_inc;

            let file: usize = x.try_into().unwrap();
            let rank: usize = y.try_into().unwrap();
            if rank == pair.end.rank && file == pair.end.file {
                return true;
            }

            if self.cells[file][rank].is_some() {
                return false;
            }
        }
    }

    fn calc_game_state(&mut self) {
        self.game_state = if self.pieces_remaining == 0 {
            BoardState::NotStarted
        } else if self.pieces_remaining == 1 {
            BoardState::Won
        } else if self.legal_moves.is_empty() {
            BoardState::Lost
        } else {
            BoardState::InProgress
        }
    }

    /// This is just a cartesian product of {occupied_squares} x {occupied_squares}
    fn all_possible_move_pairs(&self) -> impl IntoIterator<Item = SquarePair> {
        let ret = self
            .all_occupied_squares()
            .into_iter()
            .map(|start| {
                self.all_occupied_squares()
                    .into_iter()
                    .map(move |end| SquarePair::new(start.clone(), end))
            })
            .flatten()
            .collect::<Vec<SquarePair>>();

        return ret;
    }

    fn all_occupied_squares(&self) -> impl IntoIterator<Item = Square> {
        let mut ret = Vec::new();

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let p = &self.cells[i][j];
                if p.is_some() {
                    ret.push(Square::new(i, j, *p))
                }
            }
        }

        ret
    }

    fn board_state_changed(&mut self) {
        self.calc_legal_moves();
        self.calc_game_state();
    }

    fn get_piece_encoding(piece: Option<Piece>) -> u8 {
        match piece {
            Some(p) => match p {
                Piece::King => 0b001,
                Piece::Queen => 0b010,
                Piece::Rook => 0b011,
                Piece::Bishop => 0b100,
                Piece::Knight => 0b101,
                Piece::Pawn => 0b110,
            },
            None => 0b000,
        }
    }

    fn get_piece_from_encoding(encoding: u8) -> Result<Option<Piece>, SError> {
        match encoding {
            0b001 => Ok(Some(Piece::King)),
            0b010 => Ok(Some(Piece::Queen)),
            0b011 => Ok(Some(Piece::Rook)),
            0b100 => Ok(Some(Piece::Bishop)),
            0b101 => Ok(Some(Piece::Knight)),
            0b110 => Ok(Some(Piece::Pawn)),
            0b000 => Ok(None),
            _ => Err(SError::InvalidBoard),
        }
    }
}

fn get_square_for_display(piece: &Option<Piece>, pretty: bool) -> String {
    let contents = if let Some(piece) = piece {
        if pretty {
            piece.pretty()
        } else {
            piece.notation()
        }
    } else {
        ".".to_string()
    };

    if pretty {
        format!(" {} ", contents)
    } else {
        contents
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let display = match self {
            BoardState::NotStarted => "Not Started",
            BoardState::InProgress => "In Progress",
            BoardState::Lost => "Lost",
            BoardState::Won => "Won",
        };

        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sq {
        ($sq:literal) => {
            Square::parse($sq)
        };
    }

    macro_rules! mv {
        ($from:literal, $to:literal) => {{
            CMove::new(sq!($from), sq!($to))
        }};
    }

    macro_rules! validate_board {
        ($board:expr, $row1:literal, $row2:literal, $row3:literal, $row4:literal) => {
            let printed = $board.print(false);
            assert_eq!(
                printed,
                format!("{}\n{}\n{}\n{}\n", $row1, $row2, $row3, $row4)
            );
        };
    }

    macro_rules! validate_legal_moves {
        ($board:expr, $($move:expr,)*) => {
            let mut legal_moves = $board.legal_moves.iter().map(|m| m.clone()).collect::<Vec<CMove>>();

            $(
                assert!(legal_moves.contains(&$move));
                let position = legal_moves.iter().position(|m| m == &$move).unwrap();
                legal_moves.remove(position);
            )*

            if (legal_moves.len() > 0) {
                println!("The following moves were not matched - ");
                for m in &legal_moves {
                    println!("{}", m.notation());
                }

                assert!(false);
            }
        };
    }

    #[test]
    fn test_board_place() {
        let mut board = Board::new();
        assert!(board.set(sq!("Ka1")).is_none());
        assert!(board.set(sq!("Qa2")).is_none());
        assert!(board.set(sq!("Bc3")).is_none());
        assert!(board.set(sq!("Nc4")).is_none());
        assert!(board.set(sq!("Rd1")).is_none());
        assert!(board.set(sq!("Pd4")).is_none());
        assert!(board.set(sq!("Nb2")).is_none());
        let existing = board.set(sq!("Pc4"));
        assert!(existing.is_some());
        assert_eq!(Piece::Knight, existing.unwrap());
        validate_board!(board, "..PP", "..B.", "QN..", "K..R");
    }

    #[test]
    fn test_legal_moves() {
        let mut board = Board::new();
        assert_eq!(0, board.pieces_remaining);
        assert_eq!(0, board.legal_moves.len());
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_none());

        board.set(sq!("Qa4"));
        board.set(sq!("Ka2"));
        board.set(sq!("Pa1"));
        board.set(sq!("Pb3"));
        board.set(sq!("Rb2"));
        board.set(sq!("Pc4"));
        board.set(sq!("Kc3"));
        board.set(sq!("Bc1"));
        board.set(sq!("Bd2"));
        board.set(sq!("Nd1"));

        assert_eq!(10, board.pieces_remaining);

        board.pretty_print();

        // Q . P .
        // . P K .
        // K R . B
        // P . B N
        validate_legal_moves!(
            board,
            mv!("Ka2", "Pa1"),
            mv!("Ka2", "Rb2"),
            mv!("Ka2", "Pb3"),
            mv!("Kc3", "Rb2"),
            mv!("Kc3", "Pb3"),
            mv!("Kc3", "Pc4"),
            mv!("Kc3", "Bd2"),
            mv!("Pa1", "Rb2"),
            mv!("Pb3", "Pc4"),
            mv!("Pb3", "Qa4"),
            mv!("Qa4", "Ka2"),
            mv!("Qa4", "Pb3"),
            mv!("Qa4", "Pc4"),
            mv!("Rb2", "Ka2"),
            mv!("Rb2", "Pb3"),
            mv!("Rb2", "Bd2"),
            mv!("Bc1", "Rb2"),
            mv!("Bc1", "Bd2"),
            mv!("Bd2", "Kc3"),
            mv!("Bd2", "Bc1"),
            mv!("Nd1", "Rb2"),
            mv!("Nd1", "Kc3"),
        );

        assert_eq!(10, board.pieces_remaining);

        // Validate some illegal moves
        assert!(board.make_move(mv!("Ka2", "Pa2")).is_none());
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_none());

        board.set(sq!(".b2"));
        board.set(sq!(".c4"));
        board.set(sq!("Rc1"));

        // Q . . .
        // . P K .
        // K . . B
        // P . R N
        validate_legal_moves!(
            board,
            mv!("Ka2", "Pa1"),
            mv!("Ka2", "Pb3"),
            mv!("Kc3", "Pb3"),
            mv!("Kc3", "Bd2"),
            mv!("Pb3", "Qa4"),
            mv!("Bd2", "Kc3"),
            mv!("Bd2", "Rc1"),
            mv!("Qa4", "Ka2"),
            mv!("Qa4", "Pb3"),
            mv!("Rc1", "Pa1"),
            mv!("Rc1", "Kc3"),
            mv!("Rc1", "Nd1"),
            mv!("Nd1", "Kc3"),
        );

        assert_eq!(8, board.pieces_remaining);
    }

    #[test]
    fn test_smoke_puzzle() {
        let mut board = Board::new();
        assert_eq!(BoardState::NotStarted, board.game_state);
        assert_eq!(0, board.pieces_remaining);

        // K . . .
        // . P . .
        // . . R .
        // N . . .
        board.set(sq!("Ka4"));
        assert_eq!(BoardState::Won, board.game_state);

        board.set(sq!("Pb3"));
        board.set(sq!("Rc2"));
        board.set(sq!("Na1"));

        assert_eq!(BoardState::InProgress, board.game_state);
        assert_eq!(4, board.pieces_remaining);

        assert!(board.make_move(mv!("Na1", "Rc2")).is_some());
        assert_eq!(3, board.pieces_remaining);
        assert_eq!(BoardState::InProgress, board.game_state);

        assert!(board.make_move(mv!("Pb3", "Ka4")).is_some());
        assert_eq!(2, board.pieces_remaining);
        assert_eq!(BoardState::Lost, board.game_state);

        // P . . .
        // . . . .
        // . . N .
        // . . . .

        board.set(sq!("Pa1"));
        board.set(sq!("Qa3"));

        // P . . .
        // Q . . .
        // . . N .
        // P . . .
        assert_eq!(4, board.pieces_remaining);
        assert_eq!(BoardState::InProgress, board.game_state);

        board.make_move(mv!("Qa3", "Pa4"));
        board.make_move(mv!("Nc2", "Pa1"));
        assert_eq!(2, board.pieces_remaining);
        assert_eq!(BoardState::InProgress, board.game_state);

        // Q . . .
        // . . . .
        // . . . .
        // N . . .
        board.make_move(mv!("Qa4", "Na1"));
        assert_eq!(1, board.pieces_remaining);
        assert_eq!(BoardState::Won, board.game_state);
    }

    #[test]
    fn test_encoding() {
        let mut board = Board::new();
        board.set(sq!("Pa1"));
        board.set(sq!("Ra2"));
        board.set(sq!("Qb2"));
        board.set(sq!("Kd2"));
        board.set(sq!("Bd4"));
        board.set(sq!("Nc4"));

        let id = board.id();
        let board2 = Board::from_id(&id);
        let board2 = board2.unwrap();

        validate_board!(board2, "..NB", "....", "RQ.K", "P...");
    }
}
