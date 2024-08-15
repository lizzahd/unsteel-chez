// touch it here
use macroquad::prelude::*;

use crate::entittie::*;
use crate::level::*;
use crate::map_edit::*;
use crate::assets::*;

#[derive(Clone)]
pub struct Dawn {
	anim_t: f32,
	pos: Vec2,
	current_image: Texture2D,
	hitbox: Rect,
}

impl Dawn {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			anim_t: 0.,
			pos,
			current_image: assets.images.get("dawn").unwrap().clone(),
			hitbox: Rect::new(pos.x, pos.y, 50., 109.)
		}
	}
}

impl Entity for Dawn {
	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.pos.x + level.x, self.pos.y + ((self.anim_t / 30.).sin() * 16.), WHITE);
	}

	fn update(&mut self, _level: &Level) {
		self.anim_t += 1.;
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
}