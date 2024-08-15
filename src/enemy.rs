use macroquad::prelude::*;

use crate::primimptnevs::*;
use crate::assets::*;
use crate::entittie::*;

pub struct Enemy {
	movement_system: MovementSystem,
	hp: i32,
	current_image: Texture2D,
}

impl Enemy {
    pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
    	Self {
			movement_system: MovementSystem::new(pos, 0.9, 0.4, -10., Rect::new(0., 0., 100., 116.)),
			hp: 3,
			current_image: assets.images.get("gorblin").unwrap().clone(),
    	}
    }
}

impl Entity for Enemy {
	fn update(&mut self, collision: &Collision) {
		if self.movement_system.grounded {
        	self.movement_system.vel.y = 0.;
        } else {
        	self.movement_system.vel.y += GRAVITY;
        }

        self.movement_system.update(collision);
	}

	fn draw(&self) {
		draw_texture(&self.current_image, self.movement_system.pos.x, self.movement_system.pos.y, WHITE);
	}
}