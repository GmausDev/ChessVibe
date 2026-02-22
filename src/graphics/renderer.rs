use crate::engine::{Color as ChessColor, GameState, Move, PieceType, Square};
use macroquad::prelude::*;

/// Board theme colors.
pub struct BoardTheme {
    pub light_square: Color,
    pub dark_square: Color,
    pub selected_square: Color,
    pub legal_move_indicator: Color,
    pub last_move_highlight: Color,
    pub check_highlight: Color,
}

impl Default for BoardTheme {
    fn default() -> Self {
        Self {
            light_square: Color::from_rgba(240, 217, 181, 255),
            dark_square: Color::from_rgba(181, 136, 99, 255),
            selected_square: Color::from_rgba(130, 151, 105, 200),
            legal_move_indicator: Color::from_rgba(20, 85, 30, 150),
            last_move_highlight: Color::from_rgba(205, 210, 106, 150),
            check_highlight: Color::from_rgba(220, 50, 50, 150),
        }
    }
}

/// UI state for the game.
#[derive(Default)]
pub struct UiState {
    pub selected_square: Option<Square>,
    pub legal_moves: Vec<Move>,
    pub last_move: Option<Move>,
    pub promotion_pending: Option<PromotionState>,
}

pub struct PromotionState {
    pub from: Square,
    pub to: Square,
    pub is_capture: bool,
}

