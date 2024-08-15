use macroquad::prelude::*;

use crate::primimptnevs::*;
use crate::level::*;

pub trait Entity {
    fn draw(&self, level: &Level);
    fn update(&mut self, level: &Level);
    fn get_hitbox(&self) -> Rect;
    fn get_pos(&self) -> Vec2;
}