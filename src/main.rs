mod engine;
mod graphics;

use engine::{Color, GameState, Move, PieceType, Square};
use graphics::{PromotionState, Renderer, UiState};
use macroquad::prelude::*;

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

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::initial();
    let mut renderer = Renderer::new();
    let mut ui = UiState::default();

    loop {
        // Update layout based on window size
        renderer.update_layout();

        // Handle input
        handle_input(&mut game, &mut ui, &renderer);

        // Clear background
        clear_background(macroquad::color::Color::from_rgba(30, 30, 30, 255));

        // Draw everything
        renderer.draw_board();
        renderer.draw_coordinates();
        renderer.draw_highlights(&game, &ui);
        renderer.draw_pieces(&game, &ui);
        renderer.draw_status(&game);
        renderer.draw_help();

        // Draw promotion dialog if pending
        if let Some(promo) = &ui.promotion_pending {
            let color = game.side_to_move;
            renderer.draw_promotion_dialog(color);

            // Handle promotion selection
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if let Some(piece_type) = renderer.get_promotion_selection(mx, my) {
                    let captured = if promo.is_capture {
                        game.piece_at(promo.to).map(|p| p.piece_type)
                    } else {
                        None
                    };
                    let mv = Move::promotion(promo.from, promo.to, captured, piece_type);
                    game = game.apply_move(mv);
                    ui.last_move = Some(mv);
                    ui.promotion_pending = None;
                    ui.selected_square = None;
                    ui.legal_moves.clear();
                }
            }
        }

        // Exit on ESC when not in promotion dialog
        if is_key_pressed(KeyCode::Escape) {
            if ui.promotion_pending.is_some() {
                ui.promotion_pending = None;
            } else if ui.selected_square.is_some() {
                ui.selected_square = None;
                ui.legal_moves.clear();
            } else {
                break;
            }
        }

        next_frame().await
    }
}

fn handle_input(game: &mut GameState, ui: &mut UiState, renderer: &Renderer) {
    // Skip if promotion dialog is open
    if ui.promotion_pending.is_some() {
        return;
    }

    // Skip if game is over
    if game.is_checkmate() || game.is_stalemate() {
        // R to reset
        if is_key_pressed(KeyCode::R) {
            *game = GameState::initial();
            *ui = UiState::default();
        }
        return;
    }

    // R to reset
    if is_key_pressed(KeyCode::R) {
        *game = GameState::initial();
        *ui = UiState::default();
        return;
    }

    // Handle mouse click
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if let Some(clicked_square) = renderer.screen_to_square(mx, my) {
            handle_square_click(game, ui, clicked_square);
        }
    }
}

fn handle_square_click(game: &mut GameState, ui: &mut UiState, square: Square) {
    // If we have a selected piece and click on a legal move target
    if let Some(selected) = ui.selected_square {
        if let Some(mv) = ui.legal_moves.iter().find(|m| m.to == square) {
            // Check if it's a promotion
            if mv.piece == PieceType::Pawn {
                let is_promotion = match game.side_to_move {
                    Color::White => square.rank() == 7,
                    Color::Black => square.rank() == 0,
                };

                if is_promotion {
                    ui.promotion_pending = Some(PromotionState {
                        from: selected,
                        to: square,
                        is_capture: mv.captured.is_some(),
                    });
                    return;
                }
            }

            // Apply the move
            *game = game.apply_move(*mv);
            ui.last_move = Some(*mv);
            ui.selected_square = None;
            ui.legal_moves.clear();
            return;
        }
    }

    // Check if clicking on own piece to select it
    if let Some(piece) = game.piece_at(square) {
        if piece.color == game.side_to_move {
            ui.selected_square = Some(square);
            ui.legal_moves = game
                .generate_legal_moves()
                .into_iter()
                .filter(|m| m.from == square)
                .collect();
            return;
        }
    }

    // Deselect
    ui.selected_square = None;
    ui.legal_moves.clear();
}
