#[allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros, unused)]
pub mod cards;
use macroquad::prelude::*;
use crate::cards::{Deck, Card};

enum GameState {
    Menu,
    Spread,
}

#[macroquad::main("Tarot Card Game")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut d = Deck::new();
    d.fill_deck();
    d.shuffle();

    let spread = &d.cards[0..3];
    let mut spread_textures = Vec::new();
    let mut is_face_up = vec![false; 3];
    let mut flip_progress = vec![0.0; 3];
    let mut descriptions = vec!["".to_string(); 3]; // Store card descriptions

    for (i, c) in spread.iter().enumerate() {
        let front_texture = load_texture(c.clone().front().as_str()).await.unwrap();
        let back_texture = load_texture(c.clone().back().as_str()).await.unwrap();
        spread_textures.push((front_texture, back_texture));

        // Placeholder descriptions (replace with real ones)
        descriptions[i] = format!("{} - A mystical card i love this very much lorem ipsum delores somthing.", "filler");
    }

    let card_width = spread_textures[0].0.width();
    let card_height = spread_textures[0].0.height();

    loop {
        clear_background(GRAY);

        match game_state {
            GameState::Menu => {
                draw_text("Tarot Card Game", screen_width() / 2.0 - 100.0, screen_height() / 3.0, 40.0, WHITE);
                draw_text("Click to Start", screen_width() / 2.0 - 60.0, screen_height() / 2.0, 30.0, WHITE);

                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Spread;
                }
            }
            GameState::Spread => {
                let spacing = 20.0;
                let total_width = 3.0 * card_width + 2.0 * spacing;
                let start_x = (screen_width() - total_width) / 2.0;
                let text_y = screen_height() / 2.0 + card_height / 2.0 + 20.0;

                for i in 0..3 {
                    let x = start_x + i as f32 * (card_width + spacing);
                    let y = screen_height() / 2.0 - card_height / 2.0;

                    if is_face_up[i] {
                        flip_progress[i] += get_frame_time() * 4.0;
                        if flip_progress[i] > 1.0 {
                            flip_progress[i] = 1.0;
                        }
                    } else {
                        flip_progress[i] -= get_frame_time() * 4.0;
                        if flip_progress[i] < -1.0 {
                            flip_progress[i] = -1.0;
                        }
                    }

                    let scale_x = flip_progress[i];
                    let texture = if scale_x > 0.0 { &spread_textures[i].0 } else { &spread_textures[i].1 };

                    draw_texture_ex(
                        texture,
                        x + (card_width / 2.0 * (1.0 - scale_x.abs())),
                        y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(card_width * scale_x.abs(), card_height)),
                            ..Default::default()
                        },
                    );

                    if is_mouse_button_pressed(MouseButton::Left) {
                        let (mx, my) = mouse_position();
                        if mx >= x && mx <= x + card_width && my >= y && my <= y + card_height {
                            is_face_up[i] = !is_face_up[i];
                        }
                    }

                    // Draw description under flipped cards
                    if is_face_up[i] {
                        draw_text(
                            &descriptions[i],
                            x + 10.0,
                            text_y + (i as f32 * 25.0), // Adjust Y spacing per card
                            20.0,
                            WHITE,
                        );
                    }
                }
            }
        }

        next_frame().await;
    }
}
