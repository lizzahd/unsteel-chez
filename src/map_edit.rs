use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{prelude::*, BufReader};
use std::fs::OpenOptions;
use macroquad::audio::{play_sound, stop_sound, PlaySoundParams};
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
    SpawnGobloronBoss,
    Dawn,
    Trigger,
    Laser,
    KillTrigger,
    Projectile,
}

pub async fn level_edit() {
    let assets = AssetManager::new("assets").await;

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    let mut level = Level::new("level_1").await;

    let mut current_place_mode = PlaceMode::Platform;

    let mut player_start = vec2(0., 0.);

    // mostly deals with saving and loading
    let mut map_data: Vec<(PlaceMode, Rect)> = vec![
        (PlaceMode::SpawnPlayer, Rect::new(player_start.x, player_start.y, 0., 0.)),
    ];

    let death_sound = assets.sounds.get("death").expect("could not load death sound");
    let win_music = assets.sounds.get("win_music").expect("could not load win music");

    // spawn the player. must always be the first index of the vec
    entities.push(Box::new(Player::new(vec2(0., 0.), &assets)));

    // previous mouse position
    let mut m_last_pos = mouse_position();
    // last clicked position
    let mut last_pos: Option<Vec2> = None;
    // input cooldown to prevent rapid spamming
    let mut m_cool = true;

    let mut level_scroll_boost = 1.;
    const LEVEL_SCROLL_SPEED: f32 = 20.;

    loop {
        clear_background(BLACK);

        level.draw();

        // get the mouse position scaled to the "camera"
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
            current_place_mode = PlaceMode::KillTrigger;
        } else if is_key_pressed(KeyCode::Space) {
            // center on the player
            level.x = (screen_width() / 2.) - entities[0].get_pos().x;
        }

        if m_cool {
            if is_mouse_button_pressed(MouseButton::Left) {
                match current_place_mode {
                    PlaceMode::Platform => {
                        if let Some(pos) = last_pos {
                            // allow it to be placed in any order
                            let mut x = pos.x;
                            let mut w = scaled_m_pos.x - pos.x;
                            
                            if w < 0. {
                                w = w.abs();
                                x -= w;
                            }

                            let r = Rect::new(x, pos.y, w, 32.);
                            level.collision.platforms.push(r.clone());
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(scaled_m_pos.into()));
                        }
                    },
                    PlaceMode::Hitbox => {
                        if let Some(pos) = last_pos {
                            // allow it to be placed in any order
                            let mut x = pos.x;
                            let mut y = pos.y;
                            let mut w = scaled_m_pos.x - pos.x;
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
                            // put a walkable platform on top of it
                            level.collision.platforms.push(Rect::new(x, y - 1., w, 32.));
                            last_pos = None;
                        } else {
                            last_pos = Some(Vec2::from_array(scaled_m_pos.into()));
                        }
                    },
                    PlaceMode::Remove => {
                        // anything that is added to this vec gets removed once the loops are finished
                        let mut to_remove: Vec<usize> = Vec::new();
                        for i in 0..level.collision.rect_hitboxes.len() {
                            if level.collision.rect_hitboxes[i].contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            if i < level.collision.rect_hitboxes.len() {
                                level.collision.rect_hitboxes.remove(i);
                            }
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
                        for (i, trigger) in level.triggers.iter().enumerate() {
                            if trigger.rect.contains(scaled_m_pos) {
                                to_remove.push(i);
                            }
                        }

                        for i in to_remove {
                            if i < level.triggers.len() {
                                level.triggers.remove(i);
                            }
                        }
                    },
                    PlaceMode::SpawnPlayer => {
                        // ensure the player is first in the vec
                        if entities.len() > 0 {
                            entities[0] = Box::new(Player::new(scaled_m_pos, &assets));
                        } else {
                            entities.push(Box::new(Player::new(scaled_m_pos, &assets)));
                        }

                        player_start.x = scaled_m_pos.x;
                        player_start.y = scaled_m_pos.y;
                        // critical to update map_data's instance of the player
                        map_data[0] = (PlaceMode::SpawnPlayer, Rect::new(scaled_m_pos.x, scaled_m_pos.y, 0., 0.));
                    },
                    PlaceMode::SpawnGoblin => {
                        entities.push(Box::new(Enemy::new(scaled_m_pos, &assets)));
                    },
                    PlaceMode::Dawn => {
                        entities.push(Box::new(Dawn::new(scaled_m_pos, &assets)));
                    },
                    PlaceMode::KillTrigger => {
                        // exactly the same as a hitbox, but with no collision
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
                            level.triggers.push(Trigger {rect: r, t: TriggerType::Kill});
                            // map_data.push((PlaceMode::Trigger, r));
                        } else {
                            last_pos = Some(Vec2::from_array(mouse_position().into()));
                        }
                    },
                    _ => {}
                }
                m_cool = false;
            }

            // pan the camera
            if is_mouse_button_down(MouseButton::Middle) {
                level.x += mouse_position().0 - m_last_pos.0;
                m_cool = false;
            }
        } else {
            m_cool = true;
        }

        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::S) { // save map
                // clear data. player does not need to be in it right off the bat, since he will be added first in the loop
                let mut data = String::new();

                // add everything to data
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

                for trigger in level.triggers.iter_mut() {
                    map_data.push((PlaceMode::KillTrigger, trigger.rect.clone()))
                }

                for d in &map_data {
                    data.push_str(&format!("{:?} {} {} {} {}\n", d.0, d.1.x, d.1.y, d.1.w, d.1.h));
                }

                let f_name = format!("maps/{}/data", level.name);
                // File::create(&f_name).unwrap();

                fs::remove_file(&f_name).expect("could not remove file");

                fs::write(f_name, data).expect("Unable to write to file");
            } else if is_key_pressed(KeyCode::O) { // load map
                // reinitialize map_data with the player in the first index
                map_data = vec![
                    (PlaceMode::SpawnPlayer, Rect::new(player_start.x, player_start.y, 0., 0.)),
                ];
                entities = Vec::new();
                level.collision = Collision::new();
                level.triggers = Vec::new();

                // load the data file
                let file = File::open(format!("maps/{}/data", level.name)).expect("Could not load file");
                let reader = BufReader::new(file);

                for l in reader.lines() {
                    // split the line and convert it to a usable tuple
                    let line = l.expect("Ass wipe");
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
                        "SpawnGobloronBoss" => {
                            entities.push(Box::new(GobloronBoss::new(&assets)));
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
                        "KillTrigger" => {
                            let x: f32 = sx.parse().expect("Error: Not a float");
                            let y: f32 = sy.parse().expect("Error: Not a float");
                            let w: f32 = sw.parse().expect("Error: Not a float");
                            let h: f32 = sh.parse().expect("Error: Not a float");
                            level.triggers.push(Trigger{rect: Rect::new(x, y, w, h), t: TriggerType::Kill}); // TODO: add different trigger types
                        }
                        _ => {

                        }
                    }
                }
            } else if is_key_pressed(KeyCode::B) { // test map
                // copy all vectors to test variants, so it can be reset easily
                let mut test_level = level.clone();
                let mut test_entities: Vec<Box<dyn Entity>> = Vec::new();

                let mut test_events: Vec<EventType> = Vec::new();
                
                for entity in &entities {
                    test_entities.push(entity.box_clone());
                }

                play_sound(&test_level.music, PlaySoundParams{looped: true, volume: 0.5});

                'test_loop: loop {
                    clear_background(BLACK);

                    test_level.draw();
                    test_level.x = (screen_width() / 2.) - test_entities[0].get_pos().x;

                    if is_mouse_button_down(MouseButton::Middle) {
                        test_level.x += mouse_position().0 - m_last_pos.0;
                    }

                    // queue of entities that need to be spawned
                    let mut to_spawn: Vec<Box<dyn Entity>> = Vec::new();
                    // a vec of events that have already been enacted
                    let mut covered_events: Vec<usize> = Vec::new();
                    for entity in test_entities.iter_mut() {
                        let result = entity.update(&test_level);
                        if let Some(e) = result {
                            test_events.push(e);
                        }

                        for (i, event) in (&test_events).iter().enumerate() {
                            // Handle spawn events
                            match event {
                                EventType::SpawnFart{rect, d, ivel} => {
                                    if !covered_events.contains(&i) {
                                        to_spawn.push(Box::new(Fart::new(*rect, *d, *ivel, &assets)));
                                        covered_events.push(i);
                                    }
                                },
                                EventType::Laser {start_pos, angle, speed, distance, duration} => {
                                    if !covered_events.contains(&i) {
                                        to_spawn.push(Box::new(Laser{
                                            start_pos: *start_pos, angle: *angle, speed: *speed, distance: *distance, duration: *duration, dead: false,
                                        }));
                                        covered_events.push(i);
                                    }
                                },
                                EventType::SpawnGoblin {pos} => {
                                    to_spawn.push(Box::new(Enemy::new(*pos, &assets)));
                                },
                                _ => {}
                            }

                            // let entities interact with events
                            // println!("{:?}", event);
                            entity.give_event(event);
                        }
                    }

                    // spawn queued entities
                    for entity in to_spawn {
                        test_entities.push(entity);
                    }

                    // reset events vec
                    test_events = Vec::new();

                    // queue of entities that will be despawned
                    let mut to_kill: Vec<usize> = Vec::new();

                    for (i, entity) in test_entities.iter().enumerate() {
                        if entity.get_dead() {
                            to_kill.push(i);
                            continue;
                        }

                        // let entities look at other entities
                        let result = entity.give_data(&test_level, &test_entities);
                        if let Some(t) = result {
                            test_events.push(t)
                        }
                        entity.draw(&test_level);
                    }

                    // despawn queued entities
                    for i in to_kill {
                        if i >= test_entities.len() {
                            continue;
                        }
                        
                        if i == 0 {
                            stop_sound(&test_level.music);
                            play_sound(death_sound, PlaySoundParams::default());

                            let mut death_t = 0;
                            loop {
                                clear_background(BLACK);

                                death_t += 1;
                                if death_t > 1000 {
                                    break 'test_loop;
                                }

                                draw_texture(assets.images.get("death_screen").expect("Could not load death screen"), 0., 0., WHITE);
                                next_frame().await;
                            }
                        }

                        if let Some(PlaceMode::SpawnGobloronBoss) = test_entities[i].get_type() {
                            stop_sound(&test_level.music);
                            play_sound(win_music, PlaySoundParams{looped: true, volume: 1.});

                            loop {
                                clear_background(BLACK);

                                if is_key_pressed(KeyCode::Escape) {
                                    break 'test_loop;
                                }

                                draw_texture(assets.images.get("win_screen").expect("Could not load win screen"), 0., 0., WHITE);
                                next_frame().await;
                            }
                        }
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

                    for trigger in &test_level.triggers {
                        draw_rectangle_lines(trigger.rect.x + test_level.x, trigger.rect.y, trigger.rect.w, trigger.rect.h, 2., Color::from_rgba(255, 0, 255, 255));
                    }

                    if is_key_pressed(KeyCode::Escape) {
                        stop_sound(&test_level.music);
                        break 'test_loop;
                    }

                    let minimum_frame_time = 1. / 60.; // 60 FPS
                    let frame_time = get_frame_time();
                    if frame_time < minimum_frame_time {
                        let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
                        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
                    }

                    // set the last mouse position
                    m_last_pos = mouse_position();

                    next_frame().await;
                }
            }
        // pan the camera
        } else if is_key_down(KeyCode::A) {
            level.x += LEVEL_SCROLL_SPEED * level_scroll_boost;
        } else if is_key_down(KeyCode::D) {
            level.x -= LEVEL_SCROLL_SPEED * level_scroll_boost;
        }

        // make camera go FAST
        if is_key_down(KeyCode::LeftShift) {
            level_scroll_boost = 2.;
        } else {
            level_scroll_boost = 1.;
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

        for trigger in &level.triggers {
            draw_rectangle_lines(trigger.rect.x + level.x, trigger.rect.y, trigger.rect.w, trigger.rect.h, 2., Color::from_rgba(255, 0, 255, 255));
        }

        for d in &map_data {
            match d.0 {
                PlaceMode::Trigger => {
                    draw_rectangle_lines(d.1.x + level.x, d.1.y, d.1.w, d.1.h, 2., Color::from_rgba(255, 0, 255, 255));
                },
                _ => {}
            }
        }

        m_last_pos = mouse_position();

        next_frame().await;
    }
}