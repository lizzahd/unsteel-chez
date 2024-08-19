use macroquad::prelude::*;

#[derive(Debug)]
pub enum EventType {
	Pickup {pos: Vec2}, // BUG: Collecting more than one item at once causes random entities to die
	Damage {rect: Rect},
	SpawnFart {rect: Rect, d: f32, ivel: f32},
	KillPlayer,
	Laser {start_pos: Vec2, angle: f32, speed: f32, distance: f32, duration: i32},
	PlayerSpotted {pos: Vec2},
	SpawnGoblin {pos: Vec2},
}