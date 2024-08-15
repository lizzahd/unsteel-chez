use std::fs;
use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::enemy::*;
use crate::level::*;

#[derive(Debug)]
enum PlaceMode {
    Platform,
    Hitbox,
    Remove,
    SpawnPlayer,
    SpawnGoblin,
    Trigger(u8),
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
        }

        if m_cool {
            if is_mouse_button_pressed(MouseButton::Left) {
                match current_place_mode {
                    PlaceMode::Platform => {
                        if let Some(pos) = last_pos {
                            let r = Rect::new(pos.x - level_0.x, pos.y, scaled_m_pos.x - (pos.x - level_0.x), 32.);
                            level_0.collision.platforms.push(r.clone());
                            last_pos = None;
                            map_data.push((PlaceMode::Platform, r));
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Hitbox => {
                        if let Some(pos) = last_pos {
                            let r = Rect::new(pos.x - level_0.x, pos.y, scaled_m_pos.x - (pos.x - level_0.x), scaled_m_pos.y - pos.y);
                            level_0.collision.rect_hitboxes.push(r.clone());
                            last_pos = None;
                            map_data.push((PlaceMode::Hitbox, r));
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Remove => {
                        let mut to_remove: Vec<usize> = Vec::new();
                        for i in 0..level_0.collision.rect_hitboxes.len() {
                            if level_0.collision.rect_hitboxes[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                                map_data.remove(i);
                            }
                        }

                        for i in to_remove {
                            level_0.collision.rect_hitboxes.remove(i);
                        }

                        to_remove = Vec::new();
                        for i in 0..level_0.collision.platforms.len() {
                            if level_0.collision.platforms[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                                map_data.remove(i);
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
                                map_data.remove(i);
                            }
                        }

                        for i in to_remove {
                            entities.remove(i);
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
                        map_data.push((PlaceMode::SpawnGoblin, Rect::new(scaled_m_pos.x, scaled_m_pos.y, 0., 0.)));
                    },
                    _ => {

                    }
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

                for d in &map_data {
                    data.push_str(&format!("{:?} {} {} {} {}\n", d.0, d.1.x, d.1.y, d.1.w, d.1.h));
                }

                fs::write(format!("maps/{}/data", level_0.name), data).expect("Unable to write to file");
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

        m_last_pos = mouse_position();

        next_frame().await;
    }
}