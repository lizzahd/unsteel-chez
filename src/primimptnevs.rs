use macroquad::prelude::*;

pub const GRAVITY: f32 = 0.4;

pub struct Collision {
	pub rect_hitboxes: Vec<Rect>, // only for horizontal and up
	pub platforms: Vec<Rect>, // only for down, they are one way
}

pub struct MovementSystem {
	pub pos: Vec2,
	pub vel: Vec2,
	pub grounded: bool,
	pub can_move_left: bool,
	pub can_move_right: bool,
	pub move_speed: f32,
    pub move_acc: f32,
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

	pub fn update(&mut self, collision: &Collision) {
		self.grounded = false;
        for platform in &collision.platforms {
        	if self.vel.y > 0. {
        		if Rect::new(self.pos.x, self.pos.y + self.vel.y, self.hitbox.w, self.hitbox.h).overlaps(platform) {
        			self.vel.y = 0.;
        			self.grounded = true;
        			break;
        		}
        	}
        }

        self.can_move_left = true;
        self.can_move_right = true;
        for hitbox in &collision.rect_hitboxes {
        	if Rect::new(self.pos.x + self.vel.x, self.pos.y, self.hitbox.w, self.hitbox.h).overlaps(hitbox) {
        		if hitbox.y > self.pos.y + self.hitbox.h {
        			continue;
        		}

    			if hitbox.left() <= self.pos.x && self.pos.x + self.hitbox.w <= hitbox.right() {
	        		self.vel.y = 0.;
	        		self.pos.y = hitbox.bottom() + 1.;
	        		continue;
	        	}

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
	}
}