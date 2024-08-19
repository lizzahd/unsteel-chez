use ::rand::Rng;
use macroquad::prelude::*;

use crate::primimptnevs::*;
use crate::assets::*;
use crate::entittie::*;
use crate::level::*;
use crate::event::*;
use crate::map_edit::*;

const PI: f32 = std::f64::consts::PI as f32;
const PI_H: f32 = PI / 2.;

#[derive(Clone)]
pub struct Enemy {
	movement_system: MovementSystem,
	current_image: Texture2D,
	dead: bool,
	flipped: bool,
	target_position: Option<Vec2>,
}

impl Enemy {
	const DETECTION_RADIUS: f32 = 500.;

    pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
    	Self {
			movement_system: MovementSystem::new(pos, 0.5, 0.4, -10., Rect::new(0., 0., 100., 116.)),
			current_image: assets.images.get("gorblin").unwrap().clone(),
			dead: false,
			flipped: false,
			target_position: None,
    	}
    }

    fn jump(&mut self) {
    	if self.movement_system.grounded {
	    	self.movement_system.vel.y = self.movement_system.jump_acc;
    	}
    }

    fn move_left(&mut self) {
    	if self.movement_system.can_move_left {
    		self.movement_system.pos.x -= self.movement_system.move_speed;
	        // this makes it all smooth and stuff
	        self.movement_system.vel.x -= self.movement_system.move_acc;
    	} else {
    		self.jump();
    	}
        self.flipped = true;
    }

    fn move_right(&mut self) {
    	if self.movement_system.can_move_right {
    		self.movement_system.pos.x += self.movement_system.move_speed;
	        // this makes it all smooth and stuff
	        self.movement_system.vel.x += self.movement_system.move_acc;
    	} else {
    		self.jump();
    	}
        self.flipped = false;
    }
}

impl Entity for Enemy {
	fn update(&mut self, level: &Level) -> Option<EventType> {
		if self.movement_system.grounded {
        	self.movement_system.vel.y = 0.;
        } else {
        	self.movement_system.vel.y += GRAVITY;
        }

        if let Some(pos) = self.target_position {
        	if pos.x < self.get_pos().x {
        		self.move_left();
        	} else if pos.x > self.get_pos().x {
        		self.move_right();
        	}

        	// randomly jump
			if ::rand::thread_rng().gen_range(0..=50) < 3 && self.movement_system.grounded {
				self.jump();
			}

        	if dist(&self.get_pos(), &pos) > Self::DETECTION_RADIUS {
	        	self.target_position = None;
	        }
        }

		// behavior must go before this point
        self.movement_system.update(level);

        None
	}

	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.movement_system.pos.x + level.x, self.movement_system.pos.y, WHITE);
	}

	fn get_hitbox(&self) -> Rect {
		self.movement_system.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.movement_system.pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::SpawnGoblin)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		let mut r_event: Option<EventType> = None;

		for entity in entities.iter() {
			if let Some(t) = entity.get_type() {
				match t {
					PlaceMode::SpawnPlayer => {
						if dist(&entity.get_pos(), &self.get_pos()) <= Self::DETECTION_RADIUS {
							r_event = Some(EventType::PlayerSpotted{pos: entity.get_pos().clone()});
						}
					},
					PlaceMode::Laser => {
						let d = dist(&self.get_pos(), &entity.get_pos());
						if d <= entity.get_hitbox().h {
							// entity.get_hitbox().w is a lazy way of sneakily getting the angle from a laser
							let damage_point = vec2(entity.get_pos().x + entity.get_hitbox().w.cos() * d, entity.get_pos().y + entity.get_hitbox().w.sin() * d);
							if self.get_hitbox().contains(damage_point) {
								return Some(EventType::Damage{rect: Rect::new(damage_point.x - 5., damage_point.y - 5., 10., 10.)});
							}
						}
					},
					_ => {}
				}
			}
		}

		return r_event;
	}

	fn give_event(&mut self, event: &EventType) {
		match event {
			// get killed by damage
			EventType::Damage{rect} => {
				if self.get_hitbox().overlaps(rect) {
					self.dead = true;
					return;
				}
			},
			EventType::PlayerSpotted{pos} => {
				if dist(pos, &self.get_pos()) <= Self::DETECTION_RADIUS {
					self.target_position = Some(*pos);
				}
			},
			_ => {}
		}
	}

	fn get_dead(&self) -> bool {
		self.dead
	}

	fn get_hp(&self) -> i32 {
		1
	}
}

