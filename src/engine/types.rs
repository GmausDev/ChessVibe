/// Represents a player color in chess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the opposite color.
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /// Returns the direction pawns move for this color (1 for white, -1 for black).
    pub fn pawn_direction(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    /// Returns the starting rank for pawns of this color.
    pub fn pawn_start_rank(self) -> u8 {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    /// Returns the promotion rank for pawns of this color.
    pub fn promotion_rank(self) -> u8 {
        match self {
            Color::White => 7,
            Color::Black => 0,
        }
    }
}

/// Represents a chess piece type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Returns all piece types.
    pub fn all() -> [PieceType; 6] {
        [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ]
    }

    /// Returns the FEN character for this piece type (uppercase).
    pub fn to_char(self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }

    /// Creates a piece type from a FEN character (case-insensitive).
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }

    /// Returns the material value of this piece type (in centipawns).
    pub fn value(self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }
}

/// Represents a chess piece with color and type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    /// Creates a new piece.
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }

    /// Returns the FEN character for this piece.
    pub fn to_char(self) -> char {
        let c = self.piece_type.to_char();
        match self.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase(),
        }
    }

    /// Creates a piece from a FEN character.
    pub fn from_char(c: char) -> Option<Self> {
        let piece_type = PieceType::from_char(c)?;
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(Self { color, piece_type })
    }
}

/// Represents a square on the chess board (0-63).
/// Squares are numbered a1=0, b1=1, ..., h1=7, a2=8, ..., h8=63.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub u8);

impl Square {
    /// Creates a square from file (0-7) and rank (0-7).
    pub fn from_coords(file: u8, rank: u8) -> Self {
        debug_assert!(file < 8 && rank < 8);
        Square(rank * 8 + file)
    }

    /// Creates a square from algebraic notation (e.g., "e4").
    pub fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }

        let file = bytes[0].wrapping_sub(b'a');
        let rank = bytes[1].wrapping_sub(b'1');

        if file < 8 && rank < 8 {
            Some(Square::from_coords(file, rank))
        } else {
            None
        }
    }

    /// Returns the file (0-7, where 0 is 'a').
    pub fn file(self) -> u8 {
        self.0 % 8
    }

    /// Returns the rank (0-7, where 0 is rank 1).
    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    /// Returns the algebraic notation for this square (e.g., "e4").
    pub fn to_algebraic(self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;
        format!("{}{}", file, rank)
    }

    /// Returns true if this square is on a light square.
    pub fn is_light(self) -> bool {
        (self.file() + self.rank()) % 2 == 1
    }

    /// Returns the index (0-63).
    pub fn index(self) -> usize {
        self.0 as usize
    }

    /// Named square constants - Rank 1.
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    /// Rank 2.
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    /// Rank 7.
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    /// Rank 8.
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}

/// Type of chess move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion,
}

/// Represents a chess move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: PieceType,
    pub captured: Option<PieceType>,
    pub promotion: Option<PieceType>,
    pub move_type: MoveType,
}

impl Move {
    /// Creates a simple move without captures or special types.
    pub fn new(from: Square, to: Square, piece: PieceType) -> Self {
        Self {
            from,
            to,
            piece,
            captured: None,
            promotion: None,
            move_type: MoveType::Normal,
        }
    }

    /// Creates a capture move.
    pub fn capture(from: Square, to: Square, piece: PieceType, captured: PieceType) -> Self {
        Self {
            from,
            to,
            piece,
            captured: Some(captured),
            promotion: None,
            move_type: MoveType::Normal,
        }
    }

    /// Creates a promotion move.
    pub fn promotion(
        from: Square,
        to: Square,
        captured: Option<PieceType>,
        promotion: PieceType,
    ) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            captured,
            promotion: Some(promotion),
            move_type: MoveType::Promotion,
        }
    }

    /// Creates a castling move.
    pub fn castle(kingside: bool, color: Color) -> Self {
        let (from, to) = if color == Color::White {
            if kingside {
                (Square::E1, Square::G1)
            } else {
                (Square::E1, Square::C1)
            }
        } else if kingside {
            (Square::E8, Square::G8)
        } else {
            (Square::E8, Square::C8)
        };

        Self {
            from,
            to,
            piece: PieceType::King,
            captured: None,
            promotion: None,
            move_type: if kingside {
                MoveType::CastleKingside
            } else {
                MoveType::CastleQueenside
            },
        }
    }

    /// Creates an en passant capture.
    pub fn en_passant(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            captured: Some(PieceType::Pawn),
            promotion: None,
            move_type: MoveType::EnPassant,
        }
    }

    /// Creates a double pawn push.
    pub fn double_pawn_push(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            captured: None,
            promotion: None,
            move_type: MoveType::DoublePawnPush,
        }
    }
}

/// Castling rights for both players.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    /// All castling rights available.
    pub fn all() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    /// No castling rights.
    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    /// Converts to FEN string.
    pub fn to_fen(&self) -> String {
        let mut s = String::new();
        if self.white_kingside {
            s.push('K');
        }
        if self.white_queenside {
            s.push('Q');
        }
        if self.black_kingside {
            s.push('k');
        }
        if self.black_queenside {
            s.push('q');
        }
        if s.is_empty() {
            s.push('-');
        }
        s
    }

    /// Parses from FEN string.
    pub fn from_fen(s: &str) -> Self {
        Self {
            white_kingside: s.contains('K'),
            white_queenside: s.contains('Q'),
            black_kingside: s.contains('k'),
            black_queenside: s.contains('q'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_algebraic() {
        assert_eq!(Square::from_algebraic("a1"), Some(Square(0)));
        assert_eq!(Square::from_algebraic("h8"), Some(Square(63)));
        assert_eq!(Square::from_algebraic("e4"), Some(Square(28)));
        assert_eq!(Square::from_algebraic("d5"), Some(Square(35)));
        assert_eq!(Square::from_algebraic("i9"), None);
    }

    #[test]
    fn test_square_to_algebraic() {
        assert_eq!(Square(0).to_algebraic(), "a1");
        assert_eq!(Square(63).to_algebraic(), "h8");
        assert_eq!(Square(28).to_algebraic(), "e4");
    }

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_piece_from_char() {
        assert_eq!(
            Piece::from_char('K'),
            Some(Piece::new(Color::White, PieceType::King))
        );
        assert_eq!(
            Piece::from_char('p'),
            Some(Piece::new(Color::Black, PieceType::Pawn))
        );
    }

    #[test]
    fn test_castling_rights_fen() {
        assert_eq!(CastlingRights::all().to_fen(), "KQkq");
        assert_eq!(CastlingRights::none().to_fen(), "-");
    }
}
