// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::assets::*;

pub struct Player {
	pos: Vec2,
	vel: Vec2,
	hp: i32,
	current_image: Texture2D,
}

impl Player {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			pos,
			vel: vec2(0., 0.),
			hp: 5,
			current_image: assets.images.get("assu-chan-alpha").unwrap().clone(),
		}
	}
}

impl Entity for Player {
	fn draw(&self) {
		draw_texture(&self.current_image, self.pos.x, self.pos.y, WHITE);
	}

	fn update(&mut self) {

	}
}