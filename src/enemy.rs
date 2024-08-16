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