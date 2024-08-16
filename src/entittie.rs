use macroquad::prelude::*;

use crate::level::*;
use crate::event::*;
use crate::map_edit::*;

pub trait Entity {
    fn update(&mut self, level: &Level);
    fn give_event(&mut self, event: &EventType);

    fn get_hitbox(&self) -> Rect;
    fn get_pos(&self) -> Vec2;
    fn get_type(&self) -> Option<PlaceMode>;
    fn get_dead(&self) -> bool;

    fn draw(&self, level: &Level);
    fn give_data(&self, level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType>;
    
    fn box_clone(&self) -> Box<dyn Entity>;
}