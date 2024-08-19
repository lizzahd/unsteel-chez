use macroquad::prelude::*;

use crate::level::*;
use crate::entittie::*;
use crate::event::*;
use crate::map_edit::*;

pub const GRAVITY: f32 = 0.4;

pub fn dist(pos_1: &Vec2, pos_2: &Vec2) -> f32 {
	((pos_2.x - pos_1.x).powf(2.) + (pos_2.y - pos_1.y).powf(2.)).sqrt()
}

#[derive(Clone)]
pub struct MovementSystem {
	pub pos: Vec2,
	pub vel: Vec2,
	pub grounded: bool,
	pub can_move_left: bool,
	pub can_move_right: bool,
	pub move_speed: f32,
	// makes stuff more smooth
    pub move_acc: f32,
    // slows the player down over time, instead of stopping suddenly
    pub move_acc_dampener: f32,
    pub jump_acc: f32,
    pub hitbox: Rect,
}

impl MovementSystem {
	pub fn new(pos: Vec2, move_speed: f32, move_acc: f32, jump_acc: f32, hitbox: Rect) -> Self {
		Self {
			pos,
			vel: vec2(0., 0.),
			grounded: false,
			can_move_left: true,
			can_move_right: true,
			move_speed,
		    move_acc,
		    move_acc_dampener: 1.1,
		    jump_acc,
		    hitbox,
		}
	}

	pub fn update(&mut self, level: &Level) {
		// false is the default for grounded
		self.grounded = false;
		// check for a floor only below the player and stops fall, but does not snap
        for platform in &level.collision.platforms {
        	if self.vel.y > 0. && self.pos.y + self.hitbox.h <= platform.y {
        		if Rect::new(self.pos.x + 1., self.pos.y + self.vel.y, self.hitbox.w - 2., self.hitbox.h).overlaps(platform) {
        			self.vel.y = 0.;
        			self.grounded = true;
        			break;
        		}
        	}
        }

        self.can_move_left = true;
        self.can_move_right = true;
        for hitbox in &level.collision.rect_hitboxes {
        	if Rect::new(self.pos.x + self.vel.x, self.pos.y, self.hitbox.w, self.hitbox.h).overlaps(hitbox) {
        		// if it is too far below it gets skipped
        		if hitbox.y > self.pos.y + self.hitbox.h {
        			continue;
        		}

        		// get above collision and snap accordingly
    			if hitbox.left() <= self.pos.x && self.pos.x + self.hitbox.w <= hitbox.right() {
	        		self.vel.y = 0.;
	        		self.pos.y = hitbox.bottom() + 1.;
	        		continue;
	        	}

	        	// get left and right collision and snap accordingly
    			if self.vel.x > 0. {
    				self.pos.x = hitbox.left() - self.hitbox.w;
	    			self.can_move_right = false;
	        	} else if self.vel.x < 0. {
	        		self.pos.x = hitbox.right();
	        		self.can_move_left = false;
	        	}
    			self.vel.x = 0.;
    		}
        }

        self.pos += self.vel;
        self.vel.x /= self.move_acc_dampener;

        // updates hitbox
        self.hitbox.x = self.pos.x;
        self.hitbox.y = self.pos.y;
	}

	pub fn get_center(&self) -> Vec2 {
		// center of entity
		self.pos + vec2(self.hitbox.w / 2., self.hitbox.h / 2.)
	}
}