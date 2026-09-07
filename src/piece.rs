#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    #[allow(dead_code)]
    pub fn opposite(self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn as_char(self) -> char {
        match self {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(color: PieceColor, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    pub fn as_fen_char(self) -> char {
        if self.color == PieceColor::White {
            self.kind.as_char().to_ascii_uppercase()
        } else {
            self.kind.as_char()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PieceColor::*, PieceKind::*, *};

    #[test]
    fn opposite_flips_color() {
        assert_eq!(White.opposite(), Black);
        assert_eq!(Black.opposite(), White);
    }

    #[test]
    fn opposite_applied_twice_is_itself() {
        assert_eq!(White.opposite().opposite(), White);
        assert_eq!(Black.opposite().opposite(), Black);
    }

    #[test]
    fn kind_as_char_is_lowercase() {
        assert_eq!(Pawn.as_char(), 'p');
        assert_eq!(Knight.as_char(), 'n');
        assert_eq!(Bishop.as_char(), 'b');
        assert_eq!(Rook.as_char(), 'r');
        assert_eq!(Queen.as_char(), 'q');
        assert_eq!(King.as_char(), 'k');
    }

    #[test]
    fn new_piece_constructs_correctly() {
        assert_eq!(
            Piece::new(White, Pawn),
            Piece {
                color: White,
                kind: Pawn
            }
        );
        assert_eq!(
            Piece::new(Black, Queen),
            Piece {
                color: Black,
                kind: Queen
            }
        );
    }

    #[test]
    fn as_fen_char_is_cased_by_color() {
        let cases = [
            (White, Pawn, 'P'),
            (White, Knight, 'N'),
            (White, Bishop, 'B'),
            (White, Rook, 'R'),
            (White, Queen, 'Q'),
            (White, King, 'K'),
            (Black, Pawn, 'p'),
            (Black, Knight, 'n'),
            (Black, Bishop, 'b'),
            (Black, Rook, 'r'),
            (Black, Queen, 'q'),
            (Black, King, 'k'),
        ];

        for (color, kind, expected) in cases {
            let piece = Piece::new(color, kind);
            assert_eq!(
                piece.as_fen_char(),
                expected,
                "failed for {:?} {:?}",
                color,
                kind
            );
        }
    }

    #[test]
    fn pieces_compare_equal() {
        let white_pawn_a = Piece::new(White, Pawn);
        let white_pawn_b = Piece::new(White, Pawn);
        let black_pawn = Piece::new(Black, Pawn);

        assert_eq!(white_pawn_a, white_pawn_b);
        assert_ne!(white_pawn_a, black_pawn);
    }
}
