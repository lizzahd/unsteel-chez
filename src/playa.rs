// burt ass
use ::rand::Rng;

use macroquad::prelude::*;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};

use crate::entittie::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::level::*;
use crate::map_edit::*;
use crate::event::*;
use crate::enemy::*;

// HEHEHEHEHEHEHEHEHE
#[derive(Clone)]
pub struct Fart {
	pos: Vec2,
	vel: Vec2,
	hitbox: Rect,
	// how long the fart lasts
	lifetime: i32,
	current_image: Texture2D,
	pub dead: bool
}

impl Fart {
	const SPEED: f32 = 8.;
	pub fn new(rect: Rect, d: f32, ivel: f32, assets: &AssetManager) -> Self {
		let mut rng = ::rand::thread_rng();
		let n = rng.gen_range(0..=11);
		play_sound(&assets.sounds.get(&format!("fart{}", n)).expect("Could not load fart sound"), PlaySoundParams::default());
		// play_sound(&assets.sounds.get("fart0").expect("Could not load fart sound"), PlaySoundParams::default());

		Self {
			pos: vec2(rect.x, rect.y),
			// moves only horizontally
			vel: vec2(d * (Self::SPEED + ivel.abs()), 0.),
			hitbox: rect,
			lifetime: 50,
			current_image: assets.images.get("fart").unwrap().clone(),
			dead: false
		}
	}
}

impl Entity for Fart {
	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.pos.x + level.x, self.pos.y, WHITE);
	}

	fn update(&mut self, level: &Level) -> Option<EventType> {
		if self.dead {
			return None;
		}

		for hitbox in &level.collision.rect_hitboxes {
        	if self.hitbox.overlaps(hitbox) {
        		self.dead = true;
    		}
        }

		self.pos += self.vel;
		self.hitbox.x = self.pos.x;
		self.hitbox.y = self.pos.y;

		self.lifetime -= 1;
		if self.lifetime <= 0 {
			// fart dissipates
			self.dead = true;
		}

		None
	}

	fn get_hitbox(&self) -> Rect {
		self.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::Projectile)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		if self.dead {
			return None;
		}

		// check if its touchin an enemy
		for entity in entities.iter() {
			if let Some(t) = entity.get_type() {
				match t {
					PlaceMode::SpawnGoblin => {
						if entity.get_hitbox().overlaps(&self.get_hitbox()) {
							// return Some(EventType::Damage{pos: self.pos + vec2(self.hitbox.w / 2., self.hitbox.h / 2.)});
							return Some(EventType::Damage{rect: self.hitbox});
						}
					},
					PlaceMode::SpawnGobloronBoss => {
						if entity.get_hitbox().overlaps(&self.get_hitbox()) {
							return Some(EventType::Damage{rect: self.hitbox});
							// return Some(EventType::Damage{pos: self.pos + vec2(self.hitbox.w / 2., self.hitbox.h / 2.)});
						}
					},
					PlaceMode::SpawnPlayer => {
						continue;
					},
					_ => {
					}
				}
			}
		}

		None
	}

	fn give_event(&mut self, event: &EventType) {
	}

	fn get_dead(&self) -> bool {
		self.dead
	}
}

#[derive(Clone)]
pub struct Player {
	movement_system: MovementSystem,
	current_image: Texture2D,
	// ching chong must be drawn seperately due to hitbox problems
	label_image: Texture2D,
	dead: bool,
	flipped: bool,

	lovecraft: i32,
	dammit_cooldown: i32,

	lovecraft_cooldown: i32,

	fart_power: i32,
	fart_cooldown: i32,

	ow: Sound,
	hp: Sound,

	fart_icon: Texture2D,
	lovecraft_icon: Texture2D
}

impl Player {
	const FART_RESET: i32 = 90;
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			movement_system: MovementSystem::new(pos, 1., 0.5, -13., Rect::new(0., 0., 32., 64.)),
			current_image: assets.images.get("assu_chan").unwrap().clone(),
			label_image: assets.images.get("ching_chong").unwrap().clone(),
			dead: false,
			flipped: false,
			lovecraft: 100,
			dammit_cooldown: 0,
			lovecraft_cooldown: 0,
			fart_power: 100,
			fart_cooldown: 0,
			ow: assets.sounds.get("slap7").expect("nuh uh").clone(),
			hp: assets.sounds.get("hp").expect("no").clone(),
			fart_icon: assets.images.get("buttfart").unwrap().clone(),
			lovecraft_icon: assets.images.get("lovecraft").unwrap().clone()
		}
	}

	fn get_center(&self) -> Vec2 {
		self.movement_system.get_center()
	}
}

impl Entity for Player {
	fn draw(&self, level: &Level) {
		draw_texture_ex(&self.current_image, self.movement_system.pos.x + level.x, self.movement_system.pos.y, WHITE, 
			DrawTextureParams {
				flip_x: self.flipped,
				..Default::default()
		});

		draw_texture_ex(&self.label_image, self.movement_system.pos.x + level.x - 32., self.movement_system.pos.y - 32., WHITE, 
			DrawTextureParams {
				flip_x: self.flipped,
				..Default::default()
		});

		draw_rectangle(10., 10., 20., 100., BROWN);
		draw_rectangle(10., (100. - self.fart_power as f32 + 10.), 20., self.fart_power as f32, GREEN);
		draw_texture(&self.fart_icon, 10., 120., WHITE);

		draw_rectangle(40., 10., 20., 100., RED);
		draw_rectangle(40., (100. - self.lovecraft as f32 + 10.), 20., self.lovecraft as f32, GREEN);
		draw_texture(&self.lovecraft_icon, 40., 120., WHITE);
	}

