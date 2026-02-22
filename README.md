# ChessVibe

A fully-featured chess game built entirely in Rust - created as an experiment using [Claude](https://claude.ai) and the [Ralph Loops](https://github.com/anthropics/claude-code) Claude Code plugin.

## What is this?

This project is an experiment in AI-assisted development. The entire codebase was generated through iterative conversations with Claude, using the Ralph Loops plugin to manage the development workflow. It's a test of how far you can push AI-assisted coding for a non-trivial project.

**Spoiler:** It works pretty well! We ended up with a complete, playable chess game with all the rules implemented correctly.

## Features

- Full FIDE-compliant chess rules
- Bitboard-based engine for efficient move generation
- Click-to-move interface with legal move highlighting
- All special moves: castling, en passant, pawn promotion
- Check, checkmate, and stalemate detection
- 50-move rule and insufficient material draws
- FEN position import/export
- Responsive UI that scales with window size

## Tech Stack

- **Rust** (2021 edition) - because why not make a chess game in a systems language?
- **macroquad** - simple 2D graphics, no fuss
- **serde** - for FEN serialization

## Running It

```bash
cargo run --release
```

## Controls

- **Click** a piece to select it, click a highlighted square to move
- **R** - Reset the board
- **ESC** - Deselect / quit

## The Experiment

This was built to test the Ralph Loops workflow - an iterative approach to building software with Claude. The idea is to:

1. Start with a PRD (product requirements document)
2. Break it into tasks
3. Let Claude implement each piece
4. Iterate until it works

All 2,800+ lines of Rust code, 32 passing tests, and the complete chess engine came out of this process. Not bad for a conversation with an AI.

## Project Structure

```
src/
├── main.rs           # App entry point & game loop
├── engine/           # Chess logic
│   ├── types.rs      # Core data types (pieces, squares, moves)
│   ├── bitboard.rs   # Efficient board representation
│   ├── board.rs      # Game state management
│   └── movegen.rs    # Move generation & rules
└── graphics/
    └── renderer.rs   # All the visual stuff
```

## License

Do whatever you want with it. It's an experiment.

---

*Built with Claude and Ralph Loops*
