use crate::engine::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

/// A bitboard is a 64-bit integer where each bit represents a square on the chess board.
/// Bit 0 = a1, bit 1 = b1, ..., bit 7 = h1, bit 8 = a2, ..., bit 63 = h8.
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Empty bitboard (no squares set).
    pub const EMPTY: Bitboard = Bitboard(0);

    /// Full bitboard (all 64 squares set).
    pub const ALL: Bitboard = Bitboard(u64::MAX);

    /// Creates a bitboard with a single square set.
    pub fn from_square(square: Square) -> Self {
        Bitboard(1u64 << square.0)
    }

    /// Returns true if no bits are set.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns true if at least one bit is set.
    pub fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    /// Returns the number of bits set (population count).
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Sets the bit for the given square.
    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.0;
    }

    /// Clears the bit for the given square.
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.0);
    }

    /// Returns true if the bit for the given square is set.
    pub fn get(self, square: Square) -> bool {
        (self.0 >> square.0) & 1 == 1
    }

    /// Toggles the bit for the given square.
    pub fn toggle(&mut self, square: Square) {
        self.0 ^= 1u64 << square.0;
    }

    /// Returns the index of the least significant bit (0-63), or None if empty.
    pub fn lsb(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square(self.0.trailing_zeros() as u8))
        }
    }

    /// Returns the index of the most significant bit (0-63), or None if empty.
    pub fn msb(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square(63 - self.0.leading_zeros() as u8))
        }
    }

    /// Removes and returns the least significant bit.
    pub fn pop_lsb(&mut self) -> Option<Square> {
        let square = self.lsb()?;
        self.clear(square);
        Some(square)
    }

    /// Returns an iterator over all set squares.
    pub fn iter(self) -> BitboardIterator {
        BitboardIterator { bb: self }
    }

    /// Returns a bitboard representing a file (0-7).
    pub fn file(file: u8) -> Self {
        const FILE_A: u64 = 0x0101010101010101;
        Bitboard(FILE_A << file)
    }

    /// Returns a bitboard representing a rank (0-7).
    pub fn rank(rank: u8) -> Self {
        const RANK_1: u64 = 0xFF;
        Bitboard(RANK_1 << (rank * 8))
    }

    /// Returns a bitboard with files adjacent to the given file.
    pub fn adjacent_files(file: u8) -> Self {
        let mut result = Bitboard::EMPTY;
        if file > 0 {
            result |= Bitboard::file(file - 1);
        }
        if file < 7 {
            result |= Bitboard::file(file + 1);
        }
        result
    }

    // Pre-defined file and rank constants
    pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
    pub const FILE_C: Bitboard = Bitboard(0x0404040404040404);
    pub const FILE_D: Bitboard = Bitboard(0x0808080808080808);
    pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
    pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
    pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

    pub const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
    pub const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
    pub const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
    pub const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
    pub const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
    pub const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
    pub const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

    /// Light squares.
    pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);

    /// Dark squares.
    pub const DARK_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);

    /// Shifts the bitboard north (towards rank 8).
    pub fn north(self) -> Self {
        Bitboard(self.0 << 8)
    }

    /// Shifts the bitboard south (towards rank 1).
    pub fn south(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    /// Shifts the bitboard east (towards file H).
    pub fn east(self) -> Self {
        Bitboard((self.0 << 1) & !Self::FILE_A.0)
    }

    /// Shifts the bitboard west (towards file A).
    pub fn west(self) -> Self {
        Bitboard((self.0 >> 1) & !Self::FILE_H.0)
    }

    /// Shifts the bitboard northeast.
    pub fn north_east(self) -> Self {
        Bitboard((self.0 << 9) & !Self::FILE_A.0)
    }

    /// Shifts the bitboard northwest.
    pub fn north_west(self) -> Self {
        Bitboard((self.0 << 7) & !Self::FILE_H.0)
    }

    /// Shifts the bitboard southeast.
    pub fn south_east(self) -> Self {
        Bitboard((self.0 >> 7) & !Self::FILE_A.0)
    }

    /// Shifts the bitboard southwest.
    pub fn south_west(self) -> Self {
        Bitboard((self.0 >> 9) & !Self::FILE_H.0)
    }
}

/// Iterator over set squares in a bitboard.
pub struct BitboardIterator {
    bb: Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.bb.pop_lsb()
    }
}

