use super::{piece::Piece, square::Square};

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct CMove {
    pub from_piece: Piece,
    pub from: Square,
    pub to_piece: Piece,
    pub to: Square,

    // Used to disambiguate when looking at notation
    disambig: String,
}

impl CMove {
    pub fn new(from: Square, to: Square) -> Self {
        let disambig = String::from("");
        let from_piece = from.piece.expect("Trying to move a blank");
        let to_piece = to.piece.expect("Trying to capture a blank");
        CMove {
            from_piece,
            from,
            to_piece,
            to,
            disambig,
        }
    }

    pub fn notation(&self) -> String {
        let piece_qualifier = match &self.from_piece {
            Piece::Pawn => self.from.file_notation(),
            p => p.notation(),
        };
        format!(
            "{}{}x{}",
            piece_qualifier,
            self.disambig,
            self.to.notation()
        )
    }
}
