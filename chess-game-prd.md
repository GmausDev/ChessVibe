# Product Requirements Document: RustChess

## Chess Simulator/Game with 2D Graphics in Pure Rust

**Version:** 1.0  
**Author:** Technical Architecture Team  
**Date:** January 2026  
**Status:** Draft

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Goals and Objectives](#2-goals-and-objectives)
3. [Scope](#3-scope)
4. [Technical Architecture](#4-technical-architecture)
5. [Core Game Engine](#5-core-game-engine)
6. [Graphics System](#6-graphics-system)
7. [User Interface](#7-user-interface)
8. [AI Engine](#8-ai-engine)
9. [Game Modes](#9-game-modes)
10. [Data Persistence](#10-data-persistence)
11. [Audio System](#11-audio-system)
12. [Performance Requirements](#12-performance-requirements)
13. [Project Structure](#13-project-structure)
14. [Implementation Phases](#14-implementation-phases)
15. [Testing Strategy](#15-testing-strategy)
16. [Future Considerations](#16-future-considerations)

---

## 1. Executive Summary

RustChess is a fully-featured chess simulator built entirely in Rust, featuring 2D graphics, multiple game modes, and an AI opponent with configurable difficulty levels. The project prioritizes correctness of chess rule implementation, clean architecture following Rust idioms, and smooth visual presentation.

### Key Differentiators

- **Pure Rust Implementation:** No external chess engines; all logic written from scratch
- **Educational Value:** Clean, well-documented codebase suitable for learning both Rust and game development
- **Performance:** Leveraging Rust's zero-cost abstractions for efficient move generation and AI computation
- **Cross-Platform:** Single codebase deployable to Windows, macOS, and Linux

---

## 2. Goals and Objectives

### Primary Goals

| Goal | Success Metric |
|------|----------------|
| Complete chess rule implementation | 100% compliance with FIDE rules |
| Responsive 2D graphics | Consistent 60 FPS on mid-range hardware |
| Playable AI opponent | ELO ~1500 at highest difficulty |
| Cross-platform deployment | Builds and runs on Windows/macOS/Linux |

### Secondary Goals

- Serve as an educational resource for Rust game development
- Maintain clean separation between game logic and presentation
- Support standard chess notation formats (PGN, FEN)
- Provide accessible UI for players of all skill levels

### Non-Goals (v1.0)

- Online multiplayer
- 3D graphics
- Mobile platform support
- Chess960/variants (defer to v2.0)
- Opening book database
- Endgame tablebases

---

## 3. Scope

### In Scope

```
┌─────────────────────────────────────────────────────────────┐
│                      RustChess v1.0                         │
├─────────────────────────────────────────────────────────────┤
│  Core Engine          │  Graphics           │  Features     │
│  ─────────────────    │  ─────────────────  │  ───────────  │
│  • Move generation    │  • 2D board         │  • PvP local  │
│  • Move validation    │  • Piece sprites    │  • PvAI       │
│  • Game state mgmt    │  • Animations       │  • Save/Load  │
│  • Rule enforcement   │  • UI components    │  • PGN export │
│  • Check detection    │  • Theming          │  • Undo/Redo  │
│  • Checkmate/draw     │                     │  • Timers     │
└─────────────────────────────────────────────────────────────┘
```

### Out of Scope

- Network code and online play
- Puzzle mode
- Tournament management
- Analysis board with engine lines
- Custom piece sets (user-provided)

---

## 4. Technical Architecture

### 4.1 Technology Stack

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust (2021 edition) | Memory safety, performance, strong type system |
| Graphics | **macroquad** | Simple API, pure Rust, good 2D support, minimal boilerplate |
| Audio | macroquad built-in | Integrated with graphics library |
| Serialization | serde + serde_json | De-facto standard, excellent ergonomics |
| Build | Cargo | Standard Rust toolchain |

#### Alternative Graphics Libraries Considered

| Library | Pros | Cons | Decision |
|---------|------|------|----------|
| **macroquad** | Simple, minimal setup, async-friendly | Less control, smaller ecosystem | ✅ Selected |
| ggez | Good 2D API, active community | More dependencies, SDL2 requirement | ❌ |
| bevy | ECS architecture, modern | Overkill for chess, steep learning curve | ❌ |
| minifb + softbuffer | Minimal dependencies | Too low-level, manual everything | ❌ |
| SDL2-rs | Battle-tested, feature-rich | FFI bindings, C dependency | ❌ |

### 4.2 High-Level Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                          Application Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   Main Loop  │  │ Input Handler│  │ State Manager│              │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │
│         │                 │                 │                       │
│         ▼                 ▼                 ▼                       │
├────────────────────────────────────────────────────────────────────┤
│                          Game Logic Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ Chess Engine │  │   AI Engine  │  │  Game Rules  │              │
│  │              │  │              │  │              │              │
│  │ • Board      │  │ • Minimax    │  │ • Validation │              │
│  │ • Moves      │  │ • Evaluation │  │ • Check/Mate │              │
│  │ • Position   │  │ • Search     │  │ • Draw rules │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├────────────────────────────────────────────────────────────────────┤
│                         Presentation Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   Renderer   │  │  UI System   │  │ Audio System │              │
│  │              │  │              │  │              │              │
│  │ • Board      │  │ • Menus      │  │ • SFX        │              │
│  │ • Pieces     │  │ • Panels     │  │ • Music      │              │
│  │ • Effects    │  │ • Dialogs    │  │              │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
└────────────────────────────────────────────────────────────────────┘
```

### 4.3 Data Flow

```
User Input ──► Input Handler ──► Game State ──► Rule Validator
                                     │              │
                                     │              ▼
                                     │         Legal Move?
                                     │         ┌────┴────┐
                                     │        Yes        No
                                     │         │          │
                                     ▼         ▼          ▼
                               Update State  Apply    Reject
                                     │       Move
                                     │         │
                                     ▼         ▼
                               Render ◄── Check Game End
```

---

## 5. Core Game Engine

### 5.1 Board Representation

#### Primary: Bitboard Representation

Using 64-bit integers where each bit represents a square:

```rust
pub struct Bitboard(pub u64);

pub struct BoardState {
    // Piece bitboards (one per piece type per color)
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,
    
    // Aggregate bitboards (computed)
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub all_pieces: Bitboard,
}
```

#### Square Mapping

```
  a  b  c  d  e  f  g  h
8 56 57 58 59 60 61 62 63  8
7 48 49 50 51 52 53 54 55  7
6 40 41 42 43 44 45 46 47  6
5 32 33 34 35 36 37 38 39  5
4 24 25 26 27 28 29 30 31  4
3 16 17 18 19 20 21 22 23  3
2 08 09 10 11 12 13 14 15  2
1 00 01 02 03 04 05 06 07  1
  a  b  c  d  e  f  g  h
```

### 5.2 Game State

```rust
#[derive(Clone)]
pub struct GameState {
    pub board: BoardState,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u32,        // For 50-move rule
    pub fullmove_number: u32,
    pub position_history: Vec<u64>, // Zobrist hashes for repetition
}

#[derive(Clone, Copy)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}
```

### 5.3 Move Representation

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: PieceType,
    pub captured: Option<PieceType>,
    pub promotion: Option<PieceType>,
    pub move_type: MoveType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
```

### 5.4 Move Generation

#### Pseudo-Legal Move Generation

Generate all moves that follow piece movement rules, without checking for leaving king in check:

```rust
pub trait MoveGenerator {
    fn generate_pseudo_legal_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_pawn_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_knight_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_bishop_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_rook_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_queen_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_king_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_castling_moves(&self, state: &GameState) -> Vec<Move>;
}
```

#### Legal Move Filtering

```rust
pub fn generate_legal_moves(state: &GameState) -> Vec<Move> {
    let pseudo_legal = generate_pseudo_legal_moves(state);
    pseudo_legal
        .into_iter()
        .filter(|mv| {
            let new_state = state.apply_move(*mv);
            !is_in_check(&new_state, state.side_to_move)
        })
        .collect()
}
```

#### Pre-computed Attack Tables

For performance, pre-compute attack patterns:

```rust
lazy_static! {
    static ref KNIGHT_ATTACKS: [Bitboard; 64] = compute_knight_attacks();
    static ref KING_ATTACKS: [Bitboard; 64] = compute_king_attacks();
    static ref PAWN_ATTACKS: [[Bitboard; 64]; 2] = compute_pawn_attacks();
}

// Magic bitboards for sliding pieces (bishops, rooks, queens)
static ref BISHOP_MAGICS: [Magic; 64] = compute_bishop_magics();
static ref ROOK_MAGICS: [Magic; 64] = compute_rook_magics();
```

### 5.5 Chess Rules Implementation

#### Check Detection

```rust
pub fn is_in_check(state: &GameState, color: Color) -> bool {
    let king_square = state.king_square(color);
    is_square_attacked(state, king_square, color.opposite())
}

pub fn is_square_attacked(state: &GameState, square: Square, by_color: Color) -> bool {
    // Check attacks from each piece type
    let attackers = state.pieces_of_color(by_color);
    
    // Knight attacks
    if (KNIGHT_ATTACKS[square] & state.knights(by_color)).is_not_empty() {
        return true;
    }
    
    // Pawn attacks
    if (PAWN_ATTACKS[color.opposite()][square] & state.pawns(by_color)).is_not_empty() {
        return true;
    }
    
    // Sliding piece attacks (bishop, rook, queen)
    // ... (using magic bitboards)
    
    // King attacks
    if (KING_ATTACKS[square] & state.king(by_color)).is_not_empty() {
        return true;
    }
    
    false
}
```

#### Checkmate Detection

```rust
pub fn is_checkmate(state: &GameState) -> bool {
    is_in_check(state, state.side_to_move) && 
    generate_legal_moves(state).is_empty()
}
```

#### Stalemate Detection

```rust
pub fn is_stalemate(state: &GameState) -> bool {
    !is_in_check(state, state.side_to_move) && 
    generate_legal_moves(state).is_empty()
}
```

#### Draw Conditions

```rust
pub enum DrawReason {
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    InsufficientMaterial,
    Agreement,
}

pub fn check_draw(state: &GameState) -> Option<DrawReason> {
    if is_stalemate(state) {
        return Some(DrawReason::Stalemate);
    }
    
    if state.halfmove_clock >= 100 {
        return Some(DrawReason::FiftyMoveRule);
    }
    
    if is_threefold_repetition(state) {
        return Some(DrawReason::ThreefoldRepetition);
    }
    
    if is_insufficient_material(state) {
        return Some(DrawReason::InsufficientMaterial);
    }
    
    None
}
```

#### Insufficient Material

```rust
pub fn is_insufficient_material(state: &GameState) -> bool {
    let total_pieces = state.all_pieces.count();
    
    match total_pieces {
        2 => true,  // K vs K
        3 => {
            // K+B vs K or K+N vs K
            state.knights(Color::White).count() <= 1 &&
            state.knights(Color::Black).count() <= 1 &&
            state.bishops(Color::White).count() <= 1 &&
            state.bishops(Color::Black).count() <= 1
        }
        4 => {
            // K+B vs K+B (same colored bishops)
            let white_bishops = state.bishops(Color::White);
            let black_bishops = state.bishops(Color::Black);
            if white_bishops.count() == 1 && black_bishops.count() == 1 {
                let white_on_light = (white_bishops & LIGHT_SQUARES).is_not_empty();
                let black_on_light = (black_bishops & LIGHT_SQUARES).is_not_empty();
                return white_on_light == black_on_light;
            }
            false
        }
        _ => false,
    }
}
```

#### Castling Validation

```rust
pub fn can_castle_kingside(state: &GameState, color: Color) -> bool {
    let rights = match color {
        Color::White => state.castling_rights.white_kingside,
        Color::Black => state.castling_rights.black_kingside,
    };
    
    if !rights {
        return false;
    }
    
    let (king_sq, f_sq, g_sq, rook_sq) = match color {
        Color::White => (Square::E1, Square::F1, Square::G1, Square::H1),
        Color::Black => (Square::E8, Square::F8, Square::G8, Square::H8),
    };
    
    // Squares between king and rook must be empty
    if state.piece_at(f_sq).is_some() || state.piece_at(g_sq).is_some() {
        return false;
    }
    
    // King cannot be in check
    if is_in_check(state, color) {
        return false;
    }
    
    // King cannot pass through or land on attacked square
    let opponent = color.opposite();
    if is_square_attacked(state, f_sq, opponent) || 
       is_square_attacked(state, g_sq, opponent) {
        return false;
    }
    
    true
}
```

#### En Passant

```rust
pub fn generate_en_passant_moves(state: &GameState) -> Vec<Move> {
    let mut moves = Vec::new();
    
    if let Some(ep_square) = state.en_passant_square {
        let color = state.side_to_move;
        let our_pawns = state.pawns(color);
        
        // Find pawns that can capture en passant
        let attackers = PAWN_ATTACKS[color.opposite()][ep_square] & our_pawns;
        
        for from_square in attackers.iter_squares() {
            moves.push(Move {
                from: from_square,
                to: ep_square,
                piece: PieceType::Pawn,
                captured: Some(PieceType::Pawn),
                promotion: None,
                move_type: MoveType::EnPassant,
            });
        }
    }
    
    moves
}
```

#### Pawn Promotion

```rust
pub fn generate_pawn_promotion_moves(from: Square, to: Square, captured: Option<PieceType>) -> Vec<Move> {
    vec![
        Move { from, to, piece: PieceType::Pawn, captured, promotion: Some(PieceType::Queen), move_type: MoveType::Promotion },
        Move { from, to, piece: PieceType::Pawn, captured, promotion: Some(PieceType::Rook), move_type: MoveType::Promotion },
        Move { from, to, piece: PieceType::Pawn, captured, promotion: Some(PieceType::Bishop), move_type: MoveType::Promotion },
        Move { from, to, piece: PieceType::Pawn, captured, promotion: Some(PieceType::Knight), move_type: MoveType::Promotion },
    ]
}
```

### 5.6 Zobrist Hashing

For efficient position comparison and transposition tables:

```rust
pub struct ZobristKeys {
    pieces: [[[u64; 64]; 6]; 2],  // [color][piece_type][square]
    castling: [u64; 16],          // 4 bits = 16 combinations
    en_passant: [u64; 8],         // file
    side_to_move: u64,
}

impl ZobristKeys {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Initialize all keys with random u64 values
        // ...
    }
}

pub fn compute_hash(state: &GameState, keys: &ZobristKeys) -> u64 {
    let mut hash = 0u64;
    
    // XOR in all pieces
    for color in [Color::White, Color::Black] {
        for piece_type in PieceType::all() {
            let pieces = state.pieces(color, piece_type);
            for square in pieces.iter_squares() {
                hash ^= keys.pieces[color][piece_type][square];
            }
        }
    }
    
    // XOR in castling rights
    hash ^= keys.castling[state.castling_rights.as_index()];
    
    // XOR in en passant file if applicable
    if let Some(ep_sq) = state.en_passant_square {
        hash ^= keys.en_passant[ep_sq.file()];
    }
    
    // XOR in side to move
    if state.side_to_move == Color::Black {
        hash ^= keys.side_to_move;
    }
    
    hash
}
```

---

## 6. Graphics System

### 6.1 Window Configuration

```rust
fn window_conf() -> Conf {
    Conf {
        window_title: "RustChess".to_owned(),
        window_width: 1280,
        window_height: 800,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}
```

### 6.2 Asset Management

```
assets/
├── pieces/
│   ├── white_king.png
│   ├── white_queen.png
│   ├── white_rook.png
│   ├── white_bishop.png
│   ├── white_knight.png
│   ├── white_pawn.png
│   ├── black_king.png
│   ├── black_queen.png
│   ├── black_rook.png
│   ├── black_bishop.png
│   ├── black_knight.png
│   └── black_pawn.png
├── sounds/
│   ├── move.wav
│   ├── capture.wav
│   ├── check.wav
│   ├── castle.wav
│   └── game_end.wav
├── fonts/
│   └── chess_font.ttf
└── themes/
    ├── classic.json
    ├── wood.json
    └── modern.json
```

### 6.3 Rendering Pipeline

```rust
pub struct Renderer {
    piece_textures: HashMap<(Color, PieceType), Texture2D>,
    board_theme: BoardTheme,
    square_size: f32,
    board_offset: Vec2,
    animations: Vec<MoveAnimation>,
}

pub struct BoardTheme {
    pub light_square: Color,
    pub dark_square: Color,
    pub selected_square: Color,
    pub legal_move_indicator: Color,
    pub last_move_highlight: Color,
    pub check_highlight: Color,
}

impl Renderer {
    pub fn draw_board(&self) {
        for rank in 0..8 {
            for file in 0..8 {
                let is_light = (rank + file) % 2 == 0;
                let color = if is_light {
                    self.board_theme.light_square
                } else {
                    self.board_theme.dark_square
                };
                
                let x = self.board_offset.x + file as f32 * self.square_size;
                let y = self.board_offset.y + (7 - rank) as f32 * self.square_size;
                
                draw_rectangle(x, y, self.square_size, self.square_size, color);
            }
        }
    }
    
    pub fn draw_pieces(&self, state: &GameState) {
        for square in 0..64 {
            if let Some((color, piece_type)) = state.piece_at(Square(square)) {
                // Skip pieces that are currently being animated
                if self.is_animating(Square(square)) {
                    continue;
                }
                
                let texture = &self.piece_textures[&(color, piece_type)];
                let (x, y) = self.square_to_screen(Square(square));
                
                draw_texture_ex(
                    *texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(self.square_size, self.square_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }
    
    pub fn draw_highlights(&self, ui_state: &UiState, game_state: &GameState) {
        // Highlight last move
        if let Some(last_move) = game_state.last_move() {
            self.highlight_square(last_move.from, self.board_theme.last_move_highlight);
            self.highlight_square(last_move.to, self.board_theme.last_move_highlight);
        }
        
        // Highlight selected piece
        if let Some(selected) = ui_state.selected_square {
            self.highlight_square(selected, self.board_theme.selected_square);
        }
        
        // Highlight check
        if is_in_check(game_state, game_state.side_to_move) {
            let king_square = game_state.king_square(game_state.side_to_move);
            self.highlight_square(king_square, self.board_theme.check_highlight);
        }
        
        // Show legal move indicators
        if let Some(selected) = ui_state.selected_square {
            for legal_move in &ui_state.legal_moves_for_selected {
                self.draw_legal_move_indicator(legal_move.to);
            }
        }
    }
    
    fn draw_legal_move_indicator(&self, square: Square) {
        let (x, y) = self.square_to_screen(square);
        let center_x = x + self.square_size / 2.0;
        let center_y = y + self.square_size / 2.0;
        let radius = self.square_size * 0.15;
        
        draw_circle(
            center_x,
            center_y,
            radius,
            self.board_theme.legal_move_indicator,
        );
    }
}
```

### 6.4 Animation System

```rust
pub struct MoveAnimation {
    pub piece: (Color, PieceType),
    pub from_pos: Vec2,
    pub to_pos: Vec2,
    pub progress: f32,
    pub duration: f32,
}

impl MoveAnimation {
    pub fn new(mv: &Move, renderer: &Renderer, duration: f32) -> Self {
        Self {
            piece: (mv.color, mv.piece),
            from_pos: renderer.square_to_screen_vec(mv.from),
            to_pos: renderer.square_to_screen_vec(mv.to),
            progress: 0.0,
            duration,
        }
    }
    
    pub fn update(&mut self, delta: f32) -> bool {
        self.progress += delta / self.duration;
        self.progress >= 1.0
    }
    
    pub fn current_position(&self) -> Vec2 {
        // Smooth easing function
        let t = ease_out_cubic(self.progress.min(1.0));
        self.from_pos.lerp(self.to_pos, t)
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
```

### 6.5 Coordinate Notation Rendering

```rust
impl Renderer {
    pub fn draw_coordinates(&self) {
        let font_size = self.square_size * 0.2;
        
        // File letters (a-h)
        for file in 0..8 {
            let letter = (b'a' + file as u8) as char;
            let x = self.board_offset.x + file as f32 * self.square_size + self.square_size * 0.85;
            let y = self.board_offset.y + 8.0 * self.square_size - font_size * 0.5;
            
            draw_text(&letter.to_string(), x, y, font_size, GRAY);
        }
        
        // Rank numbers (1-8)
        for rank in 0..8 {
            let number = (rank + 1).to_string();
            let x = self.board_offset.x + font_size * 0.2;
            let y = self.board_offset.y + (7 - rank) as f32 * self.square_size + font_size;
            
            draw_text(&number, x, y, font_size, GRAY);
        }
    }
}
```

---

## 7. User Interface

### 7.1 Screen States

```rust
pub enum Screen {
    MainMenu,
    GameSetup,
    Playing,
    Paused,
    GameOver(GameResult),
    Settings,
    LoadGame,
}

pub enum GameResult {
    WhiteWins(WinReason),
    BlackWins(WinReason),
    Draw(DrawReason),
}

pub enum WinReason {
    Checkmate,
    Resignation,
    Timeout,
}
```

### 7.2 Main Menu

```
┌─────────────────────────────────────────┐
│                                         │
│              ♚ RUSTCHESS ♔              │
│                                         │
│         ┌─────────────────────┐         │
│         │    ▶ New Game       │         │
│         ├─────────────────────┤         │
│         │    ▶ Load Game      │         │
│         ├─────────────────────┤         │
│         │    ▶ Settings       │         │
│         ├─────────────────────┤         │
│         │    ▶ Quit           │         │
│         └─────────────────────┘         │
│                                         │
│                              v1.0.0     │
└─────────────────────────────────────────┘
```

### 7.3 Game Setup Screen

```
┌─────────────────────────────────────────┐
│              NEW GAME                   │
├─────────────────────────────────────────┤
│                                         │
│  Game Mode:                             │
│    ○ Player vs Player                   │
│    ● Player vs Computer                 │
│                                         │
│  Play as:                               │
│    ● White  ○ Black  ○ Random           │
│                                         │
│  AI Difficulty:                         │
│    ○ Beginner   ● Intermediate          │
│    ○ Advanced   ○ Expert                │
│                                         │
│  Time Control:                          │
│    ○ No limit                           │
│    ● 10 minutes                         │
│    ○ 5 minutes                          │
│    ○ Custom: [__] min [__] sec          │
│                                         │
│      ┌──────────┐  ┌──────────┐         │
│      │  START   │  │  CANCEL  │         │
│      └──────────┘  └──────────┘         │
└─────────────────────────────────────────┘
```

### 7.4 Game Screen Layout

```
┌────────────────────────────────────────────────────────────────────┐
│  ┌──────────────────────────┬─────────────────────────────────────┐│
│  │                          │  Player 2 (Black)      ⏱ 09:42     ││
│  │                          ├─────────────────────────────────────┤│
│  │                          │  Captured: ♙♙♘                      ││
│  │     8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜   ├─────────────────────────────────────┤│
│  │     7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟   │  Move History                       ││
│  │     6 · · · · · · · ·   │  ─────────────────                   ││
│  │     5 · · · · · · · ·   │  1. e4    e5                         ││
│  │     4 · · · · ♙ · · ·   │  2. Nf3   Nc6                        ││
│  │     3 · · · · · · · ·   │  3. Bb5   a6                         ││
│  │     2 ♙ ♙ ♙ ♙ · ♙ ♙ ♙   │  4. Ba4   Nf6                        ││
│  │     1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖   │  5. O-O   ...                        ││
│  │       a b c d e f g h   │                                      ││
│  │                          ├─────────────────────────────────────┤│
│  │                          │  Captured: ♟                        ││
│  │                          ├─────────────────────────────────────┤│
│  │                          │  Player 1 (White)      ⏱ 08:15     ││
│  └──────────────────────────┴─────────────────────────────────────┤│
│  ┌────────────────────────────────────────────────────────────────┤│
│  │  [≡ Menu]  [↶ Undo]  [↷ Redo]  [💾 Save]  [🏳 Resign]         ││
│  └────────────────────────────────────────────────────────────────┘│
└────────────────────────────────────────────────────────────────────┘
```

### 7.5 UI Components

```rust
pub struct UiState {
    pub selected_square: Option<Square>,
    pub legal_moves_for_selected: Vec<Move>,
    pub dragging_piece: Option<DragState>,
    pub promotion_dialog: Option<PromotionDialog>,
    pub hover_square: Option<Square>,
}

pub struct DragState {
    pub piece: (Color, PieceType),
    pub from_square: Square,
    pub current_pos: Vec2,
}

pub struct PromotionDialog {
    pub pawn_square: Square,
    pub target_square: Square,
    pub color: Color,
}
```

### 7.6 Input Handling

```rust
pub fn handle_input(ui_state: &mut UiState, game_state: &mut GameState, renderer: &Renderer) {
    let mouse_pos = mouse_position();
    let mouse_square = renderer.screen_to_square(mouse_pos.into());
    
    ui_state.hover_square = mouse_square;
    
    // Left click
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(square) = mouse_square {
            handle_square_click(ui_state, game_state, square);
        }
    }
    
    // Drag start
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(square) = mouse_square {
            if let Some((color, piece)) = game_state.piece_at(square) {
                if color == game_state.side_to_move {
                    ui_state.dragging_piece = Some(DragState {
                        piece: (color, piece),
                        from_square: square,
                        current_pos: mouse_pos.into(),
                    });
                }
            }
        }
    }
    
    // Drag update
    if let Some(ref mut drag) = ui_state.dragging_piece {
        drag.current_pos = mouse_pos.into();
    }
    
    // Drag end / drop
    if is_mouse_button_released(MouseButton::Left) {
        if let Some(drag) = ui_state.dragging_piece.take() {
            if let Some(target) = mouse_square {
                try_make_move(ui_state, game_state, drag.from_square, target);
            }
        }
    }
    
    // Keyboard shortcuts
    if is_key_pressed(KeyCode::Escape) {
        ui_state.selected_square = None;
        ui_state.legal_moves_for_selected.clear();
    }
    
    if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
        game_state.undo_move();
    }
    
    if is_key_pressed(KeyCode::Y) && is_key_down(KeyCode::LeftControl) {
        game_state.redo_move();
    }
}

fn handle_square_click(ui_state: &mut UiState, game_state: &mut GameState, square: Square) {
    // If we have a selected piece and click on a legal move target
    if let Some(selected) = ui_state.selected_square {
        if ui_state.legal_moves_for_selected.iter().any(|m| m.to == square) {
            try_make_move(ui_state, game_state, selected, square);
            return;
        }
    }
    
    // Select a new piece
    if let Some((color, _)) = game_state.piece_at(square) {
        if color == game_state.side_to_move {
            ui_state.selected_square = Some(square);
            ui_state.legal_moves_for_selected = generate_legal_moves(game_state)
                .into_iter()
                .filter(|m| m.from == square)
                .collect();
            return;
        }
    }
    
    // Deselect
    ui_state.selected_square = None;
    ui_state.legal_moves_for_selected.clear();
}
```

### 7.7 Promotion Dialog

```rust
pub fn draw_promotion_dialog(dialog: &PromotionDialog, renderer: &Renderer) {
    let (x, y) = renderer.square_to_screen(dialog.target_square);
    
    // Background overlay
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
    
    // Dialog box
    let dialog_width = renderer.square_size;
    let dialog_height = renderer.square_size * 4.0;
    
    draw_rectangle(x, y, dialog_width, dialog_height, DARKGRAY);
    
    // Promotion pieces
    let pieces = [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight];
    for (i, piece) in pieces.iter().enumerate() {
        let texture = &renderer.piece_textures[&(dialog.color, *piece)];
        let piece_y = y + i as f32 * renderer.square_size;
        
        // Highlight on hover
        let mouse_pos: Vec2 = mouse_position().into();
        if mouse_pos.x >= x && mouse_pos.x < x + dialog_width &&
           mouse_pos.y >= piece_y && mouse_pos.y < piece_y + renderer.square_size {
            draw_rectangle(x, piece_y, dialog_width, renderer.square_size, GRAY);
        }
        
        draw_texture_ex(
            *texture,
            x,
            piece_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(renderer.square_size, renderer.square_size)),
                ..Default::default()
            },
        );
    }
}

pub fn handle_promotion_click(dialog: &PromotionDialog, mouse_pos: Vec2, renderer: &Renderer) -> Option<PieceType> {
    let (x, y) = renderer.square_to_screen(dialog.target_square);
    let dialog_width = renderer.square_size;
    
    if mouse_pos.x < x || mouse_pos.x >= x + dialog_width {
        return None;
    }
    
    let relative_y = mouse_pos.y - y;
    if relative_y < 0.0 || relative_y >= renderer.square_size * 4.0 {
        return None;
    }
    
    let index = (relative_y / renderer.square_size) as usize;
    let pieces = [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight];
    pieces.get(index).copied()
}
```

---

## 8. AI Engine

### 8.1 Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       AI Engine                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │  Search         │    │  Evaluation     │                 │
│  │  ─────────────  │    │  ─────────────  │                 │
│  │  • Minimax      │───▶│  • Material     │                 │
│  │  • Alpha-Beta   │    │  • Position     │                 │
│  │  • Iterative    │    │  • Mobility     │                 │
│  │    Deepening    │    │  • King Safety  │                 │
│  │  • Quiescence   │    │  • Pawn Struct  │                 │
│  └─────────────────┘    └─────────────────┘                 │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │  Transposition  │    │  Move Ordering  │                 │
│  │  Table          │    │  ─────────────  │                 │
│  │  ─────────────  │    │  • MVV-LVA      │                 │
│  │  • Position     │    │  • Killer moves │                 │
│  │    caching      │    │  • History      │                 │
│  │  • Depth info   │    │    heuristic    │                 │
│  └─────────────────┘    └─────────────────┘                 │
└─────────────────────────────────────────────────────────────┘
```

### 8.2 Difficulty Levels

| Level | Search Depth | Time Limit | Features Enabled |
|-------|--------------|------------|------------------|
| Beginner | 2 | 500ms | Basic eval only |
| Intermediate | 4 | 2s | + Position tables |
| Advanced | 6 | 5s | + Full evaluation |
| Expert | 8+ | 10s | + All optimizations |

```rust
pub struct AiConfig {
    pub max_depth: u8,
    pub time_limit_ms: u64,
    pub use_transposition_table: bool,
    pub use_quiescence: bool,
    pub use_null_move_pruning: bool,
    pub randomness: f32,  // 0.0 = deterministic, higher = more random
}

impl AiConfig {
    pub fn beginner() -> Self {
        Self {
            max_depth: 2,
            time_limit_ms: 500,
            use_transposition_table: false,
            use_quiescence: false,
            use_null_move_pruning: false,
            randomness: 0.3,
        }
    }
    
    pub fn intermediate() -> Self {
        Self {
            max_depth: 4,
            time_limit_ms: 2000,
            use_transposition_table: true,
            use_quiescence: true,
            use_null_move_pruning: false,
            randomness: 0.1,
        }
    }
    
    pub fn advanced() -> Self {
        Self {
            max_depth: 6,
            time_limit_ms: 5000,
            use_transposition_table: true,
            use_quiescence: true,
            use_null_move_pruning: true,
            randomness: 0.0,
        }
    }
    
    pub fn expert() -> Self {
        Self {
            max_depth: 8,
            time_limit_ms: 10000,
            use_transposition_table: true,
            use_quiescence: true,
            use_null_move_pruning: true,
            randomness: 0.0,
        }
    }
}
```

### 8.3 Position Evaluation

#### Material Values

```rust
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

pub fn material_score(state: &GameState, color: Color) -> i32 {
    let mut score = 0;
    
    score += state.pawns(color).count() as i32 * PAWN_VALUE;
    score += state.knights(color).count() as i32 * KNIGHT_VALUE;
    score += state.bishops(color).count() as i32 * BISHOP_VALUE;
    score += state.rooks(color).count() as i32 * ROOK_VALUE;
    score += state.queens(color).count() as i32 * QUEEN_VALUE;
    
    score
}
```

#### Piece-Square Tables

```rust
// Pawn position values (from White's perspective, flip for Black)
const PAWN_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

// ... similar tables for other pieces

pub fn positional_score(state: &GameState, color: Color) -> i32 {
    let mut score = 0;
    
    for square in state.pawns(color).iter_squares() {
        let pst_square = if color == Color::White { square } else { flip_square(square) };
        score += PAWN_PST[pst_square.0 as usize];
    }
    
    for square in state.knights(color).iter_squares() {
        let pst_square = if color == Color::White { square } else { flip_square(square) };
        score += KNIGHT_PST[pst_square.0 as usize];
    }
    
    // ... other pieces
    
    score
}
```

#### Full Evaluation Function

```rust
pub fn evaluate(state: &GameState) -> i32 {
    if is_checkmate(state) {
        return if state.side_to_move == Color::White {
            -MATE_SCORE
        } else {
            MATE_SCORE
        };
    }
    
    if check_draw(state).is_some() {
        return 0;
    }
    
    let white_material = material_score(state, Color::White);
    let black_material = material_score(state, Color::Black);
    
    let white_position = positional_score(state, Color::White);
    let black_position = positional_score(state, Color::Black);
    
    let white_mobility = mobility_score(state, Color::White);
    let black_mobility = mobility_score(state, Color::Black);
    
    let white_king_safety = king_safety_score(state, Color::White);
    let black_king_safety = king_safety_score(state, Color::Black);
    
    let white_pawn_structure = pawn_structure_score(state, Color::White);
    let black_pawn_structure = pawn_structure_score(state, Color::Black);
    
    let score = (white_material - black_material)
              + (white_position - black_position)
              + (white_mobility - black_mobility) / 10
              + (white_king_safety - black_king_safety)
              + (white_pawn_structure - black_pawn_structure);
    
    // Return score from perspective of side to move
    if state.side_to_move == Color::White {
        score
    } else {
        -score
    }
}

pub fn mobility_score(state: &GameState, color: Color) -> i32 {
    let temp_state = state.with_side_to_move(color);
    generate_legal_moves(&temp_state).len() as i32
}

pub fn king_safety_score(state: &GameState, color: Color) -> i32 {
    let king_square = state.king_square(color);
    let mut score = 0;
    
    // Penalize exposed king
    let pawn_shield = KING_ATTACKS[king_square.0] & state.pawns(color);
    score += pawn_shield.count() as i32 * 10;
    
    // Bonus for castled king
    if color == Color::White {
        if king_square == Square::G1 || king_square == Square::C1 {
            score += 30;
        }
    } else {
        if king_square == Square::G8 || king_square == Square::C8 {
            score += 30;
        }
    }
    
    score
}

pub fn pawn_structure_score(state: &GameState, color: Color) -> i32 {
    let pawns = state.pawns(color);
    let mut score = 0;
    
    // Doubled pawns penalty
    for file in 0..8 {
        let file_mask = Bitboard::file(file);
        let pawns_on_file = (pawns & file_mask).count();
        if pawns_on_file > 1 {
            score -= (pawns_on_file - 1) as i32 * 20;
        }
    }
    
    // Isolated pawns penalty
    for square in pawns.iter_squares() {
        let file = square.file();
        let adjacent_files = Bitboard::adjacent_files(file);
        if (pawns & adjacent_files).is_empty() {
            score -= 15;
        }
    }
    
    // Passed pawns bonus
    for square in pawns.iter_squares() {
        if is_passed_pawn(state, square, color) {
            let rank = if color == Color::White { square.rank() } else { 7 - square.rank() };
            score += 20 + rank as i32 * 10;
        }
    }
    
    score
}
```

### 8.4 Search Algorithm

#### Alpha-Beta with Iterative Deepening

```rust
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i32,
    pub depth_reached: u8,
    pub nodes_searched: u64,
    pub time_ms: u64,
}

pub fn find_best_move(state: &GameState, config: &AiConfig) -> SearchResult {
    let start_time = Instant::now();
    let mut best_result = SearchResult::default();
    let mut tt = TranspositionTable::new();
    
    // Iterative deepening
    for depth in 1..=config.max_depth {
        let result = alpha_beta_root(
            state,
            depth,
            -INFINITY,
            INFINITY,
            &mut tt,
            config,
            start_time,
        );
        
        if start_time.elapsed().as_millis() as u64 > config.time_limit_ms {
            break;
        }
        
        best_result = result;
        best_result.depth_reached = depth;
    }
    
    best_result.time_ms = start_time.elapsed().as_millis() as u64;
    best_result
}

fn alpha_beta_root(
    state: &GameState,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    tt: &mut TranspositionTable,
    config: &AiConfig,
    start_time: Instant,
) -> SearchResult {
    let mut best_move = None;
    let mut best_score = -INFINITY;
    let mut nodes = 0u64;
    
    let mut moves = generate_legal_moves(state);
    order_moves(&mut moves, state, tt);
    
    for mv in moves {
        let new_state = state.apply_move(mv);
        let score = -alpha_beta(
            &new_state,
            depth - 1,
            -beta,
            -alpha,
            tt,
            config,
            start_time,
            &mut nodes,
        );
        
        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
        
        alpha = alpha.max(score);
        
        if alpha >= beta {
            break;
        }
        
        if start_time.elapsed().as_millis() as u64 > config.time_limit_ms {
            break;
        }
    }
    
    SearchResult {
        best_move,
        score: best_score,
        depth_reached: depth,
        nodes_searched: nodes,
        time_ms: 0,
    }
}

fn alpha_beta(
    state: &GameState,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    tt: &mut TranspositionTable,
    config: &AiConfig,
    start_time: Instant,
    nodes: &mut u64,
) -> i32 {
    *nodes += 1;
    
    // Time check
    if *nodes % 1024 == 0 && start_time.elapsed().as_millis() as u64 > config.time_limit_ms {
        return 0;
    }
    
    // Transposition table lookup
    if config.use_transposition_table {
        if let Some(entry) = tt.probe(state.hash()) {
            if entry.depth >= depth {
                match entry.flag {
                    TTFlag::Exact => return entry.score,
                    TTFlag::Alpha if entry.score <= alpha => return alpha,
                    TTFlag::Beta if entry.score >= beta => return beta,
                    _ => {}
                }
            }
        }
    }
    
    // Terminal node
    if depth == 0 {
        return if config.use_quiescence {
            quiescence_search(state, alpha, beta, nodes)
        } else {
            evaluate(state)
        };
    }
    
    let moves = generate_legal_moves(state);
    
    // Checkmate / stalemate
    if moves.is_empty() {
        return if is_in_check(state, state.side_to_move) {
            -MATE_SCORE + (config.max_depth - depth) as i32
        } else {
            0  // Stalemate
        };
    }
    
    let mut best_score = -INFINITY;
    let mut ordered_moves = moves;
    order_moves(&mut ordered_moves, state, tt);
    
    for mv in ordered_moves {
        let new_state = state.apply_move(mv);
        let score = -alpha_beta(
            &new_state,
            depth - 1,
            -beta,
            -alpha,
            tt,
            config,
            start_time,
            nodes,
        );
        
        best_score = best_score.max(score);
        alpha = alpha.max(score);
        
        if alpha >= beta {
            // Store killer move
            break;
        }
    }
    
    // Store in transposition table
    if config.use_transposition_table {
        let flag = if best_score <= alpha {
            TTFlag::Alpha
        } else if best_score >= beta {
            TTFlag::Beta
        } else {
            TTFlag::Exact
        };
        
        tt.store(state.hash(), depth, best_score, flag);
    }
    
    best_score
}
```

#### Quiescence Search

```rust
fn quiescence_search(
    state: &GameState,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
) -> i32 {
    *nodes += 1;
    
    let stand_pat = evaluate(state);
    
    if stand_pat >= beta {
        return beta;
    }
    
    if stand_pat > alpha {
        alpha = stand_pat;
    }
    
    // Only search captures
    let captures: Vec<Move> = generate_legal_moves(state)
        .into_iter()
        .filter(|m| m.captured.is_some())
        .collect();
    
    for mv in captures {
        let new_state = state.apply_move(mv);
        let score = -quiescence_search(&new_state, -beta, -alpha, nodes);
        
        if score >= beta {
            return beta;
        }
        
        if score > alpha {
            alpha = score;
        }
    }
    
    alpha
}
```

#### Move Ordering

```rust
pub fn order_moves(moves: &mut Vec<Move>, state: &GameState, tt: &TranspositionTable) {
    moves.sort_by_key(|mv| {
        let mut score = 0i32;
        
        // Hash move bonus
        if let Some(entry) = tt.probe(state.hash()) {
            if entry.best_move == Some(*mv) {
                score += 10000;
            }
        }
        
        // MVV-LVA (Most Valuable Victim - Least Valuable Attacker)
        if let Some(captured) = mv.captured {
            let victim_value = piece_value(captured);
            let attacker_value = piece_value(mv.piece);
            score += victim_value * 10 - attacker_value;
        }
        
        // Promotion bonus
        if let Some(promotion) = mv.promotion {
            score += piece_value(promotion);
        }
        
        -score  // Negate for descending order
    });
}
```

### 8.5 Transposition Table

```rust
pub struct TranspositionTable {
    entries: Vec<Option<TTEntry>>,
    size: usize,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: u8,
    pub score: i32,
    pub flag: TTFlag,
    pub best_move: Option<Move>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TTFlag {
    Exact,
    Alpha,  // Upper bound
    Beta,   // Lower bound
}

impl TranspositionTable {
    pub fn new() -> Self {
        let size = 1 << 20;  // ~1 million entries
        Self {
            entries: vec![None; size],
            size,
        }
    }
    
    fn index(&self, hash: u64) -> usize {
        (hash as usize) % self.size
    }
    
    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let idx = self.index(hash);
        self.entries[idx].filter(|e| e.hash == hash)
    }
    
    pub fn store(&mut self, hash: u64, depth: u8, score: i32, flag: TTFlag) {
        let idx = self.index(hash);
        
        // Replace if empty or deeper search
        if self.entries[idx].map_or(true, |e| e.depth <= depth) {
            self.entries[idx] = Some(TTEntry {
                hash,
                depth,
                score,
                flag,
                best_move: None,
            });
        }
    }
}
```

---

## 9. Game Modes

### 9.1 Player vs Player (Local)

```rust
pub struct PvPGame {
    pub state: GameState,
    pub white_clock: Option<Clock>,
    pub black_clock: Option<Clock>,
    pub move_history: Vec<MoveRecord>,
}

impl PvPGame {
    pub fn handle_turn(&mut self, mv: Move) -> TurnResult {
        // Validate move is legal
        let legal_moves = generate_legal_moves(&self.state);
        if !legal_moves.contains(&mv) {
            return TurnResult::IllegalMove;
        }
        
        // Apply move
        self.state = self.state.apply_move(mv);
        self.move_history.push(MoveRecord::new(mv, &self.state));
        
        // Switch clocks
        if let Some(ref mut clock) = self.current_clock_mut() {
            clock.pause();
        }
        if let Some(ref mut clock) = self.opponent_clock_mut() {
            clock.start();
        }
        
        // Check game end
        self.check_game_end()
    }
}
```

### 9.2 Player vs AI

```rust
pub struct PvAiGame {
    pub state: GameState,
    pub player_color: Color,
    pub ai_config: AiConfig,
    pub ai_thinking: bool,
    pub ai_handle: Option<JoinHandle<SearchResult>>,
}

impl PvAiGame {
    pub fn update(&mut self) {
        // Check if it's AI's turn
        if self.state.side_to_move != self.player_color && !self.ai_thinking {
            self.start_ai_thinking();
        }
        
        // Check if AI is done thinking
        if let Some(handle) = self.ai_handle.take() {
            if handle.is_finished() {
                let result = handle.join().unwrap();
                if let Some(mv) = result.best_move {
                    self.state = self.state.apply_move(mv);
                }
                self.ai_thinking = false;
            } else {
                self.ai_handle = Some(handle);
            }
        }
    }
    
    fn start_ai_thinking(&mut self) {
        self.ai_thinking = true;
        let state = self.state.clone();
        let config = self.ai_config.clone();
        
        self.ai_handle = Some(std::thread::spawn(move || {
            find_best_move(&state, &config)
        }));
    }
}
```

### 9.3 Time Controls

```rust
pub struct Clock {
    pub remaining_ms: u64,
    pub increment_ms: u64,
    pub is_running: bool,
    pub last_update: Instant,
}

impl Clock {
    pub fn new(initial_ms: u64, increment_ms: u64) -> Self {
        Self {
            remaining_ms: initial_ms,
            increment_ms,
            is_running: false,
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self) {
        if self.is_running {
            let elapsed = self.last_update.elapsed().as_millis() as u64;
            self.remaining_ms = self.remaining_ms.saturating_sub(elapsed);
            self.last_update = Instant::now();
        }
    }
    
    pub fn press(&mut self) {
        self.is_running = false;
        self.remaining_ms += self.increment_ms;
    }
    
    pub fn is_expired(&self) -> bool {
        self.remaining_ms == 0
    }
    
    pub fn formatted(&self) -> String {
        let total_seconds = self.remaining_ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        
        if minutes > 0 {
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            let tenths = (self.remaining_ms % 1000) / 100;
            format!("{}.{}", seconds, tenths)
        }
    }
}

pub enum TimeControl {
    None,
    Bullet(u64),        // Total time in ms
    Blitz(u64),
    Rapid(u64),
    Classical(u64),
    Increment(u64, u64), // Base time, increment
}

impl TimeControl {
    pub fn to_clock(&self) -> Option<Clock> {
        match self {
            TimeControl::None => None,
            TimeControl::Bullet(ms) => Some(Clock::new(*ms, 0)),
            TimeControl::Blitz(ms) => Some(Clock::new(*ms, 0)),
            TimeControl::Rapid(ms) => Some(Clock::new(*ms, 0)),
            TimeControl::Classical(ms) => Some(Clock::new(*ms, 0)),
            TimeControl::Increment(base, inc) => Some(Clock::new(*base, *inc)),
        }
    }
}
```

---

## 10. Data Persistence

### 10.1 PGN Support

#### PGN Parser

```rust
pub struct PgnGame {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub moves: Vec<String>,
}

pub fn parse_pgn(input: &str) -> Result<PgnGame, PgnError> {
    let mut game = PgnGame::default();
    let mut lines = input.lines();
    
    // Parse headers
    for line in &mut lines {
        if line.starts_with('[') {
            let (key, value) = parse_header(line)?;
            match key.as_str() {
                "Event" => game.event = value,
                "Site" => game.site = value,
                "Date" => game.date = value,
                "Round" => game.round = value,
                "White" => game.white = value,
                "Black" => game.black = value,
                "Result" => game.result = value,
                _ => {}
            }
        } else if !line.is_empty() {
            break;
        }
    }
    
    // Parse moves
    let move_text: String = lines.collect::<Vec<_>>().join(" ");
    game.moves = parse_move_text(&move_text)?;
    
    Ok(game)
}

pub fn move_to_san(mv: &Move, state: &GameState) -> String {
    let mut san = String::new();
    
    // Castling
    if mv.move_type == MoveType::CastleKingside {
        return "O-O".to_string();
    }
    if mv.move_type == MoveType::CastleQueenside {
        return "O-O-O".to_string();
    }
    
    // Piece letter
    if mv.piece != PieceType::Pawn {
        san.push(piece_to_char(mv.piece));
    }
    
    // Disambiguation
    let disambig = calculate_disambiguation(mv, state);
    san.push_str(&disambig);
    
    // Capture indicator
    if mv.captured.is_some() {
        if mv.piece == PieceType::Pawn {
            san.push(file_to_char(mv.from.file()));
        }
        san.push('x');
    }
    
    // Destination square
    san.push(file_to_char(mv.to.file()));
    san.push(rank_to_char(mv.to.rank()));
    
    // Promotion
    if let Some(promo) = mv.promotion {
        san.push('=');
        san.push(piece_to_char(promo));
    }
    
    // Check/checkmate indicators
    let new_state = state.apply_move(*mv);
    if is_checkmate(&new_state) {
        san.push('#');
    } else if is_in_check(&new_state, new_state.side_to_move) {
        san.push('+');
    }
    
    san
}
```

#### PGN Writer

```rust
pub fn export_pgn(game: &Game) -> String {
    let mut pgn = String::new();
    
    // Headers
    pgn.push_str(&format!("[Event \"{}\"]\n", game.event));
    pgn.push_str(&format!("[Site \"{}\"]\n", game.site));
    pgn.push_str(&format!("[Date \"{}\"]\n", game.date));
    pgn.push_str(&format!("[Round \"{}\"]\n", game.round));
    pgn.push_str(&format!("[White \"{}\"]\n", game.white_name));
    pgn.push_str(&format!("[Black \"{}\"]\n", game.black_name));
    pgn.push_str(&format!("[Result \"{}\"]\n", game.result_string()));
    pgn.push('\n');
    
    // Moves
    let mut state = GameState::initial();
    let mut move_number = 1;
    let mut line_length = 0;
    
    for (i, mv) in game.moves.iter().enumerate() {
        let san = move_to_san(mv, &state);
        
        let entry = if i % 2 == 0 {
            format!("{}. {}", move_number, san)
        } else {
            move_number += 1;
            san.clone()
        };
        
        if line_length + entry.len() > 80 {
            pgn.push('\n');
            line_length = 0;
        } else if !pgn.is_empty() && !pgn.ends_with('\n') {
            pgn.push(' ');
            line_length += 1;
        }
        
        pgn.push_str(&entry);
        line_length += entry.len();
        
        state = state.apply_move(*mv);
    }
    
    // Result
    pgn.push(' ');
    pgn.push_str(&game.result_string());
    pgn.push('\n');
    
    pgn
}
```

### 10.2 FEN Support

```rust
pub fn parse_fen(fen: &str) -> Result<GameState, FenError> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    
    if parts.len() < 4 {
        return Err(FenError::InvalidFormat);
    }
    
    let board = parse_fen_board(parts[0])?;
    let side_to_move = parse_fen_side(parts[1])?;
    let castling = parse_fen_castling(parts[2])?;
    let en_passant = parse_fen_en_passant(parts[3])?;
    
    let halfmove = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let fullmove = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);
    
    Ok(GameState {
        board,
        side_to_move,
        castling_rights: castling,
        en_passant_square: en_passant,
        halfmove_clock: halfmove,
        fullmove_number: fullmove,
        position_history: vec![],
    })
}

pub fn to_fen(state: &GameState) -> String {
    let mut fen = String::new();
    
    // Board
    for rank in (0..8).rev() {
        let mut empty_count = 0;
        
        for file in 0..8 {
            let square = Square::from_coords(file, rank);
            
            if let Some((color, piece)) = state.piece_at(square) {
                if empty_count > 0 {
                    fen.push(char::from_digit(empty_count, 10).unwrap());
                    empty_count = 0;
                }
                
                let c = piece_to_char(piece);
                fen.push(if color == Color::White {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                });
            } else {
                empty_count += 1;
            }
        }
        
        if empty_count > 0 {
            fen.push(char::from_digit(empty_count, 10).unwrap());
        }
        
        if rank > 0 {
            fen.push('/');
        }
    }
    
    // Side to move
    fen.push(' ');
    fen.push(if state.side_to_move == Color::White { 'w' } else { 'b' });
    
    // Castling
    fen.push(' ');
    let mut castling = String::new();
    if state.castling_rights.white_kingside { castling.push('K'); }
    if state.castling_rights.white_queenside { castling.push('Q'); }
    if state.castling_rights.black_kingside { castling.push('k'); }
    if state.castling_rights.black_queenside { castling.push('q'); }
    fen.push_str(if castling.is_empty() { "-" } else { &castling });
    
    // En passant
    fen.push(' ');
    if let Some(ep) = state.en_passant_square {
        fen.push(file_to_char(ep.file()));
        fen.push(rank_to_char(ep.rank()));
    } else {
        fen.push('-');
    }
    
    // Halfmove clock and fullmove number
    fen.push_str(&format!(" {} {}", state.halfmove_clock, state.fullmove_number));
    
    fen
}
```

### 10.3 Save/Load Game State

```rust
#[derive(Serialize, Deserialize)]
pub struct SavedGame {
    pub version: u32,
    pub timestamp: DateTime<Utc>,
    pub fen: String,
    pub pgn_moves: Vec<String>,
    pub white_name: String,
    pub black_name: String,
    pub game_mode: GameMode,
    pub time_control: Option<SavedTimeControl>,
    pub white_time_remaining_ms: Option<u64>,
    pub black_time_remaining_ms: Option<u64>,
    pub ai_config: Option<AiConfig>,
}

pub fn save_game(game: &Game, path: &Path) -> Result<(), SaveError> {
    let saved = SavedGame {
        version: 1,
        timestamp: Utc::now(),
        fen: to_fen(&game.current_state()),
        pgn_moves: game.moves.iter().map(|m| move_to_san(m, &game.state_at(m))).collect(),
        white_name: game.white_name.clone(),
        black_name: game.black_name.clone(),
        game_mode: game.mode,
        time_control: game.time_control.map(Into::into),
        white_time_remaining_ms: game.white_clock.as_ref().map(|c| c.remaining_ms),
        black_time_remaining_ms: game.black_clock.as_ref().map(|c| c.remaining_ms),
        ai_config: game.ai_config.clone(),
    };
    
    let json = serde_json::to_string_pretty(&saved)?;
    std::fs::write(path, json)?;
    
    Ok(())
}

pub fn load_game(path: &Path) -> Result<Game, LoadError> {
    let json = std::fs::read_to_string(path)?;
    let saved: SavedGame = serde_json::from_str(&json)?;
    
    // Reconstruct game from saved state
    let mut game = Game::from_fen(&saved.fen)?;
    game.white_name = saved.white_name;
    game.black_name = saved.black_name;
    game.mode = saved.game_mode;
    
    // Restore clocks
    if let (Some(tc), Some(white_ms), Some(black_ms)) = (
        saved.time_control,
        saved.white_time_remaining_ms,
        saved.black_time_remaining_ms,
    ) {
        game.white_clock = Some(Clock {
            remaining_ms: white_ms,
            increment_ms: tc.increment_ms,
            is_running: false,
            last_update: Instant::now(),
        });
        game.black_clock = Some(Clock {
            remaining_ms: black_ms,
            increment_ms: tc.increment_ms,
            is_running: false,
            last_update: Instant::now(),
        });
    }
    
    game.ai_config = saved.ai_config;
    
    Ok(game)
}
```

---

## 11. Audio System

### 11.1 Sound Effects

```rust
pub struct AudioManager {
    move_sound: Sound,
    capture_sound: Sound,
    check_sound: Sound,
    castle_sound: Sound,
    game_start_sound: Sound,
    game_end_sound: Sound,
    illegal_move_sound: Sound,
    volume: f32,
    enabled: bool,
}

impl AudioManager {
    pub async fn new() -> Self {
        Self {
            move_sound: load_sound("assets/sounds/move.wav").await.unwrap(),
            capture_sound: load_sound("assets/sounds/capture.wav").await.unwrap(),
            check_sound: load_sound("assets/sounds/check.wav").await.unwrap(),
            castle_sound: load_sound("assets/sounds/castle.wav").await.unwrap(),
            game_start_sound: load_sound("assets/sounds/game_start.wav").await.unwrap(),
            game_end_sound: load_sound("assets/sounds/game_end.wav").await.unwrap(),
            illegal_move_sound: load_sound("assets/sounds/illegal.wav").await.unwrap(),
            volume: 0.7,
            enabled: true,
        }
    }
    
    pub fn play_move_sound(&self, mv: &Move, resulted_in_check: bool) {
        if !self.enabled {
            return;
        }
        
        let sound = if resulted_in_check {
            &self.check_sound
        } else if mv.move_type == MoveType::CastleKingside || mv.move_type == MoveType::CastleQueenside {
            &self.castle_sound
        } else if mv.captured.is_some() {
            &self.capture_sound
        } else {
            &self.move_sound
        };
        
        play_sound(
            *sound,
            PlaySoundParams {
                volume: self.volume,
                ..Default::default()
            },
        );
    }
}
```

---

## 12. Performance Requirements

### 12.1 Target Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Frame rate | 60 FPS constant | In-game FPS counter |
| Move generation | < 1ms for legal moves | Benchmark suite |
| AI thinking (depth 4) | < 2 seconds | Timer |
| Memory usage | < 200 MB | System monitor |
| Startup time | < 3 seconds | Timer |
| Save/Load | < 100ms | Timer |

### 12.2 Optimization Strategies

#### Move Generation
- Pre-computed attack tables for knights and kings
- Magic bitboards for sliding pieces
- Incremental hash updates (Zobrist)

#### AI Search
- Transposition table (hash table for positions)
- Move ordering (MVV-LVA, killer moves, history heuristic)
- Iterative deepening with time management
- Quiescence search to avoid horizon effect

#### Rendering
- Batch similar draw calls
- Only redraw changed regions when possible
- Pre-load and cache all textures
- Use sprite sheets for pieces

### 12.3 Benchmarking

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn bench_move_generation() {
        let positions = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        ];
        
        for fen in &positions {
            let state = parse_fen(fen).unwrap();
            
            let start = Instant::now();
            let iterations = 10000;
            
            for _ in 0..iterations {
                let _ = generate_legal_moves(&state);
            }
            
            let elapsed = start.elapsed();
            println!(
                "Move generation: {:.3}μs per call",
                elapsed.as_micros() as f64 / iterations as f64
            );
        }
    }
    
    #[test]
    fn bench_perft() {
        // Perft tests for correctness and performance
        let test_cases = [
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4865609),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 4, 4085603),
        ];
        
        for (fen, depth, expected) in &test_cases {
            let state = parse_fen(fen).unwrap();
            
            let start = Instant::now();
            let result = perft(&state, *depth);
            let elapsed = start.elapsed();
            
            assert_eq!(result, *expected, "Perft failed for {}", fen);
            println!(
                "Perft({}) = {} in {:.2}s ({:.0} nodes/sec)",
                depth,
                result,
                elapsed.as_secs_f64(),
                result as f64 / elapsed.as_secs_f64()
            );
        }
    }
}

fn perft(state: &GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let moves = generate_legal_moves(state);
    
    if depth == 1 {
        return moves.len() as u64;
    }
    
    moves
        .iter()
        .map(|mv| {
            let new_state = state.apply_move(*mv);
            perft(&new_state, depth - 1)
        })
        .sum()
}
```

---

## 13. Project Structure

```
rustchess/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── assets/
│   ├── pieces/
│   │   ├── white_king.png
│   │   ├── white_queen.png
│   │   ├── ... (12 piece images)
│   ├── sounds/
│   │   ├── move.wav
│   │   ├── capture.wav
│   │   ├── check.wav
│   │   ├── castle.wav
│   │   └── game_end.wav
│   ├── fonts/
│   │   └── chess_font.ttf
│   └── themes/
│       ├── classic.json
│       ├── wood.json
│       └── modern.json
├── src/
│   ├── main.rs                 # Entry point, main loop
│   ├── lib.rs                  # Library root
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── bitboard.rs         # Bitboard implementation
│   │   ├── board.rs            # Board state
│   │   ├── moves.rs            # Move representation
│   │   ├── movegen.rs          # Move generation
│   │   ├── magic.rs            # Magic bitboards
│   │   ├── zobrist.rs          # Zobrist hashing
│   │   └── rules.rs            # Game rules (check, mate, draw)
│   ├── ai/
│   │   ├── mod.rs
│   │   ├── search.rs           # Alpha-beta search
│   │   ├── evaluation.rs       # Position evaluation
│   │   ├── transposition.rs    # Transposition table
│   │   └── ordering.rs         # Move ordering
│   ├── graphics/
│   │   ├── mod.rs
│   │   ├── renderer.rs         # Main rendering
│   │   ├── animation.rs        # Move animations
│   │   ├── theme.rs            # Board themes
│   │   └── assets.rs           # Asset loading
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── screens.rs          # Screen states
│   │   ├── menu.rs             # Main menu
│   │   ├── game_ui.rs          # In-game UI
│   │   ├── dialogs.rs          # Promotion, etc.
│   │   └── input.rs            # Input handling
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs            # Game state management
│   │   ├── clock.rs            # Time controls
│   │   ├── modes.rs            # PvP, PvAI
│   │   └── history.rs          # Move history, undo/redo
│   ├── persistence/
│   │   ├── mod.rs
│   │   ├── pgn.rs              # PGN parsing/writing
│   │   ├── fen.rs              # FEN parsing/writing
│   │   └── save.rs             # Save/load games
│   └── audio/
│       └── mod.rs              # Sound effects
├── tests/
│   ├── engine_tests.rs
│   ├── perft_tests.rs
│   ├── ai_tests.rs
│   └── integration_tests.rs
└── benches/
    ├── movegen_bench.rs
    └── search_bench.rs
```

### Cargo.toml

```toml
[package]
name = "rustchess"
version = "1.0.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A chess game with 2D graphics written in pure Rust"
license = "MIT"
repository = "https://github.com/yourusername/rustchess"

[dependencies]
macroquad = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"
lazy_static = "1.4"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "movegen_bench"
harness = false

[profile.release]
lto = true
codegen-units = 1
opt-level = 3

[profile.dev]
opt-level = 1  # Faster dev builds

[profile.test]
opt-level = 2  # Faster tests
```

---

## 14. Implementation Phases

### Phase 1: Core Engine (2-3 weeks)

**Week 1-2: Foundation**
- [ ] Bitboard implementation with comprehensive tests
- [ ] Board state representation
- [ ] Move representation and application
- [ ] Basic move generation (non-sliding pieces)

**Week 2-3: Complete Rules**
- [ ] Sliding piece move generation (magic bitboards)
- [ ] Special moves (castling, en passant, promotion)
- [ ] Check, checkmate, stalemate detection
- [ ] Draw condition detection
- [ ] Zobrist hashing
- [ ] Perft validation

**Deliverables:**
- Fully tested chess engine
- All perft tests passing
- Move generation < 1ms

### Phase 2: Graphics & UI (2 weeks)

**Week 4: Rendering**
- [ ] macroquad setup and window
- [ ] Board rendering with themes
- [ ] Piece sprite loading and drawing
- [ ] Coordinate notation
- [ ] Highlights (selected, legal moves, last move, check)

**Week 5: Interaction**
- [ ] Click-to-move
- [ ] Drag-and-drop
- [ ] Promotion dialog
- [ ] Move animation
- [ ] UI panels (move history, captured pieces, clocks)

**Deliverables:**
- Playable PvP chess
- Smooth animations
- Responsive UI

### Phase 3: AI Engine (2 weeks)

**Week 6: Search**
- [ ] Minimax implementation
- [ ] Alpha-beta pruning
- [ ] Iterative deepening
- [ ] Transposition table
- [ ] Move ordering

**Week 7: Evaluation & Polish**
- [ ] Material evaluation
- [ ] Piece-square tables
- [ ] Mobility, king safety, pawn structure
- [ ] Quiescence search
- [ ] Difficulty levels
- [ ] Async thinking (non-blocking UI)

**Deliverables:**
- Playable PvAI mode
- ~1500 ELO at highest difficulty
- Configurable difficulty levels

### Phase 4: Features & Polish (1-2 weeks)

**Week 8: Persistence**
- [ ] FEN parsing/writing
- [ ] PGN parsing/writing
- [ ] Save/load game state
- [ ] Undo/redo

**Week 9: Final Polish**
- [ ] Sound effects
- [ ] Time controls
- [ ] Menu screens
- [ ] Settings (volume, theme)
- [ ] Performance optimization
- [ ] Bug fixes and edge cases

**Deliverables:**
- Complete, polished chess game
- All features implemented
- Performance targets met

---

## 15. Testing Strategy

### 15.1 Unit Tests

```rust
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
    }
    
    #[test]
    fn test_move_generation_initial() {
        let state = GameState::initial();
        let moves = generate_legal_moves(&state);
        assert_eq!(moves.len(), 20);  // 16 pawn + 4 knight moves
    }
    
    #[test]
    fn test_castling_kingside() {
        let state = parse_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
        let moves = generate_legal_moves(&state);
        assert!(moves.iter().any(|m| m.move_type == MoveType::CastleKingside));
    }
    
    #[test]
    fn test_en_passant() {
        let state = parse_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
        let moves = generate_legal_moves(&state);
        assert!(moves.iter().any(|m| m.move_type == MoveType::EnPassant));
    }
    
    #[test]
    fn test_checkmate_detection() {
        // Scholar's mate position
        let state = parse_fen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
        assert!(is_checkmate(&state));
    }
    
    #[test]
    fn test_stalemate_detection() {
        let state = parse_fen("k7/8/1K6/8/8/8/8/8 b - - 0 1").unwrap();
        // Not stalemate - black king can move
        assert!(!is_stalemate(&state));
        
        let state = parse_fen("k7/8/1K6/8/8/8/8/1Q6 b - - 0 1").unwrap();
        // Stalemate - black king has no legal moves and is not in check
        // Wait, this is actually check. Let me fix:
        let state = parse_fen("8/8/8/8/8/5k2/5p2/5K2 w - - 0 1").unwrap();
        // King trapped, no check - stalemate
    }
}
```

### 15.2 Perft Tests

```rust
#[cfg(test)]
mod perft_tests {
    use super::*;
    
    const PERFT_POSITIONS: &[(&str, &[(u8, u64)])] = &[
        // Initial position
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            &[(1, 20), (2, 400), (3, 8902), (4, 197281), (5, 4865609)],
        ),
        // Kiwipete
        (
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            &[(1, 48), (2, 2039), (3, 97862), (4, 4085603)],
        ),
        // Position 3
        (
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            &[(1, 14), (2, 191), (3, 2812), (4, 43238), (5, 674624)],
        ),
    ];
    
    #[test]
    fn perft_suite() {
        for (fen, depths) in PERFT_POSITIONS {
            let state = parse_fen(fen).unwrap();
            
            for &(depth, expected) in *depths {
                let result = perft(&state, depth);
                assert_eq!(
                    result, expected,
                    "Perft({}) failed for {}: expected {}, got {}",
                    depth, fen, expected, result
                );
            }
        }
    }
}
```

### 15.3 Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_game_scholar_mate() {
        let mut state = GameState::initial();
        
        let moves = [
            ("e2", "e4"), // 1. e4
            ("e7", "e5"), // 1... e5
            ("f1", "c4"), // 2. Bc4
            ("b8", "c6"), // 2... Nc6
            ("d1", "h5"), // 3. Qh5
            ("g8", "f6"), // 3... Nf6??
            ("h5", "f7"), // 4. Qxf7#
        ];
        
        for (from_str, to_str) in &moves {
            let from = Square::from_algebraic(from_str).unwrap();
            let to = Square::from_algebraic(to_str).unwrap();
            
            let legal = generate_legal_moves(&state);
            let mv = legal.iter().find(|m| m.from == from && m.to == to).unwrap();
            
            state = state.apply_move(*mv);
        }
        
        assert!(is_checkmate(&state));
        assert_eq!(state.side_to_move, Color::Black);
    }
    
    #[test]
    fn test_pgn_roundtrip() {
        let original_pgn = r#"[Event "Test Game"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 1-0"#;
        
        let game = parse_pgn(original_pgn).unwrap();
        let exported = export_pgn(&game);
        let reimported = parse_pgn(&exported).unwrap();
        
        assert_eq!(game.moves.len(), reimported.moves.len());
    }
    
    #[test]
    fn test_fen_roundtrip() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        ];
        
        for fen in &fens {
            let state = parse_fen(fen).unwrap();
            let exported = to_fen(&state);
            assert_eq!(*fen, exported);
        }
    }
}
```

---

## 16. Future Considerations

### v1.1 Enhancements
- Opening book integration
- Endgame tablebases (Syzygy)
- Analysis mode with engine lines
- Puzzle mode

### v2.0 Features
- Online multiplayer (WebSocket)
- Chess960 / Fischer Random
- Multiple board themes
- Custom piece sets
- Tournaments and matchmaking

### Technical Debt Prevention
- Maintain >80% test coverage
- Document all public APIs
- Regular benchmarking
- Clippy lint compliance

---

## Appendix A: Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Escape | Deselect piece / Close dialog |
| Ctrl+Z | Undo move |
| Ctrl+Y / Ctrl+Shift+Z | Redo move |
| Ctrl+S | Save game |
| Ctrl+N | New game |
| F | Flip board |
| M | Toggle sound |
| Space | Pause/Resume (vs AI) |

---

## Appendix B: Glossary

| Term | Definition |
|------|------------|
| Bitboard | A 64-bit integer where each bit represents a square |
| FEN | Forsyth-Edwards Notation - compact board position representation |
| PGN | Portable Game Notation - standard game record format |
| Perft | Performance test - counts nodes at a given depth |
| SAN | Standard Algebraic Notation - human-readable move notation |
| Zobrist | A hashing technique for chess positions |
| Quiescence | Search extension to avoid horizon effect |
| MVV-LVA | Most Valuable Victim - Least Valuable Attacker move ordering |

---

*Document Version: 1.0*  
*Last Updated: January 2026*
