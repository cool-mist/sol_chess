#[derive(Clone, Eq, Hash, Copy, Debug, PartialEq)]
pub(crate) enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl Piece {
    pub(crate) fn parse(piece: &str) -> Option<Self> {
        match piece {
            "K" => Some(Piece::King),
            "Q" => Some(Piece::Queen),
            "B" => Some(Piece::Bishop),
            "N" => Some(Piece::Knight),
            "R" => Some(Piece::Rook),
            "P" => Some(Piece::Pawn),
            "." => None,
            p => panic!("Invalid piece {}", p),
        }
    }

    pub(crate) fn notation(&self) -> String {
        let n = match self {
            Piece::King => "K",
            Piece::Queen => "Q",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::Rook => "R",
            Piece::Pawn => "P",
        };

        n.to_string()
    }

    pub(crate) fn pretty(&self) -> String {
        let n = match self {
            Piece::King => "♔",
            Piece::Queen => "♕",
            Piece::Bishop => "♗",
            Piece::Knight => "♘",
            Piece::Rook => "♖",
            Piece::Pawn => "♙",
        };

        n.to_string()
    }
}

macro_rules! p {
    ($piece:literal) => {
        Piece::parse($piece)
    };
}

pub(crate) use p;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_parse() {
        assert_eq!(p!("K"), Some(Piece::King));
        assert_eq!(p!("Q"), Some(Piece::Queen));
        assert_eq!(p!("B"), Some(Piece::Bishop));
        assert_eq!(p!("N"), Some(Piece::Knight));
        assert_eq!(p!("R"), Some(Piece::Rook));
        assert_eq!(p!("P"), Some(Piece::Pawn));
    }
}
