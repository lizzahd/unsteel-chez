use macroquad::prelude::*;

#[derive(Debug)]
pub enum EventType {
	Pickup {pos: Vec2},
	Damage {pos: Vec2},
}