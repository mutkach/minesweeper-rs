#![allow(dead_code, unused_imports)]
use level::Level;
use macroquad::{prelude::*, window};
use renderer::Renderer;
use tile::TileState;
mod level;
mod renderer;
mod tile;
mod utils;

#[derive(PartialEq, Eq)]
enum GameState {
    Playing,
    Lost,
    Won,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut mylevel = Level::new(20, 20, 10);
    let myrenderer = Renderer::new();
    let mut gamestate = GameState::Playing;

    let mut sss = Camera2D {
        zoom: 0.006 * vec2(1., screen_width() / screen_height()),
        offset: Vec2 { x: -1.0, y: 1.0 },
        ..Default::default()
    };
    request_new_screen_size(
        16.0 * 2.0 * mylevel.width as f32,
        16.0 * 2.0 * mylevel.height as f32,
    );
    //next_frame().await;

    set_camera(&sss);

    println!("{:?}", sss.matrix());

    loop {
        clear_background(GRAY);

        myrenderer.draw_level(&mylevel);

        if gamestate == GameState::Lost {
            draw_text(
                "YOU LOSE",
                screen_width() / 8.0,
                screen_height() / 4.0,
                40.0,
                RED,
            );
            if is_key_pressed(KeyCode::R) {
                mylevel = Level::new(20, 20, 10);
                gamestate = GameState::Playing;
            }
            next_frame().await;
            continue;
        }

        if gamestate == GameState::Won {
            draw_text(
                "YOU WON",
                screen_width() / 8.0,
                screen_height() / 4.0,
                40.0,
                GREEN,
            );
            if is_key_pressed(KeyCode::R) {
                mylevel = Level::new(20, 20, 10);
                gamestate = GameState::Playing;
            }
            next_frame().await;
            continue;
        }

        if is_key_pressed(KeyCode::Left) {
            sss.offset.x += 0.05;
            sss.zoom = 0.004 * vec2(1., screen_width() / screen_height());
            set_camera(&sss);
        }
        if is_key_pressed(KeyCode::Right) {
            sss.offset.x -= 0.05;
            sss.zoom = 0.004 * vec2(1., screen_width() / screen_height());
            set_camera(&sss);
        }
        if is_key_pressed(KeyCode::Up) {
            sss.offset.y += 0.05;
            sss.zoom = 0.004 * vec2(1., screen_width() / screen_height());
            set_camera(&sss);
        }
        if is_key_pressed(KeyCode::Down) {
            sss.offset.y -= 0.05;
            sss.zoom = 0.004 * vec2(1., screen_width() / screen_height());
            set_camera(&sss);
        }

        if is_key_pressed(KeyCode::R) {
            mylevel = Level::new(20, 20, 10);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = vec2(mouse_position().0, mouse_position().1);
            let new_pos = sss.screen_to_world(pos);
            let tile_nx = new_pos.x as usize / 16;
            let tile_ny = new_pos.y as usize / 16;
            let idx = tile_nx * mylevel.width + tile_ny;
            if mylevel.check_bomb(idx) {
                gamestate = GameState::Lost;
                next_frame().await;
                continue;
            }
            mylevel.bfs_cells(idx);
            if mylevel.check_win() {
                gamestate = GameState::Won;
            }
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            let pos = vec2(mouse_position().0, mouse_position().1);
            let new_pos = sss.screen_to_world(pos);
            let tile_nx = new_pos.x as usize / 16;
            let tile_ny = new_pos.y as usize / 16;
            let idx = tile_nx * mylevel.width + tile_ny;
            mylevel.mark_cell(idx);
            if mylevel.check_win() {
                gamestate = GameState::Won;
            }
        }

        next_frame().await
    }
}
