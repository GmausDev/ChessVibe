use crate::engine::{
    Bitboard, CastlingRights, Color, Move, MoveType, Piece, PieceType, Square,
};

/// Represents the complete state of a chess game.
#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
    /// Bitboards for white pieces
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    /// Bitboards for black pieces
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

    /// Whose turn it is
    pub side_to_move: Color,

    /// Castling rights
    pub castling_rights: CastlingRights,

    /// En passant target square (if any)
    pub en_passant_square: Option<Square>,

    /// Half-move clock for 50-move rule
    pub halfmove_clock: u32,

    /// Full move number
    pub fullmove_number: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self::initial()
    }
}

impl GameState {
    /// Creates the starting position.
    pub fn initial() -> Self {
        Self {
            // White pieces
            white_pawns: Bitboard::RANK_2,
            white_knights: Bitboard::from_square(Square::B1) | Bitboard::from_square(Square::G1),
            white_bishops: Bitboard::from_square(Square::C1) | Bitboard::from_square(Square::F1),
            white_rooks: Bitboard::from_square(Square::A1) | Bitboard::from_square(Square::H1),
            white_queens: Bitboard::from_square(Square::D1),
            white_king: Bitboard::from_square(Square::E1),

            // Black pieces
            black_pawns: Bitboard::RANK_7,
            black_knights: Bitboard::from_square(Square::B8) | Bitboard::from_square(Square::G8),
            black_bishops: Bitboard::from_square(Square::C8) | Bitboard::from_square(Square::F8),
            black_rooks: Bitboard::from_square(Square::A8) | Bitboard::from_square(Square::H8),
            black_queens: Bitboard::from_square(Square::D8),
            black_king: Bitboard::from_square(Square::E8),

            side_to_move: Color::White,
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Creates an empty board.
    pub fn empty() -> Self {
        Self {
            white_pawns: Bitboard::EMPTY,
            white_knights: Bitboard::EMPTY,
            white_bishops: Bitboard::EMPTY,
            white_rooks: Bitboard::EMPTY,
            white_queens: Bitboard::EMPTY,
            white_king: Bitboard::EMPTY,
            black_pawns: Bitboard::EMPTY,
            black_knights: Bitboard::EMPTY,
            black_bishops: Bitboard::EMPTY,
            black_rooks: Bitboard::EMPTY,
            black_queens: Bitboard::EMPTY,
            black_king: Bitboard::EMPTY,
            side_to_move: Color::White,
            castling_rights: CastlingRights::none(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Returns a bitboard of all white pieces.
    pub fn white_pieces(&self) -> Bitboard {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    /// Returns a bitboard of all black pieces.
    pub fn black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    /// Returns a bitboard of all pieces.
    pub fn all_pieces(&self) -> Bitboard {
        self.white_pieces() | self.black_pieces()
    }

    /// Returns a bitboard of pieces for the given color.
    pub fn pieces_of_color(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        }
    }

    /// Returns a bitboard of pieces of the given type and color.
    pub fn pieces(&self, color: Color, piece_type: PieceType) -> Bitboard {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => self.white_pawns,
            (Color::White, PieceType::Knight) => self.white_knights,
            (Color::White, PieceType::Bishop) => self.white_bishops,
            (Color::White, PieceType::Rook) => self.white_rooks,
            (Color::White, PieceType::Queen) => self.white_queens,
            (Color::White, PieceType::King) => self.white_king,
            (Color::Black, PieceType::Pawn) => self.black_pawns,
            (Color::Black, PieceType::Knight) => self.black_knights,
            (Color::Black, PieceType::Bishop) => self.black_bishops,
            (Color::Black, PieceType::Rook) => self.black_rooks,
            (Color::Black, PieceType::Queen) => self.black_queens,
            (Color::Black, PieceType::King) => self.black_king,
        }
    }

    /// Returns a mutable reference to the bitboard for pieces of the given type and color.
    fn pieces_mut(&mut self, color: Color, piece_type: PieceType) -> &mut Bitboard {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => &mut self.white_pawns,
            (Color::White, PieceType::Knight) => &mut self.white_knights,
            (Color::White, PieceType::Bishop) => &mut self.white_bishops,
            (Color::White, PieceType::Rook) => &mut self.white_rooks,
            (Color::White, PieceType::Queen) => &mut self.white_queens,
            (Color::White, PieceType::King) => &mut self.white_king,
            (Color::Black, PieceType::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceType::Knight) => &mut self.black_knights,
            (Color::Black, PieceType::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceType::Rook) => &mut self.black_rooks,
            (Color::Black, PieceType::Queen) => &mut self.black_queens,
            (Color::Black, PieceType::King) => &mut self.black_king,
        }
    }

    /// Returns the piece at the given square, if any.
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let bb = Bitboard::from_square(square);

        // Check white pieces
        if (self.white_pawns & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::Pawn));
        }
        if (self.white_knights & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::Knight));
        }
        if (self.white_bishops & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::Bishop));
        }
        if (self.white_rooks & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::Rook));
        }
        if (self.white_queens & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::Queen));
        }
        if (self.white_king & bb).is_not_empty() {
            return Some(Piece::new(Color::White, PieceType::King));
        }

        // Check black pieces
        if (self.black_pawns & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::Pawn));
        }
        if (self.black_knights & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::Knight));
        }
        if (self.black_bishops & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::Bishop));
        }
        if (self.black_rooks & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::Rook));
        }
        if (self.black_queens & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::Queen));
        }
        if (self.black_king & bb).is_not_empty() {
            return Some(Piece::new(Color::Black, PieceType::King));
        }

        None
    }

    /// Returns the square of the king for the given color.
    pub fn king_square(&self, color: Color) -> Square {
        let king_bb = match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        };
        king_bb.lsb().expect("King must exist")
    }

    /// Places a piece on the board.
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        // First clear any existing piece at this square
        self.clear_square(square);

        // Set the piece
        self.pieces_mut(piece.color, piece.piece_type).set(square);
    }

    /// Removes any piece from the given square.
    pub fn clear_square(&mut self, square: Square) {
        let bb = Bitboard::from_square(square);
        let not_bb = !bb;

        self.white_pawns &= not_bb;
        self.white_knights &= not_bb;
        self.white_bishops &= not_bb;
        self.white_rooks &= not_bb;
        self.white_queens &= not_bb;
        self.white_king &= not_bb;
        self.black_pawns &= not_bb;
        self.black_knights &= not_bb;
        self.black_bishops &= not_bb;
        self.black_rooks &= not_bb;
        self.black_queens &= not_bb;
        self.black_king &= not_bb;
    }

    /// Creates a GameState from a FEN string.
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("FEN must have at least 4 parts".to_string());
        }

        let mut state = Self::empty();

        // Part 1: Piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("FEN piece placement must have 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_idx as u8;
            let mut file: u8 = 0;

            for c in rank_str.chars() {
                if file > 7 {
                    return Err(format!("Too many squares in rank {}", rank + 1));
                }

                if c.is_ascii_digit() {
                    let skip = c.to_digit(10).unwrap() as u8;
                    file += skip;
                } else if let Some(piece) = Piece::from_char(c) {
                    let square = Square::from_coords(file, rank);
                    state.set_piece(square, piece);
                    file += 1;
                } else {
                    return Err(format!("Invalid FEN character: {}", c));
                }
            }
        }

        // Part 2: Side to move
        state.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };

        // Part 3: Castling rights
        state.castling_rights = CastlingRights::from_fen(parts[2]);

        // Part 4: En passant square
        state.en_passant_square = if parts[3] == "-" {
            None
        } else {
            Square::from_algebraic(parts[3])
        };

        // Part 5: Halfmove clock (optional)
        state.halfmove_clock = if parts.len() > 4 {
            parts[4].parse().unwrap_or(0)
        } else {
            0
        };

        // Part 6: Fullmove number (optional)
        state.fullmove_number = if parts.len() > 5 {
            parts[5].parse().unwrap_or(1)
        } else {
            1
        };

        Ok(state)
    }

    /// Exports the current position to a FEN string.
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Part 1: Piece placement
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square = Square::from_coords(file, rank);
                if let Some(piece) = self.piece_at(square) {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        // Part 2: Side to move
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Part 3: Castling rights
        fen.push(' ');
        fen.push_str(&self.castling_rights.to_fen());

        // Part 4: En passant square
        fen.push(' ');
        match self.en_passant_square {
            Some(sq) => fen.push_str(&sq.to_algebraic()),
            None => fen.push('-'),
        }

        // Part 5: Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Part 6: Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    /// Applies a move and returns the new game state.
    pub fn apply_move(&self, mv: Move) -> Self {
        let mut new_state = self.clone();
        let color = self.side_to_move;
        let opponent = color.opposite();

        // Clear the source square
        new_state.pieces_mut(color, mv.piece).clear(mv.from);

        // Handle captures
        if let Some(captured) = mv.captured {
            if mv.move_type == MoveType::EnPassant {
                // En passant captures the pawn on a different square
                let captured_square = Square::from_coords(mv.to.file(), mv.from.rank());
                new_state.pieces_mut(opponent, captured).clear(captured_square);
            } else {
                new_state.pieces_mut(opponent, captured).clear(mv.to);
            }
        }

        // Place the piece (or promoted piece) on the target square
        let placed_piece = mv.promotion.unwrap_or(mv.piece);
        new_state.pieces_mut(color, placed_piece).set(mv.to);

        // Handle castling - move the rook
        match mv.move_type {
            MoveType::CastleKingside => {
                let (rook_from, rook_to) = match color {
                    Color::White => (Square::H1, Square::F1),
                    Color::Black => (Square::H8, Square::F8),
                };
                new_state.pieces_mut(color, PieceType::Rook).clear(rook_from);
                new_state.pieces_mut(color, PieceType::Rook).set(rook_to);
            }
            MoveType::CastleQueenside => {
                let (rook_from, rook_to) = match color {
                    Color::White => (Square::A1, Square::D1),
                    Color::Black => (Square::A8, Square::D8),
                };
                new_state.pieces_mut(color, PieceType::Rook).clear(rook_from);
                new_state.pieces_mut(color, PieceType::Rook).set(rook_to);
            }
            _ => {}
        }

        // Update en passant square
        new_state.en_passant_square = if mv.move_type == MoveType::DoublePawnPush {
            let ep_rank = (mv.from.rank() as i8 + mv.to.rank() as i8) / 2;
            Some(Square::from_coords(mv.from.file(), ep_rank as u8))
        } else {
            None
        };

        // Update castling rights
        if mv.piece == PieceType::King {
            match color {
                Color::White => {
                    new_state.castling_rights.white_kingside = false;
                    new_state.castling_rights.white_queenside = false;
                }
                Color::Black => {
                    new_state.castling_rights.black_kingside = false;
                    new_state.castling_rights.black_queenside = false;
                }
            }
        }

        // Update castling rights if rook moves or is captured
        if mv.from == Square::A1 || mv.to == Square::A1 {
            new_state.castling_rights.white_queenside = false;
        }
        if mv.from == Square::H1 || mv.to == Square::H1 {
            new_state.castling_rights.white_kingside = false;
        }
        if mv.from == Square::A8 || mv.to == Square::A8 {
            new_state.castling_rights.black_queenside = false;
        }
        if mv.from == Square::H8 || mv.to == Square::H8 {
            new_state.castling_rights.black_kingside = false;
        }

        // Update halfmove clock
        if mv.piece == PieceType::Pawn || mv.captured.is_some() {
            new_state.halfmove_clock = 0;
        } else {
            new_state.halfmove_clock = self.halfmove_clock + 1;
        }

        // Update fullmove number
        if color == Color::Black {
            new_state.fullmove_number = self.fullmove_number + 1;
        }

        // Switch side to move
        new_state.side_to_move = opponent;

        new_state
    }
}