#[derive(Debug, Clone)]
pub struct GobloronBoss {
	pub hitbox: Rect,
	pub eye: Eye,
	pub eye_image: Texture2D,
	pub eye_damage_image: Texture2D,
	pub eye_dead_image: Texture2D,
	pub nip_image: Texture2D,
	pub nip_damage_image: Texture2D,
	pub nip_dead_image: Texture2D,
	pub dead: bool,
	left_nip: Nip,
	right_nip: Nip,
}

impl GobloronBoss {
	const MOUTH_POS: Vec2 = vec2(1954., 258.);

	pub fn new(assets: &AssetManager) -> Self {
		Self {
			hitbox: Rect::new(1939., 56., 570., 592.),
			eye: Eye::new(vec2(1954., 118.)),
			left_nip: Nip::new(vec2(2175., 440.)),
			right_nip: Nip::new(vec2(2136., 396.)),
			eye_image: assets.images.get("gobloron_eye").expect("could not load gobloron_eye").clone(),
			eye_damage_image: assets.images.get("gobloron_eye_damage").expect("could not load gobloron_eye_damage").clone(),
			eye_dead_image: assets.images.get("gobloron_eye_dead").expect("could not load gobloron_eye_damage").clone(),
			nip_image: assets.images.get("gobloron_nip").expect("could not load gobloron_nip").clone(),
			nip_damage_image: assets.images.get("gobloron_nip_damage").expect("could not load gobloron_nip_damage").clone(),
			nip_dead_image: assets.images.get("gobloron_nip_dead").expect("could not load gobloron_dead_damage").clone(),
			dead: false,
		}
	}
}

impl Entity for GobloronBoss {
	fn update(&mut self, _level: &Level) -> Option<EventType> {
		if self.dead {
			return None;
		}

		let mut e_result: Option<EventType> = None;

		e_result = self.eye.update();

        self.left_nip.update();
        if self.left_nip.fire_cool > 0 {
        	self.left_nip.fire_cool -= 1;
        }

		if !self.left_nip.spawned_laser && self.left_nip.firing && e_result.is_none() {
			e_result = Some(EventType::Laser {
        		start_pos: self.left_nip.pos + Nip::NIPPLE_POINT,
        		angle: PI,
        		speed: 0.,
        		distance: self.left_nip.pos.x,
        		duration: self.left_nip.firing_duration,
        	});
        	self.left_nip.spawned_laser = true;
		}
        self.right_nip.update();
        if !self.right_nip.spawned_laser && self.right_nip.firing && e_result.is_none() {
			e_result = Some(EventType::Laser {
        		start_pos: self.right_nip.pos + Nip::NIPPLE_POINT,
        		angle: PI,
        		speed: 0.,
        		distance: self.right_nip.pos.x,
        		duration: self.right_nip.firing_duration,
        	});
        	self.right_nip.spawned_laser = true;
		}

		// TODO: Make him spawn goblins when e_result is still None at this point

		if e_result.is_none() {
			if ::rand::thread_rng().gen_range(0..=1000) < 3 {
				e_result = Some(EventType::SpawnGoblin{pos: Self::MOUTH_POS.clone()});
			}
		}

		if self.eye.dead && self.left_nip.dead && self.right_nip.dead {
			self.dead = true;
		}

        return e_result;
	}

	fn draw(&self, level: &Level) {
		let eye_tex = if self.eye.dead {
			&self.eye_dead_image
		} else if self.eye.damage_cool > 0 {
			&self.eye_damage_image
		} else {
			&self.eye_image
		};
		self.eye.draw(level);
		draw_texture(eye_tex, self.eye.pos.x + level.x, self.eye.pos.y, WHITE);

		let left_nip_tex = if self.left_nip.dead {
			&self.nip_dead_image
		} else if self.left_nip.damage_cool > 0 {
			&self.nip_damage_image
		} else {
			&self.nip_image
		};
		self.left_nip.draw(level);
		draw_texture(left_nip_tex, self.left_nip.pos.x + level.x, self.left_nip.pos.y, WHITE);

		let right_nip_tex = if self.right_nip.dead {
			&self.nip_dead_image
		} else if self.right_nip.damage_cool > 0 {
			&self.nip_damage_image
		} else {
			&self.nip_image
		};
		self.right_nip.draw(level);
		draw_texture(right_nip_tex, self.right_nip.pos.x + level.x, self.right_nip.pos.y, WHITE);
	}

	fn get_hitbox(&self) -> Rect {
		self.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.hitbox.center()
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::SpawnGobloronBoss)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, _entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		// Manually check nipple lasers for player detection
		let mut e_result: Option<EventType> = None;

