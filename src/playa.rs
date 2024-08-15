// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::assets::*;
use crate::primimptnevs::*;

pub struct Player {
	movement_system: MovementSystem,
	hp: i32,
	current_image: Texture2D,
}

impl Player {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			movement_system: MovementSystem::new(pos, 1., 0.5, -15., Rect::new(0., 0., 100., 100.)),
			hp: 5,
			current_image: assets.images.get("assu-chan-alpha").unwrap().clone(),
		}
	}
}

impl Entity for Player {
	fn draw(&self) {
		draw_texture(&self.current_image, self.movement_system.pos.x, self.movement_system.pos.y, WHITE);
	}

	fn update(&mut self, collision: &Collision) {
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

        self.movement_system.update(collision)
	}
}