impl std::fmt::Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            write!(f, "{}  ", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_coords(file, rank);
                if let Some(piece) = self.piece_at(square) {
                    write!(f, "{} ", piece.to_char())?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   a b c d e f g h")?;
        writeln!(f)?;
        writeln!(f, "Side to move: {:?}", self.side_to_move)?;
        writeln!(f, "Castling: {}", self.castling_rights.to_fen())?;
        writeln!(
            f,
            "En passant: {}",
            self.en_passant_square
                .map_or("-".to_string(), |s| s.to_algebraic())
        )?;
        writeln!(
            f,
            "Halfmove clock: {}, Fullmove: {}",
            self.halfmove_clock, self.fullmove_number
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_position() {
        let state = GameState::initial();

        assert_eq!(state.side_to_move, Color::White);
        assert!(state.castling_rights.white_kingside);
        assert!(state.castling_rights.white_queenside);
        assert!(state.castling_rights.black_kingside);
        assert!(state.castling_rights.black_queenside);
        assert_eq!(state.en_passant_square, None);
        assert_eq!(state.halfmove_clock, 0);
        assert_eq!(state.fullmove_number, 1);
    }

    #[test]
    fn test_piece_at_initial() {
        let state = GameState::initial();

        // White pieces
        assert_eq!(
            state.piece_at(Square::E1),
            Some(Piece::new(Color::White, PieceType::King))
        );
        assert_eq!(
            state.piece_at(Square::D1),
            Some(Piece::new(Color::White, PieceType::Queen))
        );
        assert_eq!(
            state.piece_at(Square::from_algebraic("e2").unwrap()),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );

        // Black pieces
        assert_eq!(
            state.piece_at(Square::E8),
            Some(Piece::new(Color::Black, PieceType::King))
        );
        assert_eq!(
            state.piece_at(Square::D8),
            Some(Piece::new(Color::Black, PieceType::Queen))
        );

        // Empty squares
        assert_eq!(state.piece_at(Square::from_algebraic("e4").unwrap()), None);
    }

    #[test]
    fn test_king_squares() {
        let state = GameState::initial();
        assert_eq!(state.king_square(Color::White), Square::E1);
        assert_eq!(state.king_square(Color::Black), Square::E8);
    }

    #[test]
    fn test_piece_counts() {
        let state = GameState::initial();

        // Each side starts with 16 pieces
        assert_eq!(state.white_pieces().count(), 16);
        assert_eq!(state.black_pieces().count(), 16);
        assert_eq!(state.all_pieces().count(), 32);

        // Specific piece counts
        assert_eq!(state.white_pawns.count(), 8);
        assert_eq!(state.white_knights.count(), 2);
        assert_eq!(state.white_bishops.count(), 2);
        assert_eq!(state.white_rooks.count(), 2);
        assert_eq!(state.white_queens.count(), 1);
        assert_eq!(state.white_king.count(), 1);
    }

    #[test]
    fn test_apply_simple_move() {
        let state = GameState::initial();

        // 1. e4
        let mv = Move {
            from: Square::from_algebraic("e2").unwrap(),
            to: Square::from_algebraic("e4").unwrap(),
            piece: PieceType::Pawn,
            captured: None,
            promotion: None,
            move_type: MoveType::DoublePawnPush,
        };

        let new_state = state.apply_move(mv);

        // Check pawn moved
        assert_eq!(new_state.piece_at(Square::from_algebraic("e2").unwrap()), None);
        assert_eq!(
            new_state.piece_at(Square::from_algebraic("e4").unwrap()),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );

        // Check en passant square
        assert_eq!(
            new_state.en_passant_square,
            Some(Square::from_algebraic("e3").unwrap())
        );

        // Check side to move switched
        assert_eq!(new_state.side_to_move, Color::Black);

        // Fullmove number should still be 1 (increments after black's move)
        assert_eq!(new_state.fullmove_number, 1);
    }

    #[test]
    fn test_apply_capture() {
        let mut state = GameState::empty();
        state.set_piece(Square::from_algebraic("e4").unwrap(), Piece::new(Color::White, PieceType::Knight));
        state.set_piece(Square::from_algebraic("d6").unwrap(), Piece::new(Color::Black, PieceType::Pawn));
        state.side_to_move = Color::White;

        let mv = Move::capture(
            Square::from_algebraic("e4").unwrap(),
            Square::from_algebraic("d6").unwrap(),
            PieceType::Knight,
            PieceType::Pawn,
        );

        let new_state = state.apply_move(mv);

        assert_eq!(new_state.piece_at(Square::from_algebraic("e4").unwrap()), None);
        assert_eq!(
            new_state.piece_at(Square::from_algebraic("d6").unwrap()),
            Some(Piece::new(Color::White, PieceType::Knight))
        );
        assert_eq!(new_state.halfmove_clock, 0); // Reset on capture
    }

    #[test]
    fn test_kingside_castle() {
        let mut state = GameState::empty();
        state.set_piece(Square::E1, Piece::new(Color::White, PieceType::King));
        state.set_piece(Square::H1, Piece::new(Color::White, PieceType::Rook));
        state.castling_rights.white_kingside = true;
        state.side_to_move = Color::White;

        let mv = Move::castle(true, Color::White);
        let new_state = state.apply_move(mv);

        // King should be on g1
        assert_eq!(
            new_state.piece_at(Square::G1),
            Some(Piece::new(Color::White, PieceType::King))
        );
        // Rook should be on f1
        assert_eq!(
            new_state.piece_at(Square::F1),
            Some(Piece::new(Color::White, PieceType::Rook))
        );
        // Original squares should be empty
        assert_eq!(new_state.piece_at(Square::E1), None);
        assert_eq!(new_state.piece_at(Square::H1), None);
        // Castling rights should be removed
        assert!(!new_state.castling_rights.white_kingside);
        assert!(!new_state.castling_rights.white_queenside);
    }

    #[test]
    fn test_fen_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();

        // Verify it matches the initial position
        let initial = GameState::initial();
        assert_eq!(state.to_fen(), initial.to_fen());
    }

    #[test]
    fn test_fen_roundtrip() {
        // Test that we can export and re-import a position
        let initial = GameState::initial();
        let fen = initial.to_fen();
        let reimported = GameState::from_fen(&fen).unwrap();
        assert_eq!(reimported.to_fen(), fen);
    }

    #[test]
    fn test_fen_custom_position() {
        // Position after 1.e4 e5
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let state = GameState::from_fen(fen).unwrap();

        // White pawn should be on e4
        assert_eq!(
            state.piece_at(Square::from_algebraic("e4").unwrap()),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        // Black pawn should be on e5
        assert_eq!(
            state.piece_at(Square::from_algebraic("e5").unwrap()),
            Some(Piece::new(Color::Black, PieceType::Pawn))
        );
        // En passant square should be e6
        assert_eq!(
            state.en_passant_square,
            Square::from_algebraic("e6")
        );
        assert_eq!(state.fullmove_number, 2);
    }
}
