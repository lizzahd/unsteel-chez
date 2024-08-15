use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::utils::*;

mod entittie;
mod playa;
mod utils;

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
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();
    entities.push(Box::new(Player::new(vec2(64., 64.))));

    let mut hitboxes: Vec<Rect> = Vec::new();
    hitboxes.push(Rect::new(0., 0., 32., 64.));

    loop {
        clear_background(BLACK);

        for entity in &mut entities {
            entity.update();
        }

        for entity in &mut entities {
            entity.draw();
        }

        for hitbox in &mut hitboxes {
            draw_rectangle_lines(hitbox.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        }

        next_frame().await;
    }
}
