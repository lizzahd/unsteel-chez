use macroquad::prelude::*;

use crate::entittie::*;

mod entittie;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Unsteel Chez"),
        window_width: 1280,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    loop {
        clear_background(BLACK);

        for entity in &mut entities {
            entity.update();
        }

        for entity in &mut entities {
            entity.draw();
        }

        next_frame().await;
    }
}
