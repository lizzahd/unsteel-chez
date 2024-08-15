use macroquad::prelude::*;

use crate::primimptnevs::*;

pub trait Entity {
    fn draw(&self);
    fn update(&mut self, hitboxes: &Collision);
}