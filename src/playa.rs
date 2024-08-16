// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::level::*;
use crate::map_edit::*;
use crate::event::*;

// HEHEHEHEHEHEHEHEHE
#[derive(Clone)]
pub struct Fart {
	pos: Vec2,
	vel: Vec2,
	hitbox: Rect,
	// how long the fart lasts
	lifetime: i32,
	current_image: Texture2D,
	pub dead: bool,
}

impl Fart {
	const SPEED: f32 = 5.;
	pub fn new(pos: Vec2, d: f32, assets: &AssetManager) -> Self {
		Self {
			pos,
			// moves only horizontally
			vel: vec2(d * Self::SPEED, 0.),
			hitbox: Rect::new(pos.x, pos.y, 50., 50.),
			lifetime: 50,
			current_image: assets.images.get("fart").unwrap().clone(),
			dead: false,
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
							return Some(EventType::Damage{pos: self.pos + vec2(self.hitbox.w / 2., self.hitbox.h / 2.)});
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
}

impl Player {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			movement_system: MovementSystem::new(pos, 1., 0.5, -15., Rect::new(0., 0., 32., 64.)),
			current_image: assets.images.get("assu_chan").unwrap().clone(),
			label_image: assets.images.get("ching_chong").unwrap().clone(),
			dead: false,
			flipped: false,
		}
	}

	fn get_center(&self) -> Vec2 {
		self.movement_system.get_center()
	}
}

impl Entity for Player {
	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.movement_system.pos.x + level.x, self.movement_system.pos.y, WHITE);
		draw_texture(&self.label_image, self.movement_system.pos.x + level.x - 32., self.movement_system.pos.y - 32., WHITE);
	}

	fn update(&mut self, level: &Level) -> Option<EventType> {
		// EventType to be returned, if at all
		let mut r_event: Option<EventType> = None;

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
        	r_event = Some(EventType::SpawnFart{pos: self.get_center(), d})
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
				self.dead = true;
				return;
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