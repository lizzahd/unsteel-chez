// touch it here
use macroquad::prelude::*;

use crate::entittie::*;
use crate::level::*;
use crate::map_edit::*;
use crate::assets::*;
use crate::event::*;

// dish soap hehehehhehe
#[derive(Clone)]
pub struct Dawn {
	// time variable for animation stuff
	anim_t: f32,
	pos: Vec2,
	current_image: Texture2D,
	hitbox: Rect,
	// also means not_active in the case of soap
	dead: bool,
}

impl Dawn {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			anim_t: 0.,
			pos,
			current_image: assets.images.get("dawn").unwrap().clone(),
			hitbox: Rect::new(pos.x, pos.y, 50., 109.),
			dead: false,
		}
	}
}

impl Entity for Dawn {
	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.pos.x + level.x, self.pos.y + ((self.anim_t / 30.).sin() * 16.), WHITE);
	}

	fn update(&mut self, _level: &Level) -> Option<EventType> {
		// increase time variable
		self.anim_t += 1.;

		None
	}

	fn get_hitbox(&self) -> Rect {
		self.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::Dawn)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	fn give_data(&self, _level: &Level, _entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		None
	}

	// check if the player is touchin it
	fn give_event(&mut self, event: &EventType) {
		match event {
			EventType::Pickup{pos} => {
				if self.hitbox.contains(*pos) {
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