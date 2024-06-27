pub mod event;

pub use std::rc::Rc;

pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
use sdl2::render::Texture;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Vehicle<'a>{
    pub id: u32,
    pub position: Position,
    pub velocity: f32,
    pub direction: Direction,
    pub time_in_intersection: f32,
    texture: Rc<Texture<'a>>,
    angle: f64, // Angle de rotation en degrés
}

#[derive(Clone, Copy)]
pub enum Direction {
    North(DirectionPath),
    South(DirectionPath),
    East(DirectionPath),
    West(DirectionPath),
}

#[derive(Clone, Copy)]
pub enum DirectionPath {
    TurnLeft,
    TurnRight,
    GoStraight,
}

impl<'a> Vehicle<'a> {
    pub fn new(id: u32, position: Position, direction: Direction, texture: Rc<Texture<'a>>) -> Self {
        let angle = match direction {
            Direction::West(_) => 270.0,
            Direction::East(_) => 90.0,
            Direction::North(_) => 0.0,
            Direction::South(_) => 180.0,
        };
        Vehicle {
            id,
            position,
            velocity: 1.0,
            direction,
            time_in_intersection: 0.0,
            texture,
            angle
        }
    }

    pub fn update_position(&mut self, vehicles: Vec<Vehicle<'a>>) {
        // match self.direction {
        //     Direction::North(_) => self.position.y -= self.velocity,
        //     Direction::South(_) => self.position.y += self.velocity,
        //     Direction::East(_)  => self.position.x += self.velocity,
        //     Direction::West(_)  => self.position.x -= self.velocity,
        // }
        match self.direction {
            Direction::North(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        for vehicule in vehicles{
                            if vehicule.id != self.id{
                                
                            }
                        }
                        self.position.y -= self.velocity
                    },
                    DirectionPath::TurnLeft => {
                        self.position.y -= self.velocity;
                        self.position.x -= self.velocity; // Ou autre logique
                    }
                    DirectionPath::TurnRight => {
                        self.position.y -= self.velocity;
                        self.position.x += self.velocity; // Ou autre logique
                    }
                }
            }
            Direction::South(path) => {
                match path {
                    DirectionPath::GoStraight => self.position.y += self.velocity,
                    DirectionPath::TurnLeft => {
                        self.position.y += self.velocity;
                        self.position.x += self.velocity; // Ou autre logique
                    }
                    DirectionPath::TurnRight => {
                        self.position.y += self.velocity;
                        self.position.x -= self.velocity; // Ou autre logique
                    }
                }
            }
            Direction::East(path) => {
                match path {
                    DirectionPath::GoStraight => self.position.x += self.velocity,
                    DirectionPath::TurnLeft => {
                        self.position.x += self.velocity;
                        self.position.y -= self.velocity; // Ou autre logique
                    }
                    DirectionPath::TurnRight => {
                        self.position.x += self.velocity;
                        self.position.y += self.velocity; // Ou autre logique
                    }
                }
            }
            Direction::West(path) => {
                match path {
                    DirectionPath::GoStraight => self.position.x -= self.velocity,
                    DirectionPath::TurnLeft => {
                        self.position.x -= self.velocity;
                        self.position.y += self.velocity; // Ou autre logique
                    }
                    DirectionPath::TurnRight => {
                        self.position.x -= self.velocity;
                        self.position.y -= self.velocity; // Ou autre logique
                    }
                }
            }
        }
        self.time_in_intersection += 1.0;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, 50, 100);
        // canvas.copy(&self.texture, None, rect).unwrap();
        canvas.copy_ex(
            &self.texture,
            None,
            Some(rect),
            self.angle,
            None,
            false,
            false
        ).unwrap();
    }
}
