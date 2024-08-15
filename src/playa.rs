// burt ass
use macroquad::prelude::*;

use crate::entittie::*;
use crate::assets::*;
use crate::primimptnevs::*;

pub struct Player {
	pos: Vec2,
	vel: Vec2,
	hp: i32,
	grounded: bool,
	can_move_left: bool,
	can_move_right: bool,
	current_image: Texture2D,
}

impl Player {
	const MOVE_SPEED: f32 = 1.0;
    const MOVE_ACC: f32 = 0.5;
    const MOVE_ACC_DAMPENER: f32 = 1.1;
    const JUMP_ACC: f32 = -15.;

    const WIDTH: f32 = 100.;
    const HEIGHT: f32 = 100.;
    const H_WIDTH: f32 = Self::WIDTH / 2.;
    const H_HEIGHT: f32 = Self::HEIGHT / 2.;

	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			pos,
			vel: vec2(0., 0.),
			hp: 5,
			grounded: false,
			can_move_left: true,
			can_move_right: true,
			current_image: assets.images.get("assu-chan-alpha").unwrap().clone(),
		}
	}
}

impl Entity for Player {
	fn draw(&self) {
		draw_texture(&self.current_image, self.pos.x, self.pos.y, WHITE);
	}

	fn update(&mut self, collision: &Collision) {        
        if self.grounded {
        	self.vel.y = 0.;
        	if is_key_down(KeyCode::Space) {
        		self.vel.y = Self::JUMP_ACC;
        	}
        } else {
        	self.vel.y += GRAVITY;
        }

        if is_key_down(KeyCode::A) && self.can_move_left {
            self.pos.x -= Self::MOVE_SPEED;
            self.vel.x -= Self::MOVE_ACC;
        }
        if is_key_down(KeyCode::D) && self.can_move_right {
            self.pos.x += Self::MOVE_SPEED;
            self.vel.x += Self::MOVE_ACC;
        }

        self.grounded = false;
        for platform in &collision.platforms {
        	if self.vel.y > 0. {
        		if Rect::new(self.pos.x, self.pos.y + self.vel.y, Self::WIDTH, Self::HEIGHT).overlaps(platform) {
        			self.vel.y = 0.;
        			self.grounded = true;
        			break;
        		}
        	}
        }

        self.can_move_left = true;
        self.can_move_right = true;
        for hitbox in &collision.rect_hitboxes {
        	if Rect::new(self.pos.x + self.vel.x, self.pos.y, Self::WIDTH, Self::HEIGHT).overlaps(hitbox) {
        		if hitbox.y > self.pos.y + Self::HEIGHT {
        			continue;
        		}

    			if hitbox.left() <= self.pos.x && self.pos.x + Self::WIDTH <= hitbox.right() {
	        		self.vel.y = 0.;
	        		self.pos.y = hitbox.bottom() + 1.;
	        		continue;
	        	}

    			if self.vel.x > 0. {
    				self.pos.x = hitbox.left() - Self::WIDTH;
	    			self.can_move_right = false;
	        	} else if self.vel.x < 0. {
	        		self.pos.x = hitbox.right();
	        		self.can_move_left = false;
	        	}
    			self.vel.x = 0.;
    		}
        }

        self.pos += self.vel;
        self.vel.x /= Self::MOVE_ACC_DAMPENER;
	}
}