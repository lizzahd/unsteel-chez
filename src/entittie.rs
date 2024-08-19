use macroquad::prelude::*;

use crate::level::*;
use crate::event::*;
use crate::map_edit::*;

pub trait Entity {
    // happens every tick
    fn update(&mut self, level: &Level) -> Option<EventType>;
    // lets Entity interact with events
    fn give_event(&mut self, event: &EventType);

    // getters
    fn get_hitbox(&self) -> Rect;
    fn get_pos(&self) -> Vec2;
    fn get_type(&self) -> Option<PlaceMode>;
    fn get_dead(&self) -> bool;
    fn get_hp(&self) -> i32;

    // happens every tick, after update
    fn draw(&self, level: &Level);
    // let Entity see other entities
    fn give_data(&self, level: &Level, entities: &Vec<Box<dyn Entity>>) -> Option<EventType>;
    
    // for cloning stuff
    fn box_clone(&self) -> Box<dyn Entity>;
}