// Implement standard bit operations
impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            write!(f, "{}  ", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_coords(file, rank);
                if self.get(square) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   a b c d e f g h")?;
        Ok(())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_and_all() {
        assert!(Bitboard::EMPTY.is_empty());
        assert!(!Bitboard::ALL.is_empty());
        assert_eq!(Bitboard::EMPTY.count(), 0);
        assert_eq!(Bitboard::ALL.count(), 64);
    }

    #[test]
    fn test_from_square() {
        let bb = Bitboard::from_square(Square(0)); // a1
        assert_eq!(bb.0, 1);
        assert_eq!(bb.count(), 1);
        assert!(bb.get(Square(0)));
        assert!(!bb.get(Square(1)));

        let bb = Bitboard::from_square(Square(63)); // h8
        assert_eq!(bb.0, 1u64 << 63);
    }

    #[test]
    fn test_set_clear_get() {
        let mut bb = Bitboard::EMPTY;
        bb.set(Square(0));
        bb.set(Square(7));
        bb.set(Square(63));

        assert!(bb.get(Square(0)));
        assert!(bb.get(Square(7)));
        assert!(bb.get(Square(63)));
        assert!(!bb.get(Square(1)));
        assert_eq!(bb.count(), 3);

        bb.clear(Square(7));
        assert!(!bb.get(Square(7)));
        assert_eq!(bb.count(), 2);
    }

    #[test]
    fn test_lsb_msb() {
        let mut bb = Bitboard::EMPTY;
        bb.set(Square(10));
        bb.set(Square(50));

        assert_eq!(bb.lsb(), Some(Square(10)));
        assert_eq!(bb.msb(), Some(Square(50)));

        assert_eq!(Bitboard::EMPTY.lsb(), None);
        assert_eq!(Bitboard::EMPTY.msb(), None);
    }

    #[test]
    fn test_iterator() {
        let mut bb = Bitboard::EMPTY;
        bb.set(Square(0));
        bb.set(Square(10));
        bb.set(Square(63));

        let squares: Vec<Square> = bb.iter().collect();
        assert_eq!(squares.len(), 3);
        assert!(squares.contains(&Square(0)));
        assert!(squares.contains(&Square(10)));
        assert!(squares.contains(&Square(63)));
    }

    #[test]
    fn test_file_and_rank() {
        assert_eq!(Bitboard::file(0), Bitboard::FILE_A);
        assert_eq!(Bitboard::file(7), Bitboard::FILE_H);
        assert_eq!(Bitboard::rank(0), Bitboard::RANK_1);
        assert_eq!(Bitboard::rank(7), Bitboard::RANK_8);

        assert_eq!(Bitboard::FILE_A.count(), 8);
        assert_eq!(Bitboard::RANK_1.count(), 8);
    }

    #[test]
    fn test_shifts() {
        let e4 = Bitboard::from_square(Square::from_algebraic("e4").unwrap());

        let e5 = e4.north();
        assert!(e5.get(Square::from_algebraic("e5").unwrap()));
        assert_eq!(e5.count(), 1);

        let e3 = e4.south();
        assert!(e3.get(Square::from_algebraic("e3").unwrap()));

        let f4 = e4.east();
        assert!(f4.get(Square::from_algebraic("f4").unwrap()));

        let d4 = e4.west();
        assert!(d4.get(Square::from_algebraic("d4").unwrap()));
    }

    #[test]
    fn test_edge_shifts() {
        // Test that shifts don't wrap around
        let a4 = Bitboard::from_square(Square::from_algebraic("a4").unwrap());
        let west_a4 = a4.west();
        assert!(west_a4.is_empty()); // Should be empty, not wrap to h4

        let h4 = Bitboard::from_square(Square::from_algebraic("h4").unwrap());
        let east_h4 = h4.east();
        assert!(east_h4.is_empty()); // Should be empty, not wrap to a4
    }

    #[test]
    fn test_bit_operations() {
        let bb1 = Bitboard::from_square(Square(0)) | Bitboard::from_square(Square(1));
        let bb2 = Bitboard::from_square(Square(1)) | Bitboard::from_square(Square(2));

        let intersection = bb1 & bb2;
        assert_eq!(intersection.count(), 1);
        assert!(intersection.get(Square(1)));

        let union = bb1 | bb2;
        assert_eq!(union.count(), 3);

        let xor = bb1 ^ bb2;
        assert_eq!(xor.count(), 2);
        assert!(xor.get(Square(0)));
        assert!(xor.get(Square(2)));
    }
}
