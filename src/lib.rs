pub mod event;

pub use std::rc::Rc;

pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
use sdl2::render::Texture;
pub use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt() as f64
    }
}

#[derive(Clone)]
pub struct Vehicle<'a> {
    pub id: u32,
    pub position: Position,
    pub velocity: f32,
    pub direction: Direction,
    pub time_in_intersection: f32,
    texture: Rc<Texture<'a>>,
    angle: f64, // Angle de rotation en degrés
    check_turn: bool,
    finish: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North(DirectionPath),
    South(DirectionPath),
    East(DirectionPath),
    West(DirectionPath),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectionPath {
    TurnLeft,
    TurnRight,
    GoStraight,
}

impl<'a> Vehicle<'a> {
    pub fn new(
        id: u32,
        position: Position,
        direction: Direction,
        texture: Rc<Texture<'a>>
    ) -> Self {
        let angle = match direction {
            Direction::West(_) => 270.0,
            Direction::East(_) => 90.0,
            Direction::North(_) => 0.0,
            Direction::South(_) => 180.0,
        };
        let mut rng = rand::thread_rng();
        let velocity = match rng.gen_range(3..=7) {
            3 => 3.0,
            5 => 5.0,
            7 => 7.0,
            _ => 4.0, // Par défaut
        };
        Vehicle {
            id,
            position,
            velocity,
            direction,
            time_in_intersection: 0.0,
            texture,
            angle,
            check_turn: false,
            finish: false,
        }
    }
   
    pub fn update_position(&mut self) {
        match self.direction {
            Direction::North(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        self.position.y -= self.velocity;
                    }
                    DirectionPath::TurnLeft => {
                        if !self.check_turn {
                            self.position.y -= self.velocity;
                        } else {
                            self.position.x -= self.velocity; // Ou autre logique
                        }
                        if self.position.y < 337.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle -= 90.0;
                        }
                    }
                    DirectionPath::TurnRight => {
                        if !self.check_turn {
                            self.position.y -= self.velocity;
                        } else {
                            self.position.x += self.velocity; // Ou autre logique
                        }

                        if self.position.y <= 463.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle += 90.0;
                        }
                    }
                }
            }
            Direction::South(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        self.position.y += self.velocity;
                    }
                    DirectionPath::TurnLeft => {
                        if !self.check_turn {
                            self.position.y += self.velocity;
                        } else {
                            self.position.x += self.velocity;
                        }

                        if self.position.y > 379.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle -= 90.0;
                        }
                    }
                    DirectionPath::TurnRight => {
                        if !self.check_turn {
                            self.position.y += self.velocity;
                        } else {
                            self.position.x -= self.velocity; // Ou autre logique
                        }

                        if self.position.y > 248.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle += 90.0;
                        }
                    }
                }
            }
            Direction::East(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        self.position.x += self.velocity;
                    }
                    DirectionPath::TurnLeft => {
                        if !self.check_turn {
                            self.position.x += self.velocity;
                        } else {
                            self.position.y -= self.velocity;
                        }

                        if self.position.x >= 400.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle -= 90.0;
                        }
                    }
                    DirectionPath::TurnRight => {
                        if !self.check_turn {
                            self.position.x += self.velocity;
                        } else {
                            self.position.y += self.velocity;
                        }

                        if self.position.x >= 270.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle += 90.0;
                        }
                    }
                }
            }
            Direction::West(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        self.position.x -= self.velocity;
                    }
                    DirectionPath::TurnLeft => {
                        if !self.check_turn {
                            self.position.x -= self.velocity;
                        } else {
                            self.position.y += self.velocity;
                        }

                        if self.position.x <= 350.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle -= 90.0;
                        }
                    }
                    DirectionPath::TurnRight => {
                        if !self.check_turn {
                            self.position.x -= self.velocity;
                        } else {
                            self.position.y -= self.velocity;
                        }

                        if self.position.x <= 486.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle += 90.0;
                        }
                    }
                }
            }
        }
        self.time_in_intersection += 1.0;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, 50, 100);
        // canvas.copy(&self.texture, None, rect).unwrap();
        canvas.copy_ex(&self.texture, None, Some(rect), self.angle, None, false, false).unwrap();
    }

    pub fn avoid_collision(&mut self, other_vehicles: &mut [Vehicle<'a>]) {
        let safe_distance = 50.0;
        for other_vehicle in other_vehicles.iter_mut() {
            let distance = self.position.distance(&other_vehicle.position);
            if distance < safe_distance + 80.0 {
                if self.direction == other_vehicle.direction {
                    if self.position.y > other_vehicle.position.y{
                        other_vehicle.velocity = self.velocity;
                    }else{
                        self.velocity = other_vehicle.velocity;
                    }
                }
            }
        }
    }
}
