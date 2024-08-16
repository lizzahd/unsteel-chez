use macroquad::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
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
mod touchytouchy;
mod event;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Unsteel Chez"),
        window_width: 1152,
        window_height: 648,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // draw loading screen
    draw_texture(&load_texture("assets/loading_screen.png").await.unwrap(), 0., 0., WHITE);
    next_frame().await;

    level_edit().await;
}

async fn arse() -> io::Result<()> {
    // load all assets
    let assets = AssetManager::new("assets/images").await;

    // where all the entities in the game live
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    // the current level. can be changed by changing the value of the `name` field
    let mut level = Level::new("level_0").await;

    // open the `data` file to load the map
    let file = File::open(format!("maps/{}/data", level.name)).expect("Could not load file");
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let line = l.expect("Could not read line");
        // split the line and convert it to a usable tuple
        let collection: Vec<&str> = line.split(" ").collect();
        let (t, sx, sy, sw, sh) = match collection[..] {
            [a, b, c, d, e] => (a, b, c, d, e),
            _ => panic!("AAAAAAAAAAAAAA"),
        };

        // parse the `data` file and spawn in necessary stuff
        match t {
            "SpawnPlayer" => {
                let x: f32 = sx.parse().expect("Error: Not a float");
                let y: f32 = sy.parse().expect("Error: Not a float");
                entities.push(Box::new(Player::new(vec2(x, y), &assets))); // player always gotta be first, bruv
            },
            "SpawnGoblin" => {
                let x: f32 = sx.parse().expect("Error: Not a float");
                let y: f32 = sy.parse().expect("Error: Not a float");
                entities.push(Box::new(Enemy::new(vec2(x, y), &assets)));
            },
            "Platform" => {
                let x: f32 = sx.parse().expect("Error: Not a float");
                let y: f32 = sy.parse().expect("Error: Not a float");
                let w: f32 = sw.parse().expect("Error: Not a float");
                let h: f32 = sh.parse().expect("Error: Not a float");
                level.collision.platforms.push(Rect::new(x, y, w, h));
            },
            "Hitbox" => {
                let x: f32 = sx.parse().expect("Error: Not a float");
                let y: f32 = sy.parse().expect("Error: Not a float");
                let w: f32 = sw.parse().expect("Error: Not a float");
                let h: f32 = sh.parse().expect("Error: Not a float");
                level.collision.rect_hitboxes.push(Rect::new(x, y, w, h));
            },
            _ => {
            }
        }
    }

    // mouse last position. is updated at the end of every frame
    let mut m_last_pos = mouse_position();

    loop {
        clear_background(BLACK);

        // display the foreground and background of the current level
        level.draw();

        // drag the level
        if is_mouse_button_down(MouseButton::Middle) {
            level.x += mouse_position().0 - m_last_pos.0;
        }

        for entity in entities.iter_mut() {
            entity.update(&level);
        }

        // this allows the entities to communicate with each other
        for entity in entities.iter() {
            entity.give_data(&level, &entities);
            entity.draw(&level);
        }

        for hitbox in &mut level.collision.rect_hitboxes {
            draw_rectangle_lines(hitbox.x + level.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        }

        for platform in &mut level.collision.platforms {
            draw_line(platform.x + level.x, platform.y, platform.x + platform.w + level.x, platform.y, 2., Color::from_rgba(255, 0, 0, 255));
        }

        m_last_pos = mouse_position();

        next_frame().await;
    }

    Ok(())
}