		return e_result;
	}

	fn give_event(&mut self, event: &EventType) {
		match event {
			// get killed by damage
			EventType::Damage{rect} => {
				if self.eye.hitbox.overlaps(rect) {
					self.eye.hurt(1);
					return;
				} else if self.right_nip.hitbox.overlaps(rect) {
					self.right_nip.hurt(1);
					return;
				} else if self.left_nip.hitbox.overlaps(rect) {
					self.left_nip.hurt(1);
					return;
				}
			},
			_ => {}
		}
	}

	fn get_dead(&self) -> bool {
		self.dead
	}

	fn get_hp(&self) -> i32 {
		self.eye.hp + self.left_nip.hp + self.right_nip.hp
	}
}

#[derive(Debug, Clone)]
pub struct Eye {
	pub pos: Vec2,
	pub hitbox: Rect,
	pub angle: f32, // this is f32 btw. angle. yeah
	pub hp: i32,
	pub dead: bool,
	pub damage_cool: i32,
	pub fire_cool: i32,
	pub warmup_cool: i32,
	pub warming_up: bool,
	pub firing_duration: i32,
	pub firing: bool,
	pub target_firetime: i32,
	pub target_location: Vec2,
}

impl Eye {
	const MAX_FIRE_COOL: i32 = 500;
	const MAX_WARMUP_DURATION: i32 = 300;
	const MAX_FIRE_DURATION: i32 = 150;
	const EYE_POINT: Vec2 = vec2(11., 33.);
	pub const LASER_DIST: f32 = 475.;
	pub const LASER_SPEED: f32 = 0.005;

	pub fn new(pos: Vec2) -> Self {
		Self {
			pos,
			hitbox: Rect::new(pos.x, pos.y, 60., 44.),
			angle: PI_H,
			hp: 50,
			dead: false,
			damage_cool: 10,
			fire_cool: Self::MAX_FIRE_COOL,
			warmup_cool: Self::MAX_WARMUP_DURATION,
			warming_up: false,
			firing: false,
			firing_duration: Self::get_fire_duration(),
			target_firetime: Self::get_target_firetime(),
			target_location: pos,
		}
	}

	fn get_fire_duration() -> i32 {
		Self::MAX_FIRE_DURATION * ::rand::thread_rng().gen_range(1..=3)
	}

	fn get_target_firetime() -> i32 {
		::rand::thread_rng().gen_range(0..Self::MAX_WARMUP_DURATION)
	}

	pub fn hurt(&mut self, damage: i32) {
		if self.dead || self.damage_cool > 0 {
			return;
		}

		self.hp -= damage;
		self.damage_cool = 10;
		if self.hp <= 0 {
			self.dead = true;
		}
	}

	pub fn update(&mut self) -> Option<EventType> {
		let mut r_event = None;

		if self.dead {
			return r_event;
		}

		if self.damage_cool > 0 {
			self.damage_cool -= 1;
		}

		if self.warming_up {
			if self.warmup_cool > 0 {
				self.warmup_cool -= 1;
			} else {
				self.firing = true;
				self.warming_up = false;
				self.warmup_cool = Self::MAX_WARMUP_DURATION;
				self.fire_cool = Self::MAX_FIRE_COOL;
				r_event = Some(EventType::Laser {
	        		start_pos: self.pos + Self::EYE_POINT,
	        		angle: self.angle,
	        		speed: Self::LASER_SPEED,
	        		distance: Self::LASER_DIST,
	        		duration: self.firing_duration,
	        	}); 
			}
		} else {
			if self.fire_cool > 0 {
				self.fire_cool -= 1;
			} else {
				self.warming_up = true;
			}
		}

		if self.firing {
			self.angle += Self::LASER_SPEED;
			if self.angle > PI {
				self.angle = PI_H;
			}

			if self.firing_duration > 0 {
				self.firing_duration -= 1;
			} else {
				self.firing = false;
				self.firing_duration = Self::get_fire_duration();
				self.angle = PI_H;
			}
		}

		return r_event;
	}

	pub fn draw(&self, level: &Level) {
		if self.dead {
			return;
		}

		// draw_line(self.pos.x + Self::EYE_POINT.x + level.x, self.pos.y + Self::EYE_POINT.y,
		// 	(self.pos.x + Self::EYE_POINT.x + self.angle.cos() * Self::LASER_DIST) + level.x,
		// 	(self.pos.y + Self::EYE_POINT.y + self.angle.sin() * Self::LASER_DIST), 2., RED);

		if self.warming_up {
			// draw_line(self.pos.x + Self::EYE_POINT.x + level.x, self.pos.y + Self::EYE_POINT.y,
			// 	(self.pos.x + Self::EYE_POINT.x + self.angle.cos() * Self::LASER_DIST) + level.x,
			// 	(self.pos.y + Self::EYE_POINT.y + self.angle.sin() * Self::LASER_DIST), 2., RED);
			draw_circle(self.pos.x + Self::EYE_POINT.x + level.x, self.pos.y + Self::EYE_POINT.y, 5., RED);
		}

		if self.firing {
			draw_line(self.pos.x + Self::EYE_POINT.x + level.x, self.pos.y + Self::EYE_POINT.y,
				(self.pos.x + Self::EYE_POINT.x + self.angle.cos() * Self::LASER_DIST) + level.x,
				(self.pos.y + Self::EYE_POINT.y + self.angle.sin() * Self::LASER_DIST), 10., RED);
		}
	}
}

