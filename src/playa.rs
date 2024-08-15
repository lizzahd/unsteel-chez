// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::utils::*;

poop::oop! (
	public class Player {
		pos: Vec2,
		vel: Vec2,
		hp: i32,
	}

	pub fn new(pos: Vec2) -> Self {
		Self {
			pos,
			vel: vec2(0., 0.),
			hp: 5,
		}
	}
);

impl Entity for Player {
	fn draw(&self) {
		
	}

	fn update(&mut self) {

	}
}