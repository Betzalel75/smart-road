pub mod event;
pub mod utils;
pub use rand::Rng;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::collections::HashMap;
pub use std::rc::Rc;
use std::time::Instant;

const FRONT_RECT_LENGTH: u32 = 160;
pub const SAFE_DISTANCE: f64 = 40.0;
pub const VEHICLE_HEIGHT: u32 = 40;
pub const VEHICLE_WIDTH: u32 = 20;

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
    id: u32,
    position: Position,
    velocity: f32,
    direction: Direction,
    time_in_intersection: f32,
    distance: u32,
    texture: Rc<Texture<'a>>,
    angle: f64,
    check_turn: bool,
    pub finish: bool,
    last_stop: Instant,
    time_in_inter: f32,
    time_in_road: f32,
    is_out: bool,
    sensor: Sensor,
    start_time: Instant,
    close_calls: HashMap<u32, Vec<u32>>,
}

#[derive(Clone, Debug)]
pub struct Sensor {
    pub w: u32,
    pub h: u32,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North(DirectionPath),
    South(DirectionPath),
    East(DirectionPath),
    West(DirectionPath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        texture: Rc<Texture<'a>>,
        path: DirectionPath,
    ) -> Self {
        let angle = match direction {
            Direction::West(_) => 270.0,
            Direction::East(_) => 90.0,
            Direction::North(_) => 0.0,
            Direction::South(_) => 180.0,
        };

        let distance = match path {
            DirectionPath::TurnLeft => 720,
            DirectionPath::TurnRight => 360,
            DirectionPath::GoStraight => 500,
        };

        Vehicle {
            id,
            position,
            velocity: 12.0,
            direction,
            time_in_intersection: 30.0,
            distance,
            texture,
            angle,
            check_turn: false,
            finish: false,
            is_out: false,
            last_stop: Instant::now(),
            time_in_inter: 0.0,
            time_in_road: 0.0,
            sensor: Sensor {
                w: 0,
                h: 0,
                position: Position {
                    x: position.x,
                    y: position.y - (FRONT_RECT_LENGTH as f32),
                },
            },
            start_time: Instant::now(),
            close_calls: HashMap::new(),
        }
    }

