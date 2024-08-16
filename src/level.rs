use macroquad::prelude::*;

use crate::assets::*;

#[derive(Clone)]
pub struct Collision {
	pub rect_hitboxes: Vec<Rect>, // only for horizontal and up
	pub platforms: Vec<Rect>, // only for down, they are one way
}

impl Collision {
	pub fn new() -> Self {
		Self {
			rect_hitboxes: Vec::new(),
			platforms: Vec::new(),
		}
	}
}

pub struct Level<'a> {
	pub collision: Collision,
	pub foreground: Texture2D,
	pub background: Texture2D,
	pub x: f32,
	pub name: &'a str,
}

impl<'a> Level<'a> {
	pub async fn new(name: &'a str) -> Self {
		let map_assets = AssetManager::new(&format!("maps/{}", name)).await;
		
		Self {
			collision: Collision::new(),
			foreground: map_assets.images.get("foreground").unwrap().clone(),
			background: map_assets.images.get("background").unwrap().clone(),
			x: 0.,
			name,
		}
	}

	pub fn draw(&self) {
		draw_texture(&self.background, self.x / 2., 0., WHITE);
		draw_texture(&self.foreground, self.x, 0., WHITE);
	}

	pub fn clone(&self) -> Self {
		Self {
			collision: self.collision.clone(),
			name: self.name,
			foreground: self.foreground.clone(),
			background: self.background.clone(),
			x: 0.,
		}
	}
}