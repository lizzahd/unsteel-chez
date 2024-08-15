use macroquad::prelude::*;

use crate::primimptnevs::*;
use crate::level::*;
use crate::map_edit::*;

pub trait Entity {
    fn draw(&self, level: &Level);
    fn update(&mut self, level: &Level);
    fn get_hitbox(&self) -> Rect;
    fn get_pos(&self) -> Vec2;
    fn get_type(&self) -> Option<PlaceMode>;
    fn box_clone(&self) -> Box<dyn Entity>;
}