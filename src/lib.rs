pub mod event;
pub mod car;

pub use rand::Rng;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;

pub struct Vehicle {
    pub id: u32,
    pub position: (f32, f32),
    pub velocity: f32,
    pub direction: Direction,
    pub time_in_intersection: f32,
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

impl Vehicle {
    pub fn new(id: u32, position: (f32, f32), direction: Direction) -> Self {
        Vehicle {
            id,
            position,
            velocity: 1.0,
            direction,
            time_in_intersection: 0.0,
        }
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::North => self.position.1 -= self.velocity,
            Direction::South => self.position.1 += self.velocity,
            Direction::East  => self.position.0 += self.velocity,
            Direction::West  => self.position.0 -= self.velocity,
            _ => (),
        }
        self.time_in_intersection += 1.0;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = match self.direction {
            Direction::North | Direction::South => Color::RED,
            Direction::East | Direction::West => Color::BLUE,
            _ => Color::GREEN,
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(self.position.0 as i32, self.position.1 as i32, 50, 50)).unwrap();
    }
}
