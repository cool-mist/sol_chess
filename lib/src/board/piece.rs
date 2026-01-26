#[derive(Clone, Eq, Hash, Copy, Debug, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub moves_made: u32,
    pub active: bool,
}
impl Piece {
    pub fn new(kind: PieceKind) -> Self {
        Self {
            kind,
            moves_made: 0,
            active: true,
        }
    }

    pub fn from_kind(kind: Option<PieceKind>) -> Option<Self> {
        kind.map(|k| Piece::new(k))
    }
}

#[derive(Clone, Eq, Hash, Copy, Debug, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceKind {
    pub fn parse(piece: &str) -> Option<Self> {
        match piece {
            "K" => Some(PieceKind::King),
            "Q" => Some(PieceKind::Queen),
            "B" => Some(PieceKind::Bishop),
            "N" => Some(PieceKind::Knight),
            "R" => Some(PieceKind::Rook),
            "P" => Some(PieceKind::Pawn),
            "." => None,
            p => panic!("Invalid piece {}", p),
        }
    }

    pub fn notation(&self) -> String {
        let n = match self {
            PieceKind::King => "K",
            PieceKind::Queen => "Q",
            PieceKind::Bishop => "B",
            PieceKind::Knight => "N",
            PieceKind::Rook => "R",
            PieceKind::Pawn => "P",
        };

        n.to_string()
    }

    pub fn pretty(&self) -> String {
        let n = match self {
            PieceKind::King => "♔",
            PieceKind::Queen => "♕",
            PieceKind::Bishop => "♗",
            PieceKind::Knight => "♘",
            PieceKind::Rook => "♖",
            PieceKind::Pawn => "♙",
        };

        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! p {
        ($piece:literal) => {
            PieceKind::parse($piece)
        };
    }

    #[test]
    fn test_piece_parse() {
        assert_eq!(p!("K"), Some(PieceKind::King));
        assert_eq!(p!("Q"), Some(PieceKind::Queen));
        assert_eq!(p!("B"), Some(PieceKind::Bishop));
        assert_eq!(p!("N"), Some(PieceKind::Knight));
        assert_eq!(p!("R"), Some(PieceKind::Rook));
        assert_eq!(p!("P"), Some(PieceKind::Pawn));
    }
}
