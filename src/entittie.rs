pub trait Entity {
    fn draw(&self);
    fn update(&mut self);
}