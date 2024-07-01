pub mod event;
pub mod utils;
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::Texture;
pub use std::rc::Rc;
pub use std::time::{Duration, Instant};
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;
pub const SAFE_DISTANCE: f64 = 130.0;
pub const VEHICLE_HEIGHT:u32 = 100;
pub const VEHICLE_WIDTH:u32 = 50;
const STOP_DELAY: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Vehicle<'a> {
    pub id: u32,
    pub position: Position,
    pub velocity: f32,
    pub origin_velocity: f32,
    pub direction: Direction,
    path: u16,
    time_in_intersection: f32,
    time_in_road: f32,
    texture: Rc<Texture<'a>>,
    angle: f64, // Angle de rotation en degrés
    check_turn: bool,
    last_stop: Instant,
    has_slowed_down: bool,
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
    fn new(
        id: u32,
        position: Position,
        direction: Direction,
        texture: Rc<Texture<'a>>,
    ) -> Self {
        let (angle, path) = match direction {
            Direction::West(path) => match path {
                DirectionPath::GoStraight => (270.0, 1),
                DirectionPath::TurnLeft => (270.0, 2),
                DirectionPath::TurnRight => (270.0, 3),
            },
            Direction::East(path) => {
                match path {
                    DirectionPath::GoStraight => (90.0, 1),
                    DirectionPath::TurnLeft => (90.0, 2),
                    DirectionPath::TurnRight => (90.0, 3),
                }
            },
            Direction::North(path) => {
                match path {
                    DirectionPath::GoStraight => (0.0, 1),
                    DirectionPath::TurnLeft => (0.0, 2),
                    DirectionPath::TurnRight => (0.0, 3),
                }
            },
            Direction::South(path) => {
                match path {
                    DirectionPath::GoStraight => (180.0, 1),
                    DirectionPath::TurnLeft => (180.0, 2),
                    DirectionPath::TurnRight => (180.0, 3),
                }
            },
        };
        let mut rng = rand::thread_rng();
        let velocity = match rng.gen_range(1..=2) {
            1 => 3.0,
            2 => 9.0,
            _ => 3.0, // Par défaut
        };
        Vehicle {
            id,
            position,
            velocity,
            origin_velocity: velocity,
            direction,
            path,
            time_in_intersection: 0.0,
            time_in_road: 0.0,
            texture,
            angle,
            check_turn: false,
            has_slowed_down: false,
            last_stop: Instant::now(),
            finish: false,
        }
    }

    fn distance_between_vehicle(&self, other: &Vehicle)->bool{
        let vehicle_height:f32 = 50.0;
        let vehicle_width:f32 = 100.0;
        self.position.x < other.position.x + vehicle_height + (SAFE_DISTANCE as f32) &&
        self.position.x + 50.0 > other.position.x &&
        self.position.y < other.position.y + vehicle_width + (SAFE_DISTANCE as f32) &&
        self.position.y + 100.0 > other.position.y
    }

    fn update_position(&mut self) {
        match self.direction {
            Direction::North(path) => {
                match path {
                    DirectionPath::GoStraight => {
                        if self.has_slowed_down
                            && self.last_stop.elapsed() >= STOP_DELAY
                        {
                            self.velocity = self.origin_velocity;
                        }
                        if self.position.y <= 219.0 {
                            self.velocity = 7.0;
                            self.has_slowed_down = false;
                        }
                        self.position.y -= self.velocity;
                        if self.position.y < -110.0 {
                            self.finish = true;
                        }
                    }
                    DirectionPath::TurnLeft => {
                        if self.has_slowed_down
                            && self.last_stop.elapsed() >= STOP_DELAY
                        {
                            self.velocity = self.origin_velocity;
                        }
                        if !self.check_turn {
                            self.position.y -= self.velocity;
                        } else {
                            if self.position.x <= 240.0 {
                                self.velocity = 7.0;
                                self.has_slowed_down = false;
                            }
                            self.position.x -= self.velocity; 
                            if self.position.x < -30.0 {
                                self.finish = true;
                            }
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
                            self.position.x += self.velocity; 
                            if self.position.x > 800.0 {
                                self.finish = true;
                            }
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
                        if self.has_slowed_down
                            && self.last_stop.elapsed() >= STOP_DELAY
                        {
                            self.velocity = self.origin_velocity;
                        }
                        if self.position.y >= 490.0 {
                            self.velocity = 7.0;
                            self.has_slowed_down = false;
                        }
                        self.position.y += self.velocity;
                        if self.position.y > 830.0 {
                            self.finish = true;
                        }
                    }
                    DirectionPath::TurnLeft => {
                        if self.has_slowed_down
                            && self.last_stop.elapsed() >= STOP_DELAY
                        {
                            self.velocity = self.origin_velocity;
                        }
                        if !self.check_turn {
                            self.position.y += self.velocity;
                        } else {
                            if self.position.x >= 480.0 {
                                self.velocity = 7.0;
                                self.has_slowed_down = false;
                            }
                            self.position.x += self.velocity;
                            if self.position.x > 830.0 {
                                self.finish = true;
                            }
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
                            if self.position.x < -10.0 {
                                self.finish = true;
                            }
                        }

                        if self.position.y > 248.0 && !self.check_turn {
                            self.check_turn = true;
                            self.angle += 90.0;
                        }
                    }
                }
            }
            Direction::East(path) => match path {
                DirectionPath::GoStraight => {
                    if self.has_slowed_down && self.last_stop.elapsed() >= STOP_DELAY {
                        self.velocity = self.origin_velocity;
                    }
                    if self.position.x >= 486.0 {
                        self.velocity = 7.0;
                        self.has_slowed_down = false;
                    }
                    self.position.x += self.velocity;
                    if self.position.x > 820.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if self.has_slowed_down && self.last_stop.elapsed() >= STOP_DELAY {
                        self.velocity = self.origin_velocity;
                    }
                    if !self.check_turn {
                        self.position.x += self.velocity;
                    } else {
                        if self.position.y <= 219.0 {
                            self.velocity = 7.0;
                            self.has_slowed_down = false;
                        }
                        self.position.y -= self.velocity;
                        if self.position.y < -110.0 {
                            self.finish = true;
                        }
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
                        if self.position.y > 810.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.x >= 270.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
            Direction::West(path) => match path {
                DirectionPath::GoStraight => {
                    if self.has_slowed_down && self.last_stop.elapsed() >= STOP_DELAY {
                        self.velocity = self.origin_velocity;
                    }
                    if self.position.x <= 240.0 {
                        self.velocity = 7.0;
                        self.has_slowed_down = false;
                    }
                    self.position.x -= self.velocity;
                    if self.position.x < -90.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if self.has_slowed_down && self.last_stop.elapsed() >= STOP_DELAY {
                        self.velocity = self.origin_velocity;
                    }
                    if !self.check_turn {
                        self.position.x -= self.velocity;
                    } else {
                        if self.position.y >= 480.0 {
                            self.velocity = 7.0;
                            self.has_slowed_down = false;
                        }
                        self.position.y += self.velocity;
                        if self.position.y > 810.0 {
                            self.finish = true;
                        }
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
                        if self.position.y < -110.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.x <= 486.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
        }
        self.time_in_road += 1.0;
        if self.in_intersection() {
            self.time_in_intersection += 1.0;
        }
    }

    fn in_intersection(&self) -> bool {
        // Vérifier si la voiture est à l'intérieur du carré central
        let mut square_size = 250;
        let central_x = (((WIDTH / 2) - (square_size / 2)) - 4) as f32;
        let central_y = ((HEIGHT / 2) - (square_size / 2)) as f32;
        square_size += 12;
        self.position.x < central_x + (square_size as f32)
            && self.position.x + 50.0 > central_x
            && self.position.y < central_y + (square_size as f32)
            && self.position.y + 100.0 > central_y
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, VEHICLE_WIDTH, VEHICLE_HEIGHT);
        canvas
            .copy_ex(
                &self.texture,
                None,
                Some(rect),
                self.angle,
                None,
                false,
                false,
            )
            .unwrap();
    }

    fn avoid_collision(&mut self, other_vehicles: &mut [Vehicle<'a>]) {
        for other in other_vehicles.iter_mut() {
            if self.direction == other.direction && self.path == other.path {
                if self.collision_vehicle(other) {
                    if self.time_in_road > other.time_in_road {
                        other.velocity = self.velocity;
                    } else {
                        self.velocity = other.velocity;
                    }
                }
            }else{
                continue;
            }
        }
    }
    fn collision_vehicle(&self, other: &Vehicle)->bool{
        let vehicle_height:f32 = VEHICLE_HEIGHT as f32;
        let vehicle_width:f32 = VEHICLE_WIDTH as f32;
        self.position.x - (SAFE_DISTANCE as f32) < other.position.x + vehicle_height + (SAFE_DISTANCE as f32) &&
        self.position.x + vehicle_width > other.position.x &&
        self.position.y - (SAFE_DISTANCE as f32) < other.position.y + vehicle_width + (SAFE_DISTANCE as f32) &&
        self.position.y + vehicle_height > other.position.y
    }
}
