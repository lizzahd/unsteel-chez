use macroquad::prelude::*;

pub const GRAVITY: f32 = 0.4;

pub struct Collision {
	pub rect_hitboxes: Vec<Rect>, // only for horizontal and up
	pub platforms: Vec<Rect>, // only for down, they are one way
}