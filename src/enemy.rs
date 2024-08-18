use macroquad::prelude::*;

use crate::primimptnevs::*;
use crate::assets::*;
use crate::entittie::*;
use crate::level::*;
use crate::event::*;
use crate::map_edit::*;

#[derive(Clone)]
pub struct Enemy {
	movement_system: MovementSystem,
	current_image: Texture2D,
	dead: bool,
}

impl Enemy {
    pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
    	Self {
			movement_system: MovementSystem::new(pos, 0.9, 0.4, -10., Rect::new(0., 0., 100., 116.)),
			current_image: assets.images.get("gorblin").unwrap().clone(),
			dead: false,
    	}
    }
}

impl Entity for Enemy {
	fn update(&mut self, level: &Level) -> Option<EventType> {
		if self.movement_system.grounded {
        	self.movement_system.vel.y = 0.;
        } else {
        	self.movement_system.vel.y += GRAVITY;
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

	fn give_data(&self, _level: &Level, _entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		None
	}

	fn give_event(&mut self, event: &EventType) {
		match event {
			// get killed by damage
			EventType::Damage{pos} => {
				if self.get_hitbox().contains(*pos) {
					self.dead = true;
					return;
				}
			},
			_ => {}
		}
	}

	fn get_dead(&self) -> bool {
		self.dead
	}
}

#[derive(Debug, Clone)]
pub struct GobloronBoss {
	pub eye_hitbox: Rect,
	pub eye_hp: i32,
	pub nip1_hitbox: Rect,
	pub nip2_hitbox: Rect,
	pub nip1_hp: i32,
	pub nip2_hp: i32,
	pub eye_image: Texture2D,
	pub eye_damage_image: Texture2D,
	pub nip_image: Texture2D,
	pub nip_damage_image: Texture2D,
	pub dead: bool,
}

impl GobloronBoss {
	pub fn new(assets: &AssetManager) -> Self {
		Self {
			eye_hitbox: Rect::new(1954., 118., 60., 44.),
			eye_hp: 10,
			nip1_hitbox: Rect::new(2136., 396., 30., 36.),
			nip2_hitbox: Rect::new(2175., 430., 30., 36.),
			nip1_hp: 20,
			nip2_hp: 20,
			eye_image: assets.images.get("gobloron_eye").expect("could not load gobloron_eye").clone(),
			eye_damage_image: assets.images.get("gobloron_eye_damage").expect("could not load gobloron_eye_damage").clone(),
			nip_image: assets.images.get("gobloron_nip").expect("could not load gobloron_nip").clone(),
			nip_damage_image: assets.images.get("gobloron_nip_damage").expect("could not load gobloron_nip_damage").clone(),
			dead: false,
		}
	}

	fn damage_eye(&mut self, damage: i32) {
		self.eye_hp -= damage;
	}

	fn damage_nip1(&mut self, damage: i32) {
		self.nip1_hp -= damage;
	}

	fn damage_nip2(&mut self, damage: i32) {
		self.nip2_hp -= damage;
	}
}

impl Entity for GobloronBoss {
	fn update(&mut self, level: &Level) -> Option<EventType> {
        None
	}

	fn draw(&self, level: &Level) {
		draw_texture(&self.eye_image, self.eye_hitbox.x + level.x, self.eye_hitbox.y, WHITE);
		draw_texture(&self.nip_image, self.nip1_hitbox.x + level.x, self.nip1_hitbox.y, WHITE);
		draw_texture(&self.nip_image, self.nip2_hitbox.x + level.x, self.nip2_hitbox.y, WHITE);
	}

	fn get_hitbox(&self) -> Rect {
		self.eye_hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.eye_hitbox.center()
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::SpawnGoblin)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, _entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		None
	}

	fn give_event(&mut self, event: &EventType) {
		match event {
			// get killed by damage
			EventType::Damage{pos} => {
				println!("UH OH ITS DAMAGE TIME");
				if self.eye_hitbox.contains(*pos) {
					println!("AAAAAAAAAAA MY EYE");
					self.damage_eye(1);
					return;
				} else if self.nip1_hitbox.contains(*pos) {
					println!("AAAAAAAAAAA MY RIGHT NIP");
					self.damage_nip1(1);
					return;
				} else if self.nip2_hitbox.contains(*pos) {
					println!("AAAAAAAAAAA MY LEFT NIP");
					self.damage_nip2(1);
					return;
				}
			},
			_ => {}
		}
	}

	fn get_dead(&self) -> bool {
		self.dead
	}
}

// TODO: Make Gobloron made up of multiple entities representing his body parts

#[derive(Debug, Clone)]
pub struct Nip {
	pub hp: i32,
	pub hitbox: Rect,
	pub image: Texture2D,
	pub damage_image: Texture2D,
	pub dead: bool,
}

impl Nip {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			hp: 20,
			hitbox: Rect::new(pos.x, pos.y, 30., 36.),
			image: assets.images.get("gobloron_nip").expect("could not load gobloron_nip").clone(),
			damage_image: assets.images.get("gobloron_nip_damage").expect("could not load gobloron_nip_damage").clone(),
			dead: false,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Eye {
	pub hp: i32,
	pub hitbox: Rect,
	pub image: Texture2D,
	pub damage_image: Texture2D,
	pub dead: bool,
}

impl Eye {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			hp: 20,
			hitbox: Rect::new(pos.x, pos.y, 60., 44.),
			image: assets.images.get("gobloron_eye").expect("could not load gobloron_eye").clone(),
			damage_image: assets.images.get("gobloron_eye_damage").expect("could not load gobloron_eye_damage").clone(),
			dead: false,
		}
	}
}