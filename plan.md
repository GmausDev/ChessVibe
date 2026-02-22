# Project Plan

## Overview
A fully-featured chess game built in Rust using macroquad for 2D graphics. Features include complete chess rule implementation, player vs player mode, and AI opponent.

**Reference:** `chess-game-prd.md`

---

## Task List

```json
[
  {
    "category": "setup",
    "description": "Initialize Rust project with Cargo and dependencies",
    "steps": [
      "Create Cargo.toml with project metadata",
      "Add macroquad dependency",
      "Add serde and serde_json dependencies",
      "Add rand dependency",
      "Create src/main.rs entry point",
      "Verify cargo check passes"
    ],
    "passes": true
  },
  {
    "category": "setup",
    "description": "Create basic window and main loop",
    "steps": [
      "Set up macroquad window configuration (1280x800)",
      "Implement async main function",
      "Create basic game loop with clear background",
      "Verify window opens and displays correctly"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement core data types (Color, PieceType, Square)",
    "steps": [
      "Create Color enum (White, Black)",
      "Create PieceType enum (Pawn, Knight, Bishop, Rook, Queen, King)",
      "Create Square struct with file/rank conversion",
      "Add algebraic notation conversion methods"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement Bitboard for efficient board representation",
    "steps": [
      "Create Bitboard struct wrapping u64",
      "Implement set, clear, test bit operations",
      "Implement population count",
      "Implement bit iteration"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement BoardState with piece positions",
    "steps": [
      "Create BoardState struct with bitboards for each piece type/color",
      "Implement piece_at() method",
      "Implement initial position setup",
      "Implement move application"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement Move struct and move types",
    "steps": [
      "Create Move struct with from/to squares",
      "Add captured piece field",
      "Add promotion piece field",
      "Create MoveType enum for special moves"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement pawn move generation",
    "steps": [
      "Generate single pawn pushes",
      "Generate double pawn pushes from starting rank",
      "Generate pawn captures",
      "Generate en passant captures",
      "Generate promotion moves"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement knight and king move generation",
    "steps": [
      "Create pre-computed knight attack tables",
      "Create pre-computed king attack tables",
      "Generate knight moves filtering blocked squares",
      "Generate king moves filtering blocked squares"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement sliding piece move generation",
    "steps": [
      "Implement ray-based move generation for bishops",
      "Implement ray-based move generation for rooks",
      "Implement queen moves as union of bishop and rook",
      "Stop rays at blocking pieces"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement check detection and legal move filtering",
    "steps": [
      "Implement is_square_attacked function",
      "Implement is_in_check using king square",
      "Filter pseudo-legal moves that leave king in check",
      "Create generate_legal_moves function"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement castling",
    "steps": [
      "Track castling rights in game state",
      "Validate castling conditions (rights, clear path, no check)",
      "Generate kingside castling move",
      "Generate queenside castling move",
      "Update castling rights when king/rook moves"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement game end detection",
    "steps": [
      "Implement is_checkmate (in check + no legal moves)",
      "Implement is_stalemate (not in check + no legal moves)",
      "Implement 50-move rule detection",
      "Implement threefold repetition detection",
      "Implement insufficient material detection"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Render chess board",
    "steps": [
      "Calculate board size and position based on window",
      "Draw alternating light/dark squares",
      "Use configurable colors for theming",
      "Center board in window"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Render chess pieces",
    "steps": [
      "Draw simple geometric shapes for pieces or load textures",
      "Position pieces correctly on their squares",
      "Render all pieces based on board state"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Render board coordinates",
    "steps": [
      "Draw file letters (a-h) below the board",
      "Draw rank numbers (1-8) beside the board",
      "Use readable font size"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement square selection and highlighting",
    "steps": [
      "Convert mouse position to square coordinates",
      "Highlight selected square",
      "Highlight legal move targets for selected piece",
      "Highlight last move from/to squares"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement click-to-move gameplay",
    "steps": [
      "Click to select own piece",
      "Click legal target to execute move",
      "Update board state after move",
      "Switch turns between players"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement promotion dialog",
    "steps": [
      "Detect when pawn reaches back rank",
      "Display promotion piece options",
      "Wait for player selection",
      "Complete move with chosen piece"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Display game status and messages",
    "steps": [
      "Show whose turn it is",
      "Display check warning",
      "Announce checkmate with winner",
      "Announce draw with reason"
    ],
    "passes": true
  },
  {
    "category": "feature",
    "description": "Implement FEN support",
    "steps": [
      "Parse FEN string to create board state",
      "Export board state to FEN string",
      "Load specific positions for testing"
    ],
    "passes": true
  },
  {
    "category": "testing",
    "description": "Verify complete game can be played",
    "steps": [
      "Play through a full game to checkmate",
      "Test all special moves work correctly",
      "Verify draw conditions trigger correctly",
      "Check for any visual glitches or bugs"
    ],
    "passes": true
  }
]
```
