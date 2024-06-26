use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Car<'a> {
    texture: Texture<'a>,
    pub x: i32,
    pub y: i32,
    pub direction: DirectionCar,
}

pub enum DirectionCar {
    Left,
    Right,
    Up,
    Down,
}

impl<'a> Car<'a> {
    pub fn new(texture: Texture<'a>, x: i32, y: i32, direction: DirectionCar) -> Car<'a> {
        Car { texture, x, y, direction }
    }

    pub fn update(&mut self) {
        match self.direction {
            DirectionCar::Left => self.x -= 2,
            DirectionCar::Right => self.x += 2,
            DirectionCar::Up => self.y -= 2,
            DirectionCar::Down => self.y += 2,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(self.x, self.y, 50, 100);
        canvas.copy(&self.texture, None, rect).unwrap();
    }
}
