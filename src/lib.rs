pub mod event;
pub mod car;

pub use std::rc::Rc;

pub use rand::Rng;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Vehicle<'a>{
    pub id: u32,
    pub position: Position,
    pub velocity: f32,
    pub direction: Direction,
    pub time_in_intersection: f32,
    texture: Rc<Texture<'a>>,
}

pub enum Direction {
    North,
    South,
    East,
    West,
    TurnLeft,
    TurnRight,
    GoStraight,
}

impl<'a> Vehicle<'a> {
    pub fn new(id: u32, position: Position, direction: Direction, texture: Rc<Texture<'a>>) -> Self {
        Vehicle {
            id,
            position,
            velocity: 1.0,
            direction,
            time_in_intersection: 0.0,
            texture
        }
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::North => self.position.y -= self.velocity,
            Direction::South => self.position.y += self.velocity,
            Direction::East  => self.position.x += self.velocity,
            Direction::West  => self.position.x -= self.velocity,
            _ => (),
        }
        self.time_in_intersection += 1.0;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, 50, 100);
        canvas.copy(&self.texture, None, rect).unwrap();
    }
}