    fn update_position(&mut self, nb: &mut u32, cars: Vec<Vehicle<'a>>) {
        match self.direction {
            Direction::North(path) => match path {
                DirectionPath::GoStraight => {
                    if self.position.y <= 219.0 {
                        self.is_out = true;
                        self.velocity = 24.0;
                    }
                    self.position.y -= self.velocity;
                    if self.position.y < -110.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if !self.check_turn {
                        self.position.y -= self.velocity;
                    } else {
                        if self.position.x <= 240.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.x -= self.velocity;
                        if self.position.x < -10.0 {
                            self.finish = true;
                        }
                    }
                    if self.position.y <= 370.0 && !self.check_turn {
                        let (x, y, w, h) = (
                            (((self.position.x - self.velocity)
                                - ((FRONT_RECT_LENGTH as f32) + 10.0))
                                as i32)
                                .abs(),
                            (((self.position.y as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        );
                        self.sensor.w = w;
                        self.sensor.h = h;
                        self.sensor.position.x = x as f32;
                        self.sensor.position.y = y as f32;
                        self.check_turn = true;
                        self.angle -= 90.0;
                    }
                }
                DirectionPath::TurnRight => {
                    if !self.check_turn {
                        self.position.y -= self.velocity;
                    } else {
                        if self.position.x >= 793.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.x += self.velocity;
                        if self.position.x > 820.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.y <= 498.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
            Direction::South(path) => match path {
                DirectionPath::GoStraight => {
                    if self.position.y >= 793.0 {
                        self.is_out = true;
                        self.velocity = 24.0;
                    }
                    self.position.y += self.velocity;
                    if self.position.y > 820.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if !self.check_turn {
                        self.position.y += self.velocity;
                    } else {
                        if self.position.x >= 793.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.x += self.velocity;
                        if self.position.x > 810.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.y >= 400.0 && !self.check_turn {
                        let (x, y, w, h) = (
                            (((self.position.x+self.velocity) as i32) - 5).abs(),
                            (self.position.y as i32) + 40,
                            30,
                            FRONT_RECT_LENGTH,
                        );
                        self.sensor.w = w;
                        self.sensor.h = h;
                        self.sensor.position.x = x as f32;
                        self.sensor.position.y = y as f32;
                        self.check_turn = true;
                        self.angle -= 90.0;
                    }
                }
                DirectionPath::TurnRight => {
                    if !self.check_turn {
                        self.position.y += self.velocity;
                    } else {
                        if self.position.x <= 240.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.x -= self.velocity;
                        if self.position.x < -10.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.y > 270.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
            Direction::East(path) => match path {
                DirectionPath::GoStraight => {
                    if self.position.x >= 793.0 {
                        self.is_out = true;
                        self.velocity = 24.0;
                    }
                    self.position.x += self.velocity;
                    if self.position.x > 820.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if !self.check_turn {
                        self.position.x += self.velocity;
                    } else {
                        if self.position.y <= 219.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.y -= self.velocity;
                        if self.position.y < -110.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.x >= 400.0 && !self.check_turn {
                        let (x, y, w, h) = (
                            (self.position.x + 30.0) as i32,
                            ((((self.position.y - self.velocity) as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        );
                        self.sensor.w = w;
                        self.sensor.h = h;
                        self.sensor.position.x = x as f32;
                        self.sensor.position.y = y as f32;
                        self.check_turn = true;
                        self.angle -= 90.0;
                    }
                }
                DirectionPath::TurnRight => {
                    if !self.check_turn {
                        self.position.x += self.velocity;
                    } else {
                        if self.position.x >= 793.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.y += self.velocity;
                        if self.position.y > 810.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.x >= 275.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
            Direction::West(path) => match path {
                DirectionPath::GoStraight => {
                    if self.position.x <= 240.0 {
                        self.is_out = true;
                        self.velocity = 24.0;
                    }
                    self.position.x -= self.velocity;
                    if self.position.x < -10.0 {
                        self.finish = true;
                    }
                }
                DirectionPath::TurnLeft => {
                    if !self.check_turn {
                        self.position.x -= self.velocity;
                    } else {
                        if self.position.y >= 796.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.y += self.velocity;
                        if self.position.y > 810.0 {
                            self.finish = true;
                        }
                    }
                    if self.position.x <= 375.0 && !self.check_turn {
                        let (x, y, w, h) = (
                            ((self.position.x - ((FRONT_RECT_LENGTH as f32) + 10.0)) as i32).abs(),
                            ((((self.position.y+self.velocity) as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        );
                        self.sensor.w = w;
                        self.sensor.h = h;
                        self.sensor.position.x = x as f32;
                        self.sensor.position.y = y as f32;
                        self.check_turn = true;
                        self.angle -= 90.0;
                    }
                }
                DirectionPath::TurnRight => {
                    if !self.check_turn {
                        self.position.x -= self.velocity;
                    } else {
                        if self.position.y <= 219.0 {
                            self.is_out = true;
                            self.velocity = 24.0;
                        }
                        self.position.y -= self.velocity;
                        if self.position.y < -110.0 {
                            self.finish = true;
                        }
                    }

                    if self.position.x <= 500.0 && !self.check_turn {
                        self.check_turn = true;
                        self.angle += 90.0;
                    }
                }
            },
        }
        self.time_in_road += 1.0;
        if self.in_intersection() {
            self.time_in_inter += 1.0;
        }

        if self.time_in_road == 1.0 {
            self.start_time = Instant::now(); // start decompting
        }

        for car in cars {
            if self.position.distance(&car.position) <= 15.0 && car.id != self.id {
                *nb += 1;
            }
        }
        self.move_sensor();
    }

    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let vehicle_width: u32 = 20;
        let vehicle_height: u32 = 40;

        // Draw the vehicle
        let rect = Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            vehicle_width,
            vehicle_height,
        );

        let check = match self.direction {
            Direction::West(path) => match path {
                DirectionPath::TurnRight => true,
                _ => false,
            },
            Direction::East(path) => match path {
                DirectionPath::TurnRight => true,
                _ => false,
            },
            Direction::South(path) => match path {
                DirectionPath::TurnRight => true,
                _ => false,
            },
            Direction::North(path) => match path {
                DirectionPath::TurnRight => true,
                _ => false,
            },
        };

        // Draw the front rectangle
        if !check {
            let front_rect = Rect::new(
                self.sensor.position.x as i32,
                self.sensor.position.y as i32,
                self.sensor.w,
                self.sensor.h,
            );

            // Activer le mode de blending pour la transparence
            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

            canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 0, 120));
            canvas.fill_rect(front_rect).unwrap();
        }
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

    fn move_sensor(&mut self) {
        let (x, y, w, h) = match self.direction {
            Direction::West(path) => match path {
                DirectionPath::TurnLeft => {
                    if self.position.x <= 375.0 {
                        (
                            (5 - self.position.x as i32).abs(),
                            (self.position.y as i32) + 40,
                            30,
                            FRONT_RECT_LENGTH,
                        )
                    } else {
                        (
                            ((self.position.x - ((FRONT_RECT_LENGTH as f32) + 10.0)) as i32).abs(),
                            (((self.position.y as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        )
                    }
                }

                _ => (
                    (self.position.x - ((FRONT_RECT_LENGTH as f32) + 10.0)) as i32,
                    (((self.position.y as i32) + 10) - 5).abs(),
                    FRONT_RECT_LENGTH,
                    30,
                ),
            },
            Direction::East(path) => match path {
                DirectionPath::TurnLeft => {
                    if self.position.x >= 400.0 {
                        (
                            (self.position.x as i32 - 5).abs(),
                            (self.position.y - (FRONT_RECT_LENGTH as f32)) as i32,
                            30,
                            FRONT_RECT_LENGTH,
                        )
                    } else {
                        (
                            (self.position.x + 30.0) as i32,
                            (((self.position.y as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        )
                    }
                }
                _ => (
                    ((self.position.x + 30.0) as i32),
                    (((self.position.y as i32) + 10) - 5).abs(),
                    FRONT_RECT_LENGTH,
                    30,
                ),
            },
            Direction::South(path) => match path {
                DirectionPath::TurnLeft => {
                    if self.position.y >= 400.0 {
                        (
                            (self.position.x + 30.0) as i32,
                            (((self.position.y as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        )
                    } else {
                        (
                            ((self.position.x as i32) - 5).abs(),
                            (self.position.y as i32) + 40,
                            30,
                            FRONT_RECT_LENGTH,
                        )
                    }
                }
                _ => (
                    ((self.position.x as i32) - 5).abs(),
                    (self.position.y as i32) + 40,
                    30,
                    FRONT_RECT_LENGTH,
                ),
            },
            Direction::North(path) => match path {
                DirectionPath::TurnLeft => {
                    if self.position.y <= 370.0 {
                        (
                            (self.position.x - ((FRONT_RECT_LENGTH as f32) + 10.0)) as i32,
                            (((self.position.y as i32) + 10) - 5).abs(),
                            FRONT_RECT_LENGTH,
                            30,
                        )
                    } else {
                        (
                            ((self.position.x as i32) - 5).abs(),
                            (self.position.y - (FRONT_RECT_LENGTH as f32)) as i32,
                            30,
                            FRONT_RECT_LENGTH,
                        )
                    }
                }
                _ => (
                    ((self.position.x as i32) - 5).abs(),
                    (self.position.y - (FRONT_RECT_LENGTH as f32)) as i32,
                    30,
                    FRONT_RECT_LENGTH,
                ),
            },
        };
        self.sensor.w = w;
        self.sensor.h = h;
        self.sensor.position.x = x as f32;
        self.sensor.position.y = y as f32;
    }

    fn avoid_collision(&mut self, other_vehicles: &mut [&mut Vehicle<'a>]) {
        for other_vehicle in other_vehicles.iter_mut() {
            if self.sensor_detection(other_vehicle) {
                if self.is_not_at_a_safe_distance(other_vehicle) {
                    self.close_calls
                        .entry(self.id)
                        .or_insert(Vec::new())
                        .push(other_vehicle.id);
                }
                if self.direction == other_vehicle.direction {
                    self.velocity = other_vehicle.velocity;
                } else {
                    self.velocity = 0.0
                }
            }
        }
    }

    fn is_not_at_a_safe_distance(&self, other: &Vehicle) -> bool {
        self.position.x < other.position.x + VEHICLE_WIDTH as f32 + (SAFE_DISTANCE as f32)
            && self.position.x + VEHICLE_WIDTH as f32 > other.position.x
            && self.position.y < other.position.y + VEHICLE_HEIGHT as f32 + (SAFE_DISTANCE as f32)
            && self.position.y + VEHICLE_HEIGHT as f32 > other.position.y
    }

    fn in_intersection(&self) -> bool {
        // Vérifier si la voiture est à l'intérieur du carré central
        let mut square_size = 250;
        let central_x = (800 / 2 - square_size / 2 - 4) as f32;
        let central_y = (800 / 2 - square_size / 2) as f32;
        square_size += 12;
        self.position.x < central_x + (square_size as f32)
            && self.position.x + 20.0 > central_x
            && self.position.y < central_y + (square_size as f32)
            && self.position.y + 40.0 > central_y
    }

    fn sensor_detection(&self, other: &Vehicle) -> bool {
        self.sensor.position.x < other.position.x + 20.0
            && self.sensor.position.x + self.sensor.w as f32 > other.position.x
            && self.sensor.position.y < other.position.y + 40.0
            && self.sensor.position.y + self.sensor.h as f32 > other.position.y
    }
}
