use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::enemy::*;
use crate::level::*;
use crate::map_edit::*;

mod entittie;
mod playa;
mod assets;
mod primimptnevs;
mod enemy;
mod level;
mod map_edit;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Unsteel Chez"),
        window_width: 1152,
        window_height: 648,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    level_edit().await;
}

async fn arse() {
    let assets = AssetManager::new("assets/images").await;

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    let mut level_0 = Level::new("level_0").await;

    entities.push(Box::new(Player::new(vec2(64., 400.), &assets)));
    entities.push(Box::new(Enemy::new(vec2(400., 300.), &assets)));

    level_0.collision = Collision {
        rect_hitboxes: vec![
            Rect::new(256., 256., 32., 300.),
        ],
        platforms: vec![
            Rect::new(0., 512., 1028., 1.),
            Rect::new(256., 256., 32., 1.),
            Rect::new(512., 350., 512., 1.),
        ]
    };

    let mut m_last_pos = mouse_position();

    loop {
        clear_background(BLACK);

        level_0.draw();

        if is_mouse_button_down(MouseButton::Middle) {
            level_0.x += mouse_position().0 - m_last_pos.0;
        }

        for entity in &mut entities {
            entity.update(&level_0);
        }

        for entity in &mut entities {
            entity.draw(&level_0);
        }

        for hitbox in &mut level_0.collision.rect_hitboxes {
            draw_rectangle_lines(hitbox.x + level_0.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        }

        for platform in &mut level_0.collision.platforms {
            draw_line(platform.x + level_0.x, platform.y, platform.x + platform.w + level_0.x, platform.y, 2., Color::from_rgba(255, 0, 0, 255));
        }

        m_last_pos = mouse_position();

        next_frame().await;
    }
}