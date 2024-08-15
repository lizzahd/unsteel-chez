use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::enemy::*;

mod entittie;
mod playa;
mod assets;
mod primimptnevs;
mod enemy;

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
    let assets = AssetManager::new().await;

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    entities.push(Box::new(Player::new(vec2(64., 400.), &assets)));
    entities.push(Box::new(Enemy::new(vec2(400., 300.), &assets)));

    let mut collision = Collision {
        rect_hitboxes: vec![
            Rect::new(256., 256., 32., 300.),
        ],
        platforms: vec![
            Rect::new(0., 512., 1028., 1.),
            Rect::new(256., 256., 32., 1.),
            Rect::new(512., 350., 512., 1.),
        ]
    };

    loop {
        clear_background(BLACK);

        for entity in &mut entities {
            entity.update(&collision);
        }

        for entity in &mut entities {
            entity.draw();
        }

        for hitbox in &mut collision.rect_hitboxes {
            draw_rectangle_lines(hitbox.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        }

        for platform in &mut collision.platforms {
            draw_line(platform.x, platform.y, platform.x + platform.w, platform.y, 1., Color::from_rgba(0, 255, 0, 255));
        }

        next_frame().await;
    }
}
