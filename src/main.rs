use macroquad::prelude::*;
use macroquad::audio::{play_sound, stop_sound, PlaySoundParams};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::entittie::*;
use crate::playa::*;
use crate::assets::*;
use crate::enemy::*;
use crate::level::*;
use crate::map_edit::*;
use crate::event::*;
use crate::touchytouchy::*;

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

    arse().await;
}

async fn arse() -> io::Result<()> {
    // load all assets
    let assets = AssetManager::new("assets").await;

    let mut level = Level::new("level_0").await;
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();

    let mut events: Vec<EventType> = Vec::new();

    play_sound(&level.music, PlaySoundParams{looped: true, volume: 0.5});

    let mut taken_damage = false;

    let mut player_start = vec2(0., 0.);

    let death_sound = assets.sounds.get("death").expect("could not load death sound");
    let win_music = assets.sounds.get("win_music").expect("could not load win music");
    let cheese_music = assets.sounds.get("cheese").expect("could not load cheese music");
    let win_nohit_music = assets.sounds.get("win_nohit").expect("could not load win_nohit music");

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
            },
            "CheeseTrigger" => {
                let x: f32 = sx.parse().expect("Error: Not a float");
                let y: f32 = sy.parse().expect("Error: Not a float");
                let w: f32 = sw.parse().expect("Error: Not a float");
                let h: f32 = sh.parse().expect("Error: Not a float");
                level.triggers.push(Trigger{rect: Rect::new(x, y, w, h), t: TriggerType::Cheese}); // TODO: add different trigger types
            },
            _ => {

            }
        }
    }

    // mouse last position. is updated at the end of every frame
    let mut m_last_pos = mouse_position();

    
    'main_loop: loop {
        clear_background(BLACK);

        level.draw();
        level.x = (screen_width() / 2.) - entities[0].get_pos().x;

        if is_mouse_button_down(MouseButton::Middle) {
            level.x += mouse_position().0 - m_last_pos.0;
        }

        // queue of entities that need to be spawned
        let mut to_spawn: Vec<Box<dyn Entity>> = Vec::new();
        // a vec of events that have already been enacted
        let mut covered_events: Vec<usize> = Vec::new();
        for entity in entities.iter_mut() {
            let result = entity.update(&level);
            if let Some(e) = result {
                events.push(e);
            }

            for (i, event) in (&events).iter().enumerate() {
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
                    EventType::Win => {
                        stop_sound(&level.music);
                        play_sound(cheese_music, PlaySoundParams::default());

                        let mut death_t = 0;
                        loop {
                            clear_background(BLACK);

                            death_t += 1;
                            if death_t > 800 {
                                if !taken_damage {
                                    play_sound(win_nohit_music, PlaySoundParams::default());

                                    let mut death_t = 0;
                                    loop {
                                        clear_background(BLACK);

                                        death_t += 1;
                                        if death_t > 800 {
                                            draw_texture(&load_texture("assets/loading_screen.png").await.unwrap(), 0., 0., WHITE);
                                            next_frame().await;

                                            entities = Vec::new();
                                            events = Vec::new();

                                            level = Level::new("level_1").await;

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
                                                    },
                                                    "CheeseTrigger" => {
                                                        let x: f32 = sx.parse().expect("Error: Not a float");
                                                        let y: f32 = sy.parse().expect("Error: Not a float");
                                                        let w: f32 = sw.parse().expect("Error: Not a float");
                                                        let h: f32 = sh.parse().expect("Error: Not a float");
                                                        level.triggers.push(Trigger{rect: Rect::new(x, y, w, h), t: TriggerType::Cheese}); // TODO: add different trigger types
                                                    },
                                                    _ => {

                                                    }
                                                }
                                            }

                                            play_sound(&level.music, PlaySoundParams{looped: true, volume: 0.5});

                                            continue 'main_loop;
                                        }

                                        draw_texture(assets.images.get("win_nohit_screen").expect("Could not load death screen"), 0., 0., WHITE);
                                        next_frame().await;
                                    }
                                }
                                break 'main_loop;
                            }

                            draw_texture(assets.images.get("cheese_screen").expect("Could not load death screen"), 0., 0., WHITE);
                            next_frame().await;
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

        if entities[0].get_hp() < Player::MAX_HP {
            taken_damage = true;
        }

        // spawn queued entities
        for entity in to_spawn {
            entities.push(entity);
        }

        // reset events vec
        events = Vec::new();

        // queue of entities that will be despawned
        let mut to_kill: Vec<usize> = Vec::new();

        for (i, entity) in entities.iter().enumerate() {
            if entity.get_dead() {
                to_kill.push(i);
                continue;
            }

            // let entities look at other entities
            let result = entity.give_data(&level, &entities);
            if let Some(t) = result {
                events.push(t)
            }
            entity.draw(&level);
        }

        // despawn queued entities
        for i in to_kill {
            if i >= entities.len() {
                continue;
            }
            
            if i == 0 {
                stop_sound(&level.music);
                play_sound(death_sound, PlaySoundParams::default());

                let mut death_t = 0;
                loop {
                    clear_background(BLACK);

                    death_t += 1;
                    if death_t > 100 {
                        break 'main_loop;
                    }

                    draw_texture(assets.images.get("death_screen").expect("Could not load death screen"), 0., 0., WHITE);
                    next_frame().await;
                }
            }

            if let Some(PlaceMode::SpawnGobloronBoss) = entities[i].get_type() {
                stop_sound(&level.music);
                play_sound(win_music, PlaySoundParams{looped: true, volume: 1.});

                loop {
                    clear_background(BLACK);

                    if is_key_pressed(KeyCode::Escape) {
                        break 'main_loop;
                    }

                    draw_texture(assets.images.get("win_screen").expect("Could not load win screen"), 0., 0., WHITE);
                    next_frame().await;
                }
            }
            if i < entities.len() {
                entities.remove(i);
            }
        }

        // for hitbox in &mut level.collision.rect_hitboxes {
        //     draw_rectangle_lines(hitbox.x + level.x, hitbox.y, hitbox.w, hitbox.h, 2., Color::from_rgba(0, 255, 0, 255));
        // }

        // for platform in &mut level.collision.platforms {
        //     draw_line(platform.x + level.x, platform.y, platform.x + platform.w + level.x, platform.y, 2., Color::from_rgba(255, 0, 0, 255));
        // }

        // for trigger in &level.triggers {
        //     draw_rectangle_lines(trigger.rect.x + level.x, trigger.rect.y, trigger.rect.w, trigger.rect.h, 2., Color::from_rgba(255, 0, 255, 255));
        // }

        if is_key_pressed(KeyCode::Escape) {
            stop_sound(&level.music);
            break 'main_loop;
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

    Ok(())
}