use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use macroquad::prelude::*;

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::enemy::*;
use crate::level::*;
use crate::touchytouchy::*;
use crate::event::*;

#[derive(Debug)]
pub enum PlaceMode {
    Platform,
    Hitbox,
    Remove,
    SpawnPlayer,
    SpawnGoblin,
    Dawn,
    Trigger,
    Projectile,
    TriggerN(u8),
}

pub async fn level_edit() {
    let assets = AssetManager::new("assets/images").await;

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    let mut level = Level::new("level_0").await;

    let mut m_last_pos = mouse_position();

    let mut last_pos: Option<Vec2> = None;
    let mut m_cool = true;

    let mut current_place_mode = PlaceMode::Platform;

    let mut player_start = vec2(0., 0.);
    let mut map_data: Vec<(PlaceMode, Rect)> = vec![
        (PlaceMode::SpawnPlayer, Rect::new(player_start.x, player_start.y, 0., 0.)),
    ];

    let mut trigger_i = 0;

    entities.push(Box::new(Player::new(vec2(0., 0.), &assets)));

    loop {
        clear_background(BLACK);

        level.draw();

        let scaled_m_pos = Vec2::from_array(mouse_position().into()) - vec2(level.x, 0.);

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
            current_place_mode = PlaceMode::Dawn;
        } else if is_key_pressed(KeyCode::Key7) {
            current_place_mode = PlaceMode::Trigger;
        }

        if m_cool {
            if is_mouse_button_pressed(MouseButton::Left) {
                match current_place_mode {
                    PlaceMode::Platform => {
                        if let Some(pos) = last_pos {
                            let mut x = pos.x - level.x;
                            let mut w = scaled_m_pos.x - (pos.x - level.x);
                            
                            if w < 0. {
                                w = w.abs();
                                x -= w;
                            }

                            let r = Rect::new(x, pos.y, w, 32.);
                            level.collision.platforms.push(r.clone());
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Hitbox => {
                        if let Some(pos) = last_pos {
                            let mut x = pos.x - level.x;
                            let mut y = pos.y;
                            let mut w = scaled_m_pos.x - (pos.x - level.x);
                            let mut h = scaled_m_pos.y - pos.y;
                            
                            if w < 0. {
                                w = w.abs();
                                x -= w;
                            }
                            if h < 0. {
                                h = h.abs();
                                y -= h;
                            }

                            let r = Rect::new(x, y, w, h);
                            level.collision.rect_hitboxes.push(r.clone());
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    PlaceMode::Remove => {
                        let mut to_remove: Vec<usize> = Vec::new();
                        for i in 0..level.collision.rect_hitboxes.len() {
                            if level.collision.rect_hitboxes[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            level.collision.rect_hitboxes.remove(i);
                        }

                        to_remove = Vec::new();
                        for i in 0..level.collision.platforms.len() {
                            if level.collision.platforms[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            level.collision.platforms.remove(i);   
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
                                    PlaceMode::TriggerN(_) => {
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
                        player_start.x = scaled_m_pos.x;
                        player_start.y = scaled_m_pos.y;
                        map_data[0] = (PlaceMode::SpawnPlayer, Rect::new(scaled_m_pos.x, scaled_m_pos.y, 0., 0.));
                    },
                    PlaceMode::SpawnGoblin => {
                        entities.push(Box::new(Enemy::new(scaled_m_pos, &assets)));
                    },
                    PlaceMode::Dawn => {
                        entities.push(Box::new(Dawn::new(scaled_m_pos, &assets)));
                    },
                    PlaceMode::Trigger => {
                        if let Some(pos) = last_pos {
                            let mut x = pos.x - level.x;
                            let mut y = pos.y;
                            let mut w = scaled_m_pos.x - (pos.x - level.x);
                            let mut h = scaled_m_pos.y - pos.y;
                            
                            if w < 0. {
                                w = w.abs();
                                x -= w;
                            }
                            if h < 0. {
                                h = h.abs();
                                y -= h;
                            }

                            let r = Rect::new(x, y, w, h);
                            last_pos = None;
                            map_data.push((PlaceMode::TriggerN(trigger_i), r));
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
                level.x += mouse_position().0 - m_last_pos.0;
                m_cool = false;
            }
        } else {
            m_cool = true;
        }

        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::S) { // save map
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

                for hitbox in &mut level.collision.rect_hitboxes {
                    map_data.push((PlaceMode::Hitbox, Rect::new(hitbox.x, hitbox.y, hitbox.w, hitbox.h)))
                }

                for platform in &mut level.collision.platforms {
                    map_data.push((PlaceMode::Platform, Rect::new(platform.x, platform.y, platform.w, platform.h)))
                }

                for d in &map_data {
                    data.push_str(&format!("{:?} {} {} {} {}\n", d.0, d.1.x, d.1.y, d.1.w, d.1.h));
                }

                fs::write(format!("maps/{}/data", level.name), data).expect("Unable to write to file");
            } else if is_key_pressed(KeyCode::O) { // load map
                map_data = vec![
                    (PlaceMode::SpawnPlayer, Rect::new(player_start.x, player_start.y, 0., 0.)),
                ];
                entities = Vec::new();
                level.collision = Collision::new();

                let file = File::open(format!("maps/{}/data", level.name)).expect("Could not load file");
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
                            level.collision.platforms.push(Rect::new(x, y, w, h));
                        },
                        "Hitbox" => {
                            let x: f32 = sx.parse().expect("Error: Not a float");
                            let y: f32 = sy.parse().expect("Error: Not a float");
                            let w: f32 = sw.parse().expect("Error: Not a float");
                            let h: f32 = sh.parse().expect("Error: Not a float");
                            level.collision.rect_hitboxes.push(Rect::new(x, y, w, h));
                        },
                        "Dawn" => {
                            let x: f32 = sx.parse().expect("Error: Not a float");
                            let y: f32 = sy.parse().expect("Error: Not a float");
                            entities.push(Box::new(Dawn::new(vec2(x, y), &assets)));
                        },
                        _ => { // should be TriggerN<number>

                        }
                    }
                }
            } else if is_key_pressed(KeyCode::B) { // test map
                let mut test_level = level.clone();
                let mut test_entities: Vec<Box<dyn Entity>> = Vec::new();

                let mut test_events: Vec<EventType> = Vec::new();
                
                for entity in &entities {
                    test_entities.push(entity.box_clone());
                }

                'test_loop: loop {
                    clear_background(BLACK);

                    test_level.draw();

                    if is_mouse_button_down(MouseButton::Middle) {
                        test_level.x += mouse_position().0 - m_last_pos.0;
                    }

                    let mut to_spawn: Vec<Box<dyn Entity>> = Vec::new();
                    let mut covered_events: Vec<usize> = Vec::new();
                    for entity in test_entities.iter_mut() {
                        let result = entity.update(&test_level);
                        if let Some(e) = result {
                            test_events.push(e);
                        }

                        for (i, event) in (&test_events).iter().enumerate() {
                            // Handle spawn events
                            match event {
                                EventType::SpawnFart{pos, d} => {
                                    if !covered_events.contains(&i) {
                                        to_spawn.push(Box::new(Fart::new(*pos, *d, &assets)));
                                        covered_events.push(i);
                                    }
                                },
                                _ => {}
                            }

                            entity.give_event(event);
                        }
                    }

                    for entity in to_spawn {
                        test_entities.push(entity);
                    }

                    test_events = Vec::new();
                    let mut to_kill: Vec<usize> = Vec::new();

                    for (i, entity) in test_entities.iter().enumerate() {
                        if entity.get_dead() {
                            to_kill.push(i);
                            continue;
                        }

                        let result = entity.give_data(&test_level, &test_entities);
                        if let Some(t) = result {
                            test_events.push(t)
                        }
                        entity.draw(&test_level);
                    }

                    for i in to_kill {
                        if i < test_entities.len() {
                            test_entities.remove(i);
                        }
                    }

                    for hitbox in &mut test_level.collision.rect_hitboxes {
                        draw_rectangle_lines(hitbox.x + test_level.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
                    }

                    for platform in &mut test_level.collision.platforms {
                        draw_line(platform.x + test_level.x, platform.y, platform.x + platform.w + test_level.x, platform.y, 2., Color::from_rgba(255, 0, 0, 255));
                    }

                    if is_key_pressed(KeyCode::Escape) {
                        break 'test_loop;
                    }

                    m_last_pos = mouse_position();

                    next_frame().await;
                }
            }
        }

        for entity in &mut entities {
            entity.draw(&level);
        }

        for hitbox in &mut level.collision.rect_hitboxes {
            draw_rectangle_lines(hitbox.x + level.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        }

        for platform in &mut level.collision.platforms {
            draw_line(platform.x + level.x, platform.y, platform.x + platform.w + level.x, platform.y, 2., Color::from_rgba(255, 0, 0, 255));
        }

        for d in &map_data {
            match d.0 {
                PlaceMode::TriggerN(_) => {
                    draw_rectangle_lines(d.1.x + level.x, d.1.y, d.1.w, d.1.h, 2., Color::from_rgba(255, 0, 255, 255));
                },
                _ => {}
            }
        }

        m_last_pos = mouse_position();

        next_frame().await;
    }
}