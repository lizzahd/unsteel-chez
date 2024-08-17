use macroquad::prelude::*;

#[derive(Debug)]
pub enum EventType {
	Pickup {pos: Vec2}, // BUG: Collecting more than one item at once causes random entities to die
	Damage {pos: Vec2},
	SpawnFart {pos: Vec2, d: f32, ivel: f32},
	KillPlayer,
}