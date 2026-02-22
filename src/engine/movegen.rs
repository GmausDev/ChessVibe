use crate::engine::{Bitboard, Color, GameState, Move, PieceType, Square};

impl GameState {
    /// Generates all pseudo-legal moves (doesn't check if king is left in check).
    pub fn generate_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.generate_pawn_moves(&mut moves);
        self.generate_knight_moves(&mut moves);
        self.generate_bishop_moves(&mut moves);
        self.generate_rook_moves(&mut moves);
        self.generate_queen_moves(&mut moves);
        self.generate_king_moves(&mut moves);

        moves
    }

    /// Generates all legal moves.
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.generate_pseudo_legal_moves()
            .into_iter()
            .filter(|mv| {
                let new_state = self.apply_move(*mv);
                !new_state.is_in_check(self.side_to_move)
            })
            .collect()
    }

    /// Generates pawn moves.
    fn generate_pawn_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let pawns = self.pieces(color, PieceType::Pawn);
        let occupied = self.all_pieces();
        let enemies = self.pieces_of_color(color.opposite());

        let (push_dir, _start_rank, promo_rank, double_rank): (
            fn(Bitboard) -> Bitboard,
            Bitboard,
            Bitboard,
            Bitboard,
        ) = match color {
            Color::White => (
                Bitboard::north,
                Bitboard::RANK_2,
                Bitboard::RANK_8,
                Bitboard::RANK_4,
            ),
            Color::Black => (
                Bitboard::south,
                Bitboard::RANK_7,
                Bitboard::RANK_1,
                Bitboard::RANK_5,
            ),
        };

        // Single pawn pushes
        let single_pushes = push_dir(pawns) & !occupied;

        // Non-promotion pushes
        for to_sq in (single_pushes & !promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 8),
                Color::Black => Square(to_sq.0 + 8),
            };
            moves.push(Move::new(from_sq, to_sq, PieceType::Pawn));
        }

        // Promotion pushes
        for to_sq in (single_pushes & promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 8),
                Color::Black => Square(to_sq.0 + 8),
            };
            self.add_promotions(moves, from_sq, to_sq, None);
        }

        // Double pawn pushes
        let double_pushes = push_dir(single_pushes & match color {
            Color::White => Bitboard::RANK_3,
            Color::Black => Bitboard::RANK_6,
        }) & !occupied
            & double_rank;

        for to_sq in double_pushes.iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 16),
                Color::Black => Square(to_sq.0 + 16),
            };
            moves.push(Move::double_pawn_push(from_sq, to_sq));
        }

        // Pawn captures
        let (east_attack, west_attack): (fn(Bitboard) -> Bitboard, fn(Bitboard) -> Bitboard) =
            match color {
                Color::White => (Bitboard::north_east, Bitboard::north_west),
                Color::Black => (Bitboard::south_east, Bitboard::south_west),
            };

        // East captures
        let east_captures = east_attack(pawns) & enemies;
        for to_sq in (east_captures & !promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 9),
                Color::Black => Square(to_sq.0 + 7),
            };
            let captured = self.piece_at(to_sq).unwrap().piece_type;
            moves.push(Move::capture(from_sq, to_sq, PieceType::Pawn, captured));
        }
        for to_sq in (east_captures & promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 9),
                Color::Black => Square(to_sq.0 + 7),
            };
            let captured = self.piece_at(to_sq).unwrap().piece_type;
            self.add_promotions(moves, from_sq, to_sq, Some(captured));
        }

        // West captures
        let west_captures = west_attack(pawns) & enemies;
        for to_sq in (west_captures & !promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 7),
                Color::Black => Square(to_sq.0 + 9),
            };
            let captured = self.piece_at(to_sq).unwrap().piece_type;
            moves.push(Move::capture(from_sq, to_sq, PieceType::Pawn, captured));
        }
        for to_sq in (west_captures & promo_rank).iter() {
            let from_sq = match color {
                Color::White => Square(to_sq.0 - 7),
                Color::Black => Square(to_sq.0 + 9),
            };
            let captured = self.piece_at(to_sq).unwrap().piece_type;
            self.add_promotions(moves, from_sq, to_sq, Some(captured));
        }

        // En passant
        if let Some(ep_sq) = self.en_passant_square {
            let ep_bb = Bitboard::from_square(ep_sq);

            // East en passant
            if (east_attack(pawns) & ep_bb).is_not_empty() {
                let from_sq = match color {
                    Color::White => Square(ep_sq.0 - 9),
                    Color::Black => Square(ep_sq.0 + 7),
                };
                moves.push(Move::en_passant(from_sq, ep_sq));
            }

            // West en passant
            if (west_attack(pawns) & ep_bb).is_not_empty() {
                let from_sq = match color {
                    Color::White => Square(ep_sq.0 - 7),
                    Color::Black => Square(ep_sq.0 + 9),
                };
                moves.push(Move::en_passant(from_sq, ep_sq));
            }
        }
    }

    fn add_promotions(
        &self,
        moves: &mut Vec<Move>,
        from: Square,
        to: Square,
        captured: Option<PieceType>,
    ) {
        moves.push(Move::promotion(from, to, captured, PieceType::Queen));
        moves.push(Move::promotion(from, to, captured, PieceType::Rook));
        moves.push(Move::promotion(from, to, captured, PieceType::Bishop));
        moves.push(Move::promotion(from, to, captured, PieceType::Knight));
    }

    /// Generates knight moves.
    fn generate_knight_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let knights = self.pieces(color, PieceType::Knight);
        let own_pieces = self.pieces_of_color(color);

        for from_sq in knights.iter() {
            let attacks = knight_attacks(from_sq) & !own_pieces;

            for to_sq in attacks.iter() {
                if let Some(piece) = self.piece_at(to_sq) {
                    moves.push(Move::capture(from_sq, to_sq, PieceType::Knight, piece.piece_type));
                } else {
                    moves.push(Move::new(from_sq, to_sq, PieceType::Knight));
                }
            }
        }
    }

    /// Generates bishop moves.
    fn generate_bishop_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let bishops = self.pieces(color, PieceType::Bishop);
        let own_pieces = self.pieces_of_color(color);
        let occupied = self.all_pieces();

        for from_sq in bishops.iter() {
            let attacks = bishop_attacks(from_sq, occupied) & !own_pieces;

            for to_sq in attacks.iter() {
                if let Some(piece) = self.piece_at(to_sq) {
                    moves.push(Move::capture(from_sq, to_sq, PieceType::Bishop, piece.piece_type));
                } else {
                    moves.push(Move::new(from_sq, to_sq, PieceType::Bishop));
                }
            }
        }
    }

    /// Generates rook moves.
    fn generate_rook_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let rooks = self.pieces(color, PieceType::Rook);
        let own_pieces = self.pieces_of_color(color);
        let occupied = self.all_pieces();

        for from_sq in rooks.iter() {
            let attacks = rook_attacks(from_sq, occupied) & !own_pieces;

            for to_sq in attacks.iter() {
                if let Some(piece) = self.piece_at(to_sq) {
                    moves.push(Move::capture(from_sq, to_sq, PieceType::Rook, piece.piece_type));
                } else {
                    moves.push(Move::new(from_sq, to_sq, PieceType::Rook));
                }
            }
        }
    }

    /// Generates queen moves.
    fn generate_queen_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let queens = self.pieces(color, PieceType::Queen);
        let own_pieces = self.pieces_of_color(color);
        let occupied = self.all_pieces();

        for from_sq in queens.iter() {
            let attacks = queen_attacks(from_sq, occupied) & !own_pieces;

            for to_sq in attacks.iter() {
                if let Some(piece) = self.piece_at(to_sq) {
                    moves.push(Move::capture(from_sq, to_sq, PieceType::Queen, piece.piece_type));
                } else {
                    moves.push(Move::new(from_sq, to_sq, PieceType::Queen));
                }
            }
        }
    }

    /// Generates king moves including castling.
    fn generate_king_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let king = self.pieces(color, PieceType::King);
        let own_pieces = self.pieces_of_color(color);

        for from_sq in king.iter() {
            let attacks = king_attacks(from_sq) & !own_pieces;

            for to_sq in attacks.iter() {
                if let Some(piece) = self.piece_at(to_sq) {
                    moves.push(Move::capture(from_sq, to_sq, PieceType::King, piece.piece_type));
                } else {
                    moves.push(Move::new(from_sq, to_sq, PieceType::King));
                }
            }
        }

        // Castling
        self.generate_castling_moves(moves);
    }

    /// Generates castling moves.
    fn generate_castling_moves(&self, moves: &mut Vec<Move>) {
        let color = self.side_to_move;
        let occupied = self.all_pieces();

        let (kingside_right, queenside_right, king_sq, kingside_path, _queenside_path, queenside_rook_path) =
            match color {
                Color::White => (
                    self.castling_rights.white_kingside,
                    self.castling_rights.white_queenside,
                    Square::E1,
                    Bitboard::from_square(Square::F1) | Bitboard::from_square(Square::G1),
                    Bitboard::from_square(Square::D1) | Bitboard::from_square(Square::C1),
                    Bitboard::from_square(Square::D1)
                        | Bitboard::from_square(Square::C1)
                        | Bitboard::from_square(Square::B1),
                ),
                Color::Black => (
                    self.castling_rights.black_kingside,
                    self.castling_rights.black_queenside,
                    Square::E8,
                    Bitboard::from_square(Square::F8) | Bitboard::from_square(Square::G8),
                    Bitboard::from_square(Square::D8) | Bitboard::from_square(Square::C8),
                    Bitboard::from_square(Square::D8)
                        | Bitboard::from_square(Square::C8)
                        | Bitboard::from_square(Square::B8),
                ),
            };

        // Kingside castling
        if kingside_right && (occupied & kingside_path).is_empty() {
            // Check that king doesn't pass through or end on attacked square
            let f_sq = match color {
                Color::White => Square::F1,
                Color::Black => Square::F8,
            };
            let g_sq = match color {
                Color::White => Square::G1,
                Color::Black => Square::G8,
            };

            if !self.is_square_attacked(king_sq, color.opposite())
                && !self.is_square_attacked(f_sq, color.opposite())
                && !self.is_square_attacked(g_sq, color.opposite())
            {
                moves.push(Move::castle(true, color));
            }
        }

        // Queenside castling
        if queenside_right && (occupied & queenside_rook_path).is_empty() {
            let d_sq = match color {
                Color::White => Square::D1,
                Color::Black => Square::D8,
            };
            let c_sq = match color {
                Color::White => Square::C1,
                Color::Black => Square::C8,
            };

            if !self.is_square_attacked(king_sq, color.opposite())
                && !self.is_square_attacked(d_sq, color.opposite())
                && !self.is_square_attacked(c_sq, color.opposite())
            {
                moves.push(Move::castle(false, color));
            }
        }
    }

    /// Checks if a square is attacked by the given color.
    pub fn is_square_attacked(&self, square: Square, by_color: Color) -> bool {
        let occupied = self.all_pieces();

        // Knight attacks
        if (knight_attacks(square) & self.pieces(by_color, PieceType::Knight)).is_not_empty() {
            return true;
        }

        // King attacks
        if (king_attacks(square) & self.pieces(by_color, PieceType::King)).is_not_empty() {
            return true;
        }

        // Pawn attacks
        let pawn_attackers = match by_color {
            Color::White => {
                // White pawns attack from below
                let sw = Bitboard::from_square(square).south_west();
                let se = Bitboard::from_square(square).south_east();
                sw | se
            }
            Color::Black => {
                // Black pawns attack from above
                let nw = Bitboard::from_square(square).north_west();
                let ne = Bitboard::from_square(square).north_east();
                nw | ne
            }
        };
        if (pawn_attackers & self.pieces(by_color, PieceType::Pawn)).is_not_empty() {
            return true;
        }

        // Bishop/Queen attacks (diagonal)
        let bishop_attacks = bishop_attacks(square, occupied);
        if (bishop_attacks
            & (self.pieces(by_color, PieceType::Bishop) | self.pieces(by_color, PieceType::Queen)))
        .is_not_empty()
        {
            return true;
        }

        // Rook/Queen attacks (straight)
        let rook_attacks = rook_attacks(square, occupied);
        if (rook_attacks
            & (self.pieces(by_color, PieceType::Rook) | self.pieces(by_color, PieceType::Queen)))
        .is_not_empty()
        {
            return true;
        }

        false
    }

    /// Checks if the given color's king is in check.
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_sq = self.king_square(color);
        self.is_square_attacked(king_sq, color.opposite())
    }

    /// Checks if the current position is checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.is_in_check(self.side_to_move) && self.generate_legal_moves().is_empty()
    }

    /// Checks if the current position is stalemate.
    pub fn is_stalemate(&self) -> bool {
        !self.is_in_check(self.side_to_move) && self.generate_legal_moves().is_empty()
    }

    /// Checks for draw by insufficient material.
    pub fn is_insufficient_material(&self) -> bool {
        let total = self.all_pieces().count();

        // K vs K
        if total == 2 {
            return true;
        }

        // K+minor vs K
        if total == 3 {
            let minors = self.white_knights.count()
                + self.white_bishops.count()
                + self.black_knights.count()
                + self.black_bishops.count();
            if minors == 1 {
                return true;
            }
        }

        // K+B vs K+B (same colored bishops)
        if total == 4 {
            if self.white_bishops.count() == 1 && self.black_bishops.count() == 1 {
                let white_on_light =
                    (self.white_bishops & Bitboard::LIGHT_SQUARES).is_not_empty();
                let black_on_light =
                    (self.black_bishops & Bitboard::LIGHT_SQUARES).is_not_empty();
                return white_on_light == black_on_light;
            }
        }

        false
    }

    /// Checks if the 50-move rule applies.
    pub fn is_fifty_move_rule(&self) -> bool {
        self.halfmove_clock >= 100
    }
}

