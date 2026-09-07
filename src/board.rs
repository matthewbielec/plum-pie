use crate::piece::{Piece, PieceColor, PieceKind};
use std::fmt;

pub fn file_rank_to_index(file: usize, rank: usize) -> usize {
    21 + file + (10 * rank)
}

#[allow(dead_code)]
pub fn index_to_file_rank(index: usize) -> (usize, usize) {
    let file = (index - 21) % 10;
    let rank = (index - 21) / 10;

    (file, rank)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    OffBoard,
    Occupied(Piece),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingAbility {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingAbility {
    pub fn all_true() -> CastlingAbility {
        CastlingAbility {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub piece_placement: [Square; 120],
    pub side_to_move: PieceColor,
    pub castling_ability: CastlingAbility,
    pub en_passant_target_square: Option<usize>,
    pub halfmove_clock: u16,
    pub fullmove_counter: u16,
}

impl Board {
    pub fn square_at(&self, index: usize) -> Square {
        self.piece_placement[index]
    }

    #[allow(dead_code)]
    pub fn set_square(&mut self, index: usize, square: Square) {
        self.piece_placement[index] = square;
    }

    pub fn start() -> Board {
        let mut squares = [Square::OffBoard; 120];
        let pieces_order = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, piece) in pieces_order.iter().copied().enumerate() {
            for rank in 0..8 {
                let index = file_rank_to_index(file, rank);

                match rank {
                    0 => squares[index] = Square::Occupied(Piece::new(PieceColor::White, piece)),
                    1 => {
                        squares[index] =
                            Square::Occupied(Piece::new(PieceColor::White, PieceKind::Pawn))
                    }
                    6 => {
                        squares[index] =
                            Square::Occupied(Piece::new(PieceColor::Black, PieceKind::Pawn))
                    }
                    7 => squares[index] = Square::Occupied(Piece::new(PieceColor::Black, piece)),
                    _ => squares[index] = Square::Empty,
                }
            }
        }

        Board {
            piece_placement: squares,
            side_to_move: PieceColor::White,
            castling_ability: CastlingAbility::all_true(),
            en_passant_target_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            let start = file_rank_to_index(0, rank);
            let line: String = (start..start + 8)
                .map(|i| self.square_at(i))
                .map(|sq| match sq {
                    Square::Occupied(p) => p.as_fen_char(),
                    Square::Empty => '.',
                    Square::OffBoard => unreachable!("OffBoard squares should not be displayed!"),
                })
                .collect();
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::PieceColor::{Black, White};
    use crate::piece::PieceKind::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[test]
    fn file_rank_to_index_returns_correct_index() {
        let a1 = file_rank_to_index(0, 0);
        let h1 = file_rank_to_index(7, 0);
        let a8 = file_rank_to_index(0, 7);
        let h8 = file_rank_to_index(7, 7);
        let e4 = file_rank_to_index(4, 3);
        let e5 = file_rank_to_index(4, 4);
        let d4 = file_rank_to_index(3, 3);
        let d5 = file_rank_to_index(3, 4);

        assert_eq!(a1, 21);
        assert_eq!(h1, 28);
        assert_eq!(a8, 91);
        assert_eq!(h8, 98);
        assert_eq!(e4, 55);
        assert_eq!(e5, 65);
        assert_eq!(d4, 54);
        assert_eq!(d5, 64);
    }

    #[test]
    fn index_to_file_rank_returns_correct_tuple() {
        let a1 = index_to_file_rank(21);
        let h1 = index_to_file_rank(28);
        let a8 = index_to_file_rank(91);
        let h8 = index_to_file_rank(98);
        let e4 = index_to_file_rank(55);
        let e5 = index_to_file_rank(65);
        let d4 = index_to_file_rank(54);
        let d5 = index_to_file_rank(64);

        assert_eq!(a1, (0, 0));
        assert_eq!(h1, (7, 0));
        assert_eq!(a8, (0, 7));
        assert_eq!(h8, (7, 7));
        assert_eq!(e4, (4, 3));
        assert_eq!(e5, (4, 4));
        assert_eq!(d4, (3, 3));
        assert_eq!(d5, (3, 4));
    }

    #[test]
    fn castling_ability_all_true_sets_all_to_true() {
        let actual = CastlingAbility::all_true();
        let expected = CastlingAbility {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn start_board_spot_checks() {
        let board = Board::start();

        assert_eq!(board.side_to_move, White);
        assert_eq!(board.castling_ability, CastlingAbility::all_true());
        assert_eq!(board.en_passant_target_square, None);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_counter, 1);

        assert_eq!(board.piece_placement[0], Square::OffBoard);
        assert_eq!(board.piece_placement[20], Square::OffBoard);
        assert_eq!(board.piece_placement[99], Square::OffBoard);
        assert_eq!(board.piece_placement[119], Square::OffBoard);

        assert_eq!(
            board.piece_placement[21],
            Square::Occupied(Piece::new(White, Rook))
        );
        assert_eq!(
            board.piece_placement[98],
            Square::Occupied(Piece::new(Black, Rook))
        );

        assert_eq!(
            board.piece_placement[22],
            Square::Occupied(Piece::new(White, Knight))
        );
        assert_eq!(
            board.piece_placement[97],
            Square::Occupied(Piece::new(Black, Knight))
        );

        assert_eq!(
            board.piece_placement[23],
            Square::Occupied(Piece::new(White, Bishop))
        );
        assert_eq!(
            board.piece_placement[96],
            Square::Occupied(Piece::new(Black, Bishop))
        );

        assert_eq!(
            board.piece_placement[24],
            Square::Occupied(Piece::new(White, Queen))
        );
        assert_eq!(
            board.piece_placement[95],
            Square::Occupied(Piece::new(Black, King))
        );

        assert_eq!(
            board.piece_placement[25],
            Square::Occupied(Piece::new(White, King))
        );
        assert_eq!(
            board.piece_placement[94],
            Square::Occupied(Piece::new(Black, Queen))
        );

        assert_eq!(
            board.piece_placement[38],
            Square::Occupied(Piece::new(White, Pawn))
        );
        assert_eq!(
            board.piece_placement[81],
            Square::Occupied(Piece::new(Black, Pawn))
        );

        assert_eq!(board.piece_placement[55], Square::Empty);
        assert_eq!(board.piece_placement[65], Square::Empty);
    }

    #[test]
    fn start_board_count_checks() {
        let board = Board::start();

        let off_board_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| matches!(sq, Square::OffBoard))
            .count();

        assert_eq!(off_board_cnt, 56);

        let empty_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| matches!(sq, Square::Empty))
            .count();

        assert_eq!(empty_cnt, 32);

        let occupied_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| matches!(sq, Square::Occupied(_)))
            .count();

        assert_eq!(occupied_cnt, 32);

        let white_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| matches!(sq, Square::Occupied(Piece { color: White, .. })))
            .count();

        assert_eq!(white_cnt, 16);

        let black_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| matches!(sq, Square::Occupied(Piece { color: Black, .. })))
            .count();

        assert_eq!(black_cnt, 16);

        let white_pawn_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: Pawn
                    })
                )
            })
            .count();

        assert_eq!(white_pawn_cnt, 8);

        let black_pawn_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: Pawn
                    })
                )
            })
            .count();

        assert_eq!(black_pawn_cnt, 8);

        let white_rook_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: Rook
                    })
                )
            })
            .count();

        assert_eq!(white_rook_cnt, 2);

        let black_rook_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: Rook
                    })
                )
            })
            .count();

        assert_eq!(black_rook_cnt, 2);

        let white_knight_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: Knight
                    })
                )
            })
            .count();

        assert_eq!(white_knight_cnt, 2);

        let black_knight_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: Knight
                    })
                )
            })
            .count();

        assert_eq!(black_knight_cnt, 2);

        let white_bishop_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: Bishop
                    })
                )
            })
            .count();

        assert_eq!(white_bishop_cnt, 2);

        let black_bishop_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: Bishop
                    })
                )
            })
            .count();

        assert_eq!(black_bishop_cnt, 2);

        let white_queen_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: Queen
                    })
                )
            })
            .count();

        assert_eq!(white_queen_cnt, 1);

        let black_queen_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: Queen
                    })
                )
            })
            .count();

        assert_eq!(black_queen_cnt, 1);

        let white_king_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: White,
                        kind: King
                    })
                )
            })
            .count();

        assert_eq!(white_king_cnt, 1);

        let black_king_cnt = board
            .piece_placement
            .iter()
            .filter(|sq| {
                matches!(
                    sq,
                    Square::Occupied(Piece {
                        color: Black,
                        kind: King
                    })
                )
            })
            .count();

        assert_eq!(black_king_cnt, 1);
    }

    #[test]
    fn start_board_displays() {
        let board = Board::start();
        let output = board.to_string();

        assert_eq!(
            output,
            "rnbqkbnr\npppppppp\n........\n........\n........\n........\nPPPPPPPP\nRNBQKBNR\n"
        )
    }
}