/// Renderer for the chess game.
pub struct Renderer {
    pub theme: BoardTheme,
    pub board_x: f32,
    pub board_y: f32,
    pub square_size: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            theme: BoardTheme::default(),
            board_x: 0.0,
            board_y: 0.0,
            square_size: 80.0,
        }
    }

    /// Updates board position and size based on window dimensions.
    pub fn update_layout(&mut self) {
        let window_width = screen_width();
        let window_height = screen_height();

        // Leave space for UI on the right side and status bar at top
        let board_area_width = window_width - 200.0; // Reserve 200px for help text
        let board_area_height = window_height - 80.0; // Reserve space for status

        // Calculate square size to fit the board area
        let max_square_size = (board_area_width.min(board_area_height) / 8.0).floor();
        self.square_size = max_square_size.clamp(50.0, 90.0);

        let board_size = self.square_size * 8.0;
        // Center the board in the available area, with margin from top
        self.board_x = (board_area_width - board_size) / 2.0 + 20.0;
        self.board_y = 50.0 + (board_area_height - board_size) / 2.0;
    }

    /// Converts a screen position to a board square.
    pub fn screen_to_square(&self, x: f32, y: f32) -> Option<Square> {
        // Calculate position relative to board
        let rel_x = x - self.board_x;
        let rel_y = y - self.board_y;

        // Check if within board bounds
        let board_size = self.square_size * 8.0;
        if rel_x < 0.0 || rel_x >= board_size || rel_y < 0.0 || rel_y >= board_size {
            return None;
        }

        // Calculate file (0-7 from left to right)
        let file = (rel_x / self.square_size) as u8;
        // Calculate rank (0-7 from bottom to top, but screen y increases downward)
        let row_from_top = (rel_y / self.square_size) as u8;
        let rank = 7 - row_from_top;

        Some(Square::from_coords(file, rank))
    }

    /// Converts a board square to screen coordinates (top-left of square).
    pub fn square_to_screen(&self, square: Square) -> (f32, f32) {
        let x = self.board_x + square.file() as f32 * self.square_size;
        let y = self.board_y + (7 - square.rank()) as f32 * self.square_size;
        (x, y)
    }

    /// Draws the chess board.
    pub fn draw_board(&self) {
        for rank in 0..8 {
            for file in 0..8 {
                let is_light = (rank + file) % 2 == 1;
                let color = if is_light {
                    self.theme.light_square
                } else {
                    self.theme.dark_square
                };

                let x = self.board_x + file as f32 * self.square_size;
                let y = self.board_y + (7 - rank) as f32 * self.square_size;

                draw_rectangle(x, y, self.square_size, self.square_size, color);
            }
        }
    }

    /// Draws board coordinates (a-h, 1-8).
    pub fn draw_coordinates(&self) {
        let font_size = (self.square_size * 0.2).max(12.0);

        // File letters (a-h)
        for file in 0..8 {
            let letter = (b'a' + file) as char;
            let x = self.board_x + file as f32 * self.square_size + self.square_size - font_size;
            let y = self.board_y + 8.0 * self.square_size - 2.0;

            let is_light = file % 2 == 1;
            let color = if is_light {
                self.theme.dark_square
            } else {
                self.theme.light_square
            };

            draw_text(&letter.to_string(), x, y, font_size, color);
        }

        // Rank numbers (1-8)
        for rank in 0..8 {
            let number = (rank + 1).to_string();
            let x = self.board_x + 2.0;
            let y = self.board_y + (7 - rank) as f32 * self.square_size + font_size;

            let is_light = rank % 2 == 1;
            let color = if is_light {
                self.theme.dark_square
            } else {
                self.theme.light_square
            };

            draw_text(&number, x, y, font_size, color);
        }
    }

    /// Draws highlights for last move, selection, and legal moves.
    pub fn draw_highlights(&self, game: &GameState, ui: &UiState) {
        // Last move highlight
        if let Some(last_move) = &ui.last_move {
            self.highlight_square(last_move.from, self.theme.last_move_highlight);
            self.highlight_square(last_move.to, self.theme.last_move_highlight);
        }

        // Check highlight
        if game.is_in_check(game.side_to_move) {
            let king_sq = game.king_square(game.side_to_move);
            self.highlight_square(king_sq, self.theme.check_highlight);
        }

        // Selected square highlight
        if let Some(selected) = ui.selected_square {
            self.highlight_square(selected, self.theme.selected_square);
        }

        // Legal move indicators
        for mv in &ui.legal_moves {
            self.draw_legal_move_indicator(mv.to, game.piece_at(mv.to).is_some());
        }
    }

    fn highlight_square(&self, square: Square, color: macroquad::color::Color) {
        let (x, y) = self.square_to_screen(square);
        draw_rectangle(x, y, self.square_size, self.square_size, color);
    }

    fn draw_legal_move_indicator(&self, square: Square, is_capture: bool) {
        let (x, y) = self.square_to_screen(square);
        let center_x = x + self.square_size / 2.0;
        let center_y = y + self.square_size / 2.0;

        if is_capture {
            // Draw ring for captures
            let outer_radius = self.square_size * 0.45;
            let inner_radius = self.square_size * 0.35;
            draw_circle(center_x, center_y, outer_radius, self.theme.legal_move_indicator);
            // Draw inner circle with square color to create ring effect
            let file = square.file();
            let rank = square.rank();
            let is_light = (file + rank) % 2 == 1;
            let bg_color = if is_light {
                self.theme.light_square
            } else {
                self.theme.dark_square
            };
            draw_circle(center_x, center_y, inner_radius, bg_color);
        } else {
            // Draw dot for non-captures
            let radius = self.square_size * 0.15;
            draw_circle(center_x, center_y, radius, self.theme.legal_move_indicator);
        }
    }

    /// Draws all pieces on the board.
    pub fn draw_pieces(&self, game: &GameState, ui: &UiState) {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::from_coords(file, rank);

                // Skip the piece being dragged (if any)
                if ui.selected_square == Some(square) && is_mouse_button_down(MouseButton::Left) {
                    continue;
                }

                if let Some(piece) = game.piece_at(square) {
                    let (x, y) = self.square_to_screen(square);
                    self.draw_piece(piece.color, piece.piece_type, x, y);
                }
            }
        }

        // Draw dragged piece at mouse position
        if let Some(selected) = ui.selected_square {
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(piece) = game.piece_at(selected) {
                    let (mx, my) = mouse_position();
                    let x = mx - self.square_size / 2.0;
                    let y = my - self.square_size / 2.0;
                    self.draw_piece(piece.color, piece.piece_type, x, y);
                }
            }
        }
    }

    /// Draws a chess piece using circles with letter labels.
    fn draw_piece(&self, color: ChessColor, piece_type: PieceType, x: f32, y: f32) {
        let letter = match piece_type {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Rook => "R",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
            PieceType::Pawn => "P",
        };

        let center_x = x + self.square_size / 2.0;
        let center_y = y + self.square_size / 2.0;
        let radius = self.square_size * 0.4;

        // Piece colors
        let (fill_color, outline_color, text_color) = match color {
            ChessColor::White => (
                Color::from_rgba(255, 255, 240, 255),  // Ivory fill
                Color::from_rgba(50, 50, 50, 255),     // Dark outline
                Color::from_rgba(30, 30, 30, 255),     // Dark text
            ),
            ChessColor::Black => (
                Color::from_rgba(40, 40, 40, 255),     // Dark fill
                Color::from_rgba(200, 200, 200, 255),  // Light outline
                Color::from_rgba(255, 255, 255, 255),  // White text
            ),
        };

        // Draw piece circle with outline
        draw_circle(center_x, center_y, radius + 2.0, outline_color);
        draw_circle(center_x, center_y, radius, fill_color);

        // Draw letter
        let font_size = self.square_size * 0.5;
        let text_width = font_size * 0.6; // Approximate width
        let text_x = center_x - text_width / 2.0;
        let text_y = center_y + font_size * 0.35;

        draw_text(letter, text_x, text_y, font_size, text_color);
    }

    /// Draws the promotion dialog.
    pub fn draw_promotion_dialog(&self, color: ChessColor) {
        let pieces = [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
        ];

        let dialog_width = self.square_size * 4.0;
        let dialog_height = self.square_size;
        let dialog_x = (screen_width() - dialog_width) / 2.0;
        let dialog_y = (screen_height() - dialog_height) / 2.0;

        // Background
        draw_rectangle(
            dialog_x - 10.0,
            dialog_y - 10.0,
            dialog_width + 20.0,
            dialog_height + 20.0,
            Color::from_rgba(50, 50, 50, 240),
        );

        // Pieces
        for (i, piece_type) in pieces.iter().enumerate() {
            let x = dialog_x + i as f32 * self.square_size;
            let y = dialog_y;

            // Highlight on hover
            let (mx, my) = mouse_position();
            if mx >= x && mx < x + self.square_size && my >= y && my < y + self.square_size {
                draw_rectangle(x, y, self.square_size, self.square_size, GRAY);
            }

            self.draw_piece(color, *piece_type, x, y);
        }
    }

    /// Gets the promotion piece selection from click position.
    pub fn get_promotion_selection(&self, x: f32, y: f32) -> Option<PieceType> {
        let pieces = [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
        ];

        let dialog_width = self.square_size * 4.0;
        let dialog_height = self.square_size;
        let dialog_x = (screen_width() - dialog_width) / 2.0;
        let dialog_y = (screen_height() - dialog_height) / 2.0;

        if y >= dialog_y && y < dialog_y + dialog_height {
            let index = ((x - dialog_x) / self.square_size) as usize;
            if index < 4 {
                return Some(pieces[index]);
            }
        }

        None
    }

    /// Draws the game status bar.
    pub fn draw_status(&self, game: &GameState) {
        let status = if game.is_checkmate() {
            let winner = game.side_to_move.opposite();
            format!("Checkmate! {:?} wins!", winner)
        } else if game.is_stalemate() {
            "Stalemate! Draw.".to_string()
        } else if game.is_in_check(game.side_to_move) {
            format!("{:?} to move - CHECK!", game.side_to_move)
        } else {
            format!("{:?} to move", game.side_to_move)
        };

        draw_text(&status, 20.0, 25.0, 24.0, WHITE);

        // Draw move count
        let move_info = format!("Move: {}", game.fullmove_number);
        draw_text(&move_info, screen_width() - 120.0, 25.0, 20.0, LIGHTGRAY);
    }

    /// Draws help text.
    pub fn draw_help(&self) {
        let help_x = self.board_x + self.square_size * 8.0 + 20.0;
        let help_y = self.board_y;

        draw_text("Controls:", help_x, help_y, 20.0, WHITE);
        draw_text("Click to select/move", help_x, help_y + 25.0, 16.0, LIGHTGRAY);
        draw_text("ESC - Deselect", help_x, help_y + 45.0, 16.0, LIGHTGRAY);
        draw_text("R - Reset game", help_x, help_y + 65.0, 16.0, LIGHTGRAY);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