/// Pre-computed knight attack patterns.
fn knight_attacks(square: Square) -> Bitboard {
    let bb = Bitboard::from_square(square);
    let mut attacks = Bitboard::EMPTY;

    // All 8 knight moves
    attacks |= (bb << 17) & !Bitboard::FILE_A; // Up 2, right 1
    attacks |= (bb << 15) & !Bitboard::FILE_H; // Up 2, left 1
    attacks |= (bb << 10) & !(Bitboard::FILE_A | Bitboard::FILE_B); // Up 1, right 2
    attacks |= (bb << 6) & !(Bitboard::FILE_G | Bitboard::FILE_H); // Up 1, left 2
    attacks |= (bb >> 6) & !(Bitboard::FILE_A | Bitboard::FILE_B); // Down 1, right 2
    attacks |= (bb >> 10) & !(Bitboard::FILE_G | Bitboard::FILE_H); // Down 1, left 2
    attacks |= (bb >> 15) & !Bitboard::FILE_A; // Down 2, right 1
    attacks |= (bb >> 17) & !Bitboard::FILE_H; // Down 2, left 1

    attacks
}

/// Pre-computed king attack patterns.
fn king_attacks(square: Square) -> Bitboard {
    let bb = Bitboard::from_square(square);

    bb.north()
        | bb.south()
        | bb.east()
        | bb.west()
        | bb.north_east()
        | bb.north_west()
        | bb.south_east()
        | bb.south_west()
}

