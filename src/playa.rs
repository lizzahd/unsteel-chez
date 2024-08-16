// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::assets::*;
use crate::primimptnevs::*;
use crate::level::*;
use crate::map_edit::*;
use crate::event::*;

#[derive(Clone)]
pub struct Player {
	movement_system: MovementSystem,
	current_image: Texture2D,
	dead: bool,
}

impl Player {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			movement_system: MovementSystem::new(pos, 1., 0.5, -15., Rect::new(0., 0., 100., 100.)),
			current_image: assets.images.get("assu-chan-alpha").unwrap().clone(),
			dead: false,
		}
	}

	fn get_center(&self) -> Vec2 {
		self.movement_system.get_center()
	}
}

impl Entity for Player {
	fn draw(&self, level: &Level) {
		draw_texture(&self.current_image, self.movement_system.pos.x + level.x, self.movement_system.pos.y, WHITE);
	}

	fn update(&mut self, level: &Level) {
        if self.movement_system.grounded {
        	self.movement_system.vel.y = 0.;
        	if is_key_down(KeyCode::Space) {
        		self.movement_system.vel.y = self.movement_system.jump_acc;
        	}
        } else {
        	self.movement_system.vel.y += GRAVITY;
        }

        if is_key_down(KeyCode::A) && self.movement_system.can_move_left {
            self.movement_system.pos.x -= self.movement_system.move_speed;
            self.movement_system.vel.x -= self.movement_system.move_acc;
        }
        if is_key_down(KeyCode::D) && self.movement_system.can_move_right {
            self.movement_system.pos.x += self.movement_system.move_speed;
            self.movement_system.vel.x += self.movement_system.move_acc;
        }

        self.movement_system.update(level)
	}

	fn get_hitbox(&self) -> Rect {
		self.movement_system.hitbox
	}

	fn get_pos(&self) -> Vec2 {
		self.movement_system.pos
	}

	fn get_type(&self) -> Option<PlaceMode> {
		Some(PlaceMode::SpawnPlayer)
	}

	fn box_clone(&self) -> Box<dyn Entity> {
		Box::new(self.clone())
	}

	// this will return an event if anything happens, which will be appended to an events vector that will be looped through post-update
	fn give_data(&self, _level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType> {
		for entity in entities.iter() {
			println!("{:?}", entity.get_type());
			if let Some(t) = entity.get_type() {
				match t {
					PlaceMode::Dawn => {
						if entity.get_hitbox().overlaps(&Rect::new(self.get_pos().x, self.get_pos().y, self.get_hitbox().w, self.get_hitbox().h)) {
							return Some(EventType::Pickup {pos: self.get_center()})
						}
					},
					PlaceMode::SpawnGoblin => {
					},
					PlaceMode::SpawnPlayer => {
						continue;
					},
					_ => {
					}
				}
			}
		}

		None
	}

	fn give_event(&mut self, _event: &EventType) {

	}

	fn get_dead(&self) -> bool {
		self.dead
	}
}