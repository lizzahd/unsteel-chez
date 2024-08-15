use macroquad::prelude::*;

use crate::assets::*;

pub struct Collision {
	pub rect_hitboxes: Vec<Rect>, // only for horizontal and up
	pub platforms: Vec<Rect>, // only for down, they are one way
}

impl Collision {
	fn new() -> Self {
		Self {
			rect_hitboxes: Vec::new(),
			platforms: Vec::new(),
		}
	}
}

pub struct Level {
	pub collision: Collision,
	pub foreground: Texture2D,
	pub background: Texture2D,
	pub x: f32,
}

impl Level {
	pub async fn new(name: &str) -> Self {
		let mut collision = Collision::new();
		let map_assets = AssetManager::new(&format!("maps/{}", name)).await;
		
		Self {
			collision,
			foreground: map_assets.images.get("foreground").unwrap().clone(),
			background: map_assets.images.get("background").unwrap().clone(),
			x: 0.,
		}
	}

	pub fn draw(&self) {
		draw_texture(&self.background, self.x / 2., 0., WHITE);
		draw_texture(&self.foreground, self.x, 0., WHITE);
	}
}