/// Bishop attacks using ray casting.
fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    // Northeast
    attacks |= ray_attacks(square, occupied, |bb| bb.north_east());
    // Northwest
    attacks |= ray_attacks(square, occupied, |bb| bb.north_west());
    // Southeast
    attacks |= ray_attacks(square, occupied, |bb| bb.south_east());
    // Southwest
    attacks |= ray_attacks(square, occupied, |bb| bb.south_west());

    attacks
}

/// Rook attacks using ray casting.
fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    // North
    attacks |= ray_attacks(square, occupied, |bb| bb.north());
    // South
    attacks |= ray_attacks(square, occupied, |bb| bb.south());
    // East
    attacks |= ray_attacks(square, occupied, |bb| bb.east());
    // West
    attacks |= ray_attacks(square, occupied, |bb| bb.west());

    attacks
}

/// Queen attacks (combination of rook and bishop).
fn queen_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    bishop_attacks(square, occupied) | rook_attacks(square, occupied)
}

/// Helper for generating ray attacks.
fn ray_attacks(square: Square, occupied: Bitboard, shift: fn(Bitboard) -> Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let mut bb = Bitboard::from_square(square);

    loop {
        bb = shift(bb);
        if bb.is_empty() {
            break;
        }
        attacks |= bb;
        if (bb & occupied).is_not_empty() {
            break;
        }
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::MoveType;

    #[test]
    fn test_initial_position_moves() {
        let state = GameState::initial();
        let moves = state.generate_legal_moves();

        // Initial position has 20 legal moves (16 pawn + 4 knight)
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_knight_attacks() {
        // Knight on e4 should attack 8 squares
        let attacks = knight_attacks(Square::from_algebraic("e4").unwrap());
        assert_eq!(attacks.count(), 8);

        // Knight on a1 should attack 2 squares
        let attacks = knight_attacks(Square::A1);
        assert_eq!(attacks.count(), 2);
    }

    #[test]
    fn test_king_attacks() {
        // King on e4 should attack 8 squares
        let attacks = king_attacks(Square::from_algebraic("e4").unwrap());
        assert_eq!(attacks.count(), 8);

        // King on a1 should attack 3 squares
        let attacks = king_attacks(Square::A1);
        assert_eq!(attacks.count(), 3);
    }

    #[test]
    fn test_pawn_double_push() {
        let state = GameState::initial();
        let moves = state.generate_legal_moves();

        // Count double pawn pushes
        let double_pushes: Vec<_> = moves
            .iter()
            .filter(|m| m.move_type == MoveType::DoublePawnPush)
            .collect();
        assert_eq!(double_pushes.len(), 8);
    }

    #[test]
    fn test_is_in_check() {
        let mut state = GameState::empty();
        state.set_piece(Square::E1, crate::engine::Piece::new(Color::White, PieceType::King));
        state.set_piece(Square::E8, crate::engine::Piece::new(Color::Black, PieceType::King));
        state.set_piece(Square::E7, crate::engine::Piece::new(Color::Black, PieceType::Rook));
        state.side_to_move = Color::White;

        assert!(state.is_in_check(Color::White));
        assert!(!state.is_in_check(Color::Black));
    }

    #[test]
    fn test_checkmate() {
        // Fool's mate position
        let mut state = GameState::initial();

        // 1. f3
        let mv = Move::new(
            Square::from_algebraic("f2").unwrap(),
            Square::from_algebraic("f3").unwrap(),
            PieceType::Pawn,
        );
        state = state.apply_move(mv);

        // 1... e5
        let mv = Move::double_pawn_push(
            Square::from_algebraic("e7").unwrap(),
            Square::from_algebraic("e5").unwrap(),
        );
        state = state.apply_move(mv);

        // 2. g4
        let mv = Move::double_pawn_push(
            Square::from_algebraic("g2").unwrap(),
            Square::from_algebraic("g4").unwrap(),
        );
        state = state.apply_move(mv);

        // 2... Qh4#
        let mv = Move::new(
            Square::from_algebraic("d8").unwrap(),
            Square::from_algebraic("h4").unwrap(),
            PieceType::Queen,
        );
        state = state.apply_move(mv);

        assert!(state.is_checkmate());
    }

    #[test]
    fn test_en_passant() {
        let mut state = GameState::empty();
        state.set_piece(
            Square::from_algebraic("e5").unwrap(),
            crate::engine::Piece::new(Color::White, PieceType::Pawn),
        );
        state.set_piece(
            Square::from_algebraic("d5").unwrap(),
            crate::engine::Piece::new(Color::Black, PieceType::Pawn),
        );
        state.set_piece(Square::E1, crate::engine::Piece::new(Color::White, PieceType::King));
        state.set_piece(Square::E8, crate::engine::Piece::new(Color::Black, PieceType::King));
        state.en_passant_square = Some(Square::from_algebraic("d6").unwrap());
        state.side_to_move = Color::White;

        let moves = state.generate_legal_moves();
        let ep_moves: Vec<_> = moves
            .iter()
            .filter(|m| m.move_type == MoveType::EnPassant)
            .collect();

        assert_eq!(ep_moves.len(), 1);
        assert_eq!(ep_moves[0].to, Square::from_algebraic("d6").unwrap());
    }

    #[test]
    fn test_castling() {
        let mut state = GameState::empty();
        state.set_piece(Square::E1, crate::engine::Piece::new(Color::White, PieceType::King));
        state.set_piece(Square::H1, crate::engine::Piece::new(Color::White, PieceType::Rook));
        state.set_piece(Square::A1, crate::engine::Piece::new(Color::White, PieceType::Rook));
        state.set_piece(Square::E8, crate::engine::Piece::new(Color::Black, PieceType::King));
        state.castling_rights.white_kingside = true;
        state.castling_rights.white_queenside = true;
        state.side_to_move = Color::White;

        let moves = state.generate_legal_moves();
        let castle_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                m.move_type == MoveType::CastleKingside
                    || m.move_type == MoveType::CastleQueenside
            })
            .collect();

        assert_eq!(castle_moves.len(), 2);
    }
}