#[derive(Debug, Clone)]
pub struct Laser {
	pub start_pos: Vec2,
	pub angle: f32,
	pub speed: f32,
	pub distance: f32,
	pub duration: i32,
	pub dead: bool,
}

impl Entity for Laser {
	fn update(&mut self, level: &Level) -> Option<EventType> {
		if self.dead {
			return None;
		}

		self.angle += self.speed;
		if self.angle > PI {
			self.angle = PI_H;
		}

		self.duration -= 1;
		if self.duration <= 0 {
			self.dead = true;
		}

        None
	}

	fn draw(&self, level: &Level) {
	}

	fn get_hitbox(&self) -> Rect {
		Rect::new(self.start_pos.x, self.start_pos.y, self.angle, self.distance) // returns angle disguised as Rect.w
	}

	fn get_pos(&self) -> Vec2 {
		self.start_pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::Laser)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, _entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		None
	}

	fn give_event(&mut self, event: &EventType) {
	}

	fn get_dead(&self) -> bool {
		self.dead
	}

	fn get_hp(&self) -> i32 {
		1
	}
}

#[derive(Debug, Clone)]
struct Nip {
	pub pos: Vec2,
	pub hitbox: Rect,
	pub hp: i32,
	pub dead: bool,
	pub damage_cool: i32,
	pub fire_cool: i32,
	pub warmup_cool: i32,
	pub warming_up: bool,
	pub firing_duration: i32,
	pub firing: bool,
	pub spawned_laser: bool,
}

impl Nip {
	const MAX_FIRE_COOL: i32 = 10000; // 10000 is good
	const MAX_WARMUP_DURATION: i32 = 200;
	const MAX_FIRE_DURATION: i32 = 100;
	const NIPPLE_POINT: Vec2 = vec2(3., 14.);

	pub fn new(pos: Vec2) -> Self {
		Self {
			pos,
			hitbox: Rect::new(pos.x, pos.y, 50., 50.),
			hp: 20,
			dead: false,
			damage_cool: 10,
			fire_cool: Self::MAX_FIRE_COOL,
			warmup_cool: Self::MAX_WARMUP_DURATION,
			warming_up: false,
			firing: false,
			firing_duration: Self::MAX_FIRE_DURATION,
			spawned_laser: false,
		}
	}

	pub fn hurt(&mut self, damage: i32) {
		if self.dead || self.damage_cool > 0 {
			return;
		}

		self.hp -= damage;
		self.damage_cool = 10;
		if self.hp <= 0 {
			self.dead = true;
		}
	}

	pub fn update(&mut self) {
		if self.dead {
			return;
		}

		if self.damage_cool > 0 {
			self.damage_cool -= 1;
		}

		if self.warming_up {
			if self.warmup_cool > 0 {
				self.warmup_cool -= 1;
			} else {
				self.firing = true;
				self.warming_up = false;
				self.warmup_cool = Self::MAX_WARMUP_DURATION;
				self.fire_cool = Self::MAX_FIRE_COOL;
			}
		} else {
			if self.fire_cool > 0 {
				self.fire_cool -= ::rand::thread_rng().gen_range(0..=10);
			} else {
				self.warming_up = true;
			}
		}

		if self.firing {
			if self.firing_duration > 0 {
				self.firing_duration -= 1;
			} else {
				self.firing = false;
				self.spawned_laser = false;
				self.firing_duration = Self::MAX_FIRE_DURATION;
			}
		}
	}

	pub fn draw(&self, level: &Level) {
		if self.dead {
			return;
		}

		if self.warming_up {
			draw_line(self.pos.x + Self::NIPPLE_POINT.x + level.x, self.pos.y + Self::NIPPLE_POINT.y, 0., self.pos.y + Self::NIPPLE_POINT.y, 1., RED);
		}

		if self.firing {
			draw_line(self.pos.x + Self::NIPPLE_POINT.x + level.x, self.pos.y + Self::NIPPLE_POINT.y, 0., self.pos.y + Self::NIPPLE_POINT.y, 8., RED);
		}
	}
}