	fn update(&mut self, level: &Level) -> Option<EventType> {
		// EventType to be returned, if at all
		let mut r_event: Option<EventType> = None;

		for trigger in &level.triggers {
			if self.get_hitbox().overlaps(&trigger.rect) {
				match trigger.t {
					TriggerType::Kill => {
						self.dead = true;
					}
				}
			}
		}

		// JUMPIES
        if self.movement_system.grounded {
        	self.movement_system.vel.y = 0.;
        	if is_key_down(KeyCode::Space) {
        		self.movement_system.vel.y = self.movement_system.jump_acc;
        	}
        } else {
        	self.movement_system.vel.y += GRAVITY;
        }

        if is_key_down(KeyCode::A) && self.movement_system.can_move_left {
            self.movement_system.pos.x -= self.movement_system.move_speed;
            // this makes it all smooth and stuff
            self.movement_system.vel.x -= self.movement_system.move_acc;
            self.flipped = true;
        }
        if is_key_down(KeyCode::D) && self.movement_system.can_move_right {
            self.movement_system.pos.x += self.movement_system.move_speed;
            // this makes it all smooth and stuff
            self.movement_system.vel.x += self.movement_system.move_acc;
            self.flipped = false;
        }

        // face towards the mouse
        self.flipped = if mouse_position().0 < self.get_pos().x + level.x {
        	true
        } else {
        	false
        };

        if is_mouse_button_pressed(MouseButton::Left) {
        	// get fard direction
        	let d = if self.flipped {
        		-1.
        	} else {
        		1.
        	};

        	// fard.
        	// it gets subtracted by a vec2 because that makes it perfectly centered on the player
        	if self.fart_power > 0 {
        		r_event = Some(EventType::SpawnFart{rect: Rect::new(self.get_center().x - 50., self.get_center().y - 50., 50., 50.), d, ivel: self.movement_system.vel.x});
        		
        		self.fart_power -= 10;
        		self.fart_cooldown = Self::FART_RESET;
        		if self.fart_power < 0 { self.fart_power = 0; }
        	}
        }

        if self.fart_cooldown > 0 {
        	self.fart_cooldown -= 1;
        }

        if self.fart_cooldown == 0 && self.fart_power < 100 {
        	self.fart_power += 1;
        }

        if self.lovecraft_cooldown > 0 {
        	self.lovecraft_cooldown -= 1;
        }

        if self.dammit_cooldown > 0 {
        	self.dammit_cooldown -= 1;
        }

        // handle physics and stuff
        self.movement_system.update(level);

        // return produced event
        return r_event;
	}

	fn get_hitbox(&self) -> Rect {
		self.movement_system.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.movement_system.pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::SpawnPlayer)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	// this will return an event if anything happens, which will be appended to an events vector that will be looped through post-update
	fn give_data(&self, _level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		for entity in entities.iter() {
			if let Some(t) = entity.get_type() {
				match t {
					// pick up some soap, that you hopefully didn't drop
					PlaceMode::Dawn => {
						if entity.get_hitbox().overlaps(&self.get_hitbox()) {
							return Some(EventType::Pickup{pos: self.get_center()});
						}
					},
					// get touched by goblins
					PlaceMode::SpawnGoblin => {
						if entity.get_hitbox().overlaps(&self.get_hitbox()) {
							return Some(EventType::KillPlayer);
						}
					},
					// -- Laser functionality --
					// The player will check if the distance between himself and the laser is less than Eye::LASER_DIST
					// Spawn a kill box at laser_point.x + cos(laser_angle) * d, laser_point.y + sin(laser_angle) * d
					PlaceMode::Laser => {
						let d = dist(&self.get_center(), &entity.get_pos());
						if d <= entity.get_hitbox().h { // h is distance
							// entity.get_hitbox().w is a lazy way of sneakily getting the angle from a laser
							let damage_point = vec2(entity.get_pos().x + entity.get_hitbox().w.cos() * d, entity.get_pos().y + entity.get_hitbox().w.sin() * d);
							if self.get_hitbox().contains(damage_point) {
								return Some(EventType::KillPlayer);
							}
						}
					},
					// this is for preventing the player from touching himself
					PlaceMode::SpawnPlayer => {
						continue;
					},
					_ => {
					}
				}
			}
		}

		None
	}

	fn give_event(&mut self, event: &EventType) {
		match event {
			// make the player die
			EventType::KillPlayer => {
				if self.dammit_cooldown == 0 {
					play_sound(&self.ow, PlaySoundParams{looped: false, volume: 1.});
					self.lovecraft -= 10;
					self.dammit_cooldown = 20;
				}

				if self.lovecraft < 1 {
					self.dead = true;
					return;
				}
			},

			EventType::HPGrab => {
				if self.lovecraft_cooldown == 0 {
					play_sound(&self.hp, PlaySoundParams{looped: false, volume: 0.3});
					self.lovecraft += 20;
					if self.lovecraft > 100 {
						self.lovecraft = 100;
					}
					self.lovecraft_cooldown = 10;
				}

			},
			_ => {
			}
		}
	}

	fn get_dead(&self) -> bool {
		// ded
		self.dead
	}
}