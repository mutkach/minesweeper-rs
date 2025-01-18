use core::fmt;

use macroquad::color::{
    hsl_to_rgb, Color, BLUE, GOLD, GREEN, LIME, MAGENTA, PURPLE, RED, WHITE, YELLOW,
};
use macroquad::text::{
    camera_font_scale, draw_text, draw_text_ex, load_ttf_font, load_ttf_font_from_bytes, Font,
    TextParams,
};
use macroquad::texture::{draw_texture, Image};
use macroquad::texture::{load_texture, Texture2D};

use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::level::Level;
use crate::tile::TileState;

const PALETTE: [Color; 8] = [BLUE, GREEN, YELLOW, RED, PURPLE, MAGENTA, LIME, GOLD];

pub struct Renderer {
    bomb_texture: Texture2D,
    tile_texture: Texture2D,
    floor_texture: Texture2D,
    marked_texture: Texture2D,
    palette: Vec<Color>,
    font: Font,
}

impl Renderer {
    pub fn new() -> Self {
        let mut palette = Vec::<Color>::new();
        for i in 0..8 {
            palette.push(hsl_to_rgb(0.01, 0.5, 0.4 + (i as f32) / 8.0));
        }
        Renderer {
            bomb_texture: Texture2D::from_file_with_format(include_bytes!("../res/bomb.png"), None),

            tile_texture: Texture2D::from_file_with_format(include_bytes!("../res/tile.png"), None),

            floor_texture: Texture2D::from_file_with_format(
                include_bytes!("../res/floor.png"),
                None,
            ),

            marked_texture: Texture2D::from_file_with_format(
                include_bytes!("../res/marked.png"),
                None,
            ),
            font: load_ttf_font_from_bytes(include_bytes!("../res/arcada_font.ttf")).expect("must"),
            palette,
        }
    }

    pub fn draw_level(&self, level: &Level) {
        for tile in &level.grid {
            match tile.state {
                TileState::Hidden => {
                    draw_texture(
                        &self.tile_texture,
                        tile.x as f32 * 16.0,
                        tile.y as f32 * 16.0,
                        WHITE,
                    );
                }
                TileState::Free => {
                    draw_texture(
                        &self.floor_texture,
                        tile.x as f32 * 16.0,
                        tile.y as f32 * 16.0,
                        WHITE,
                    );
                    if tile.number > 0 {
                        let (font_size, font_scale, font_scale_aspect) = camera_font_scale(16.0);
                        draw_text_ex(
                            &format!("{}", tile.number),
                            tile.x as f32 * 16.0 + 6.0,
                            tile.y as f32 * 16.0 + 12.0,
                            TextParams {
                                font_size,
                                font: Some(&self.font),
                                font_scale,
                                font_scale_aspect,
                                rotation: 0.0,
                                color: self.palette[tile.number - 1],
                            },
                        );
                    }
                }
                TileState::Marked => draw_texture(
                    &self.marked_texture,
                    tile.x as f32 * 16.0,
                    tile.y as f32 * 16.0,
                    WHITE,
                ),
            }
        }
    }
}
