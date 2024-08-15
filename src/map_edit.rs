use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::enemy::*;
use crate::level::*;

#[derive(Debug)]
pub enum PlaceMode {
    Platform,
    Hitbox,
    Remove,
    SpawnPlayer,
    SpawnGoblin,
    Trigger,
    Trigger_n(u8),
}

fn is_trigger_n(value: u8) -> bool {
    matches!(value, 0..=255)
}

fn is_trigger_n_variant(place_mode: &PlaceMode, value: u8) -> bool {
    match place_mode {
        PlaceMode::Trigger_n(v) => *v == value,
        _ => false,
    }
}

pub async fn level_edit() {
    let assets = AssetManager::new("assets/images").await;

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    let mut level_0 = Level::new("level_0").await;

    let mut m_last_pos = mouse_position();

    let mut last_pos: Option<Vec2> = None;
    let mut m_cool = true;

    let mut current_place_mode = PlaceMode::Platform;

    let mut map_data: Vec<(PlaceMode, Rect)> = vec![
        (PlaceMode::SpawnPlayer, Rect::new(0., 0., 0., 0.)),
    ];

    let mut trigger_i = 0;

    entities.push(Box::new(Player::new(vec2(0., 0.), &assets)));

    loop {
        clear_background(BLACK);

        level_0.draw();

        let scaled_m_pos = Vec2::from_array(mouse_position().into()) - vec2(level_0.x, 0.);

        if is_key_pressed(KeyCode::Key1) {
            current_place_mode = PlaceMode::Platform;
        } else if is_key_pressed(KeyCode::Key2) {
            current_place_mode = PlaceMode::Hitbox;
        } else if is_key_pressed(KeyCode::Key3) {
            current_place_mode = PlaceMode::Remove;
        } else if is_key_pressed(KeyCode::Key4) {
            current_place_mode = PlaceMode::SpawnPlayer;
        } else if is_key_pressed(KeyCode::Key5) {
            current_place_mode = PlaceMode::SpawnGoblin;
        } else if is_key_pressed(KeyCode::Key6) {
            current_place_mode = PlaceMode::Trigger;
        }

        if m_cool {
            if is_mouse_button_pressed(MouseButton::Left) {
                match current_place_mode {
                    PlaceMode::Platform => {
                        if let Some(pos) = last_pos {
                            let r = Rect::new(pos.x - level_0.x, pos.y, scaled_m_pos.x - (pos.x - level_0.x), 32.);
                            level_0.collision.platforms.push(r.clone());
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Hitbox => {
                        if let Some(pos) = last_pos {
                            let r = Rect::new(pos.x - level_0.x, pos.y, scaled_m_pos.x - (pos.x - level_0.x), scaled_m_pos.y - pos.y);
                            level_0.collision.rect_hitboxes.push(r.clone());
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Remove => {
                        let mut to_remove: Vec<usize> = Vec::new();
                        for i in 0..level_0.collision.rect_hitboxes.len() {
                            if level_0.collision.rect_hitboxes[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            level_0.collision.rect_hitboxes.remove(i);
                        }

                        to_remove = Vec::new();
                        for i in 0..level_0.collision.platforms.len() {
                            if level_0.collision.platforms[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            level_0.collision.platforms.remove(i);   
                        }

                        to_remove = Vec::new();
                        for i in 1..entities.len() {
                            let e = &entities[i];
                            if Rect::new(e.get_pos().x, e.get_pos().y, e.get_hitbox().w, e.get_hitbox().h).contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            if i < entities.len() {
                                entities.remove(i);
                            }
                        }

                        to_remove = Vec::new();
                        for i in 1..map_data.len() {
                            let d = &map_data[i];

                            if Rect::new(d.1.x, d.1.y, d.1.w, d.1.h).contains(scaled_m_pos) {
                                match d.0 {
                                    PlaceMode::Trigger_n(_) => {
                                        to_remove.push(i);
                                    },
                                    _ => {}
                                }
                            }
                        }

                        for i in to_remove {
                            map_data.remove(i);
                        }
                    },
                    PlaceMode::SpawnPlayer => {
                        if entities.len() > 0 {
                            entities[0] = Box::new(Player::new(scaled_m_pos, &assets));
                        } else {
                            entities.push(Box::new(Player::new(scaled_m_pos, &assets)));
                        }
                        map_data[0] = (PlaceMode::SpawnPlayer, Rect::new(scaled_m_pos.x, scaled_m_pos.y, 0., 0.));
                    },
                    PlaceMode::SpawnGoblin => {
                        entities.push(Box::new(Enemy::new(scaled_m_pos, &assets)));
                    },
                    PlaceMode::Trigger => {
                        if let Some(pos) = last_pos {
                            let r = Rect::new(pos.x - level_0.x, pos.y, scaled_m_pos.x - (pos.x - level_0.x), scaled_m_pos.y - pos.y);
                            last_pos = None;
                            map_data.push((PlaceMode::Trigger_n(trigger_i), r));
                            trigger_i += 1;
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    _ => {}
                }
                m_cool = false;
            }

            if is_mouse_button_down(MouseButton::Middle) {
                level_0.x += mouse_position().0 - m_last_pos.0;
                m_cool = false;
            }
        } else {
            m_cool = true;
        }

        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::S) {
                let mut data = String::new();

                for entity in &mut entities {
                    if let Some(t) = entity.get_type() {
                        match t {
                            PlaceMode::SpawnPlayer => continue,
                            _ => ()
                        }

                        map_data.push((t, Rect::new(entity.get_pos().x, entity.get_pos().y, 0., 0.)))
                    }
                }

                for hitbox in &mut level_0.collision.rect_hitboxes {
                    map_data.push((PlaceMode::Hitbox, Rect::new(hitbox.x, hitbox.y, hitbox.w, hitbox.h)))
                }

                for platform in &mut level_0.collision.platforms {
                    map_data.push((PlaceMode::Platform, Rect::new(platform.x, platform.y, platform.w, platform.h)))
                }

                for d in &map_data {
                    data.push_str(&format!("{:?} {} {} {} {}\n", d.0, d.1.x, d.1.y, d.1.w, d.1.h));
                }

                fs::write(format!("maps/{}/data", level_0.name), data).expect("Unable to write to file");
            } else if is_key_pressed(KeyCode::O) {
                entities = Vec::new();
                level_0.collision = Collision::new();

                let file = File::open(format!("maps/{}/data", level_0.name)).expect("Could not load file");
                let reader = BufReader::new(file);

                for l in reader.lines() {
                    let line = l.expect("Ass wipe");
                    let collection: Vec<&str> = line.split(" ").collect();
                    let (t, sx, sy, sw, sh) = match collection[..] {
                        [a, b, c, d, e] => (a, b, c, d, e),
                        _ => panic!("AAAAAAAAAAAAAA"),
                    };
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
                            level_0.collision.platforms.push(Rect::new(x, y, w, h));
                        },
                        "Hitbox" => {
                            let x: f32 = sx.parse().expect("Error: Not a float");
                            let y: f32 = sy.parse().expect("Error: Not a float");
                            let w: f32 = sw.parse().expect("Error: Not a float");
                            let h: f32 = sh.parse().expect("Error: Not a float");
                            level_0.collision.rect_hitboxes.push(Rect::new(x, y, w, h));
                        },
                        _ => { // should be Trigger_n<number>

                        }
                    }
                }
            }
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

        for d in &map_data {
            match d.0 {
                PlaceMode::Trigger_n(_) => {
                    draw_rectangle_lines(d.1.x + level_0.x, d.1.y, d.1.w, d.1.h, 2., Color::from_rgba(255, 0, 255, 255));
                },
                _ => {}
            }
        }

        m_last_pos = mouse_position();

        next_frame().await;
    }
}