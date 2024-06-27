use crate::Direction;
use crate::DirectionPath;
use crate::Position;
use crate::Vehicle;
pub use rand::Rng;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
use sdl2::messagebox::*;
use sdl2::render::*;
use std::rc::Rc;
pub use std::time::{Duration, Instant};

pub struct Simulation<'a> {
    pub vehicles: Vec<Vehicle<'a>>,
    pub next_id: u32,
    pub last_r_press: Instant,
    pub last_u_press: Instant,
    pub last_d_press: Instant,
    pub last_l_press: Instant,
    pub last_rt_press: Instant,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Simulation {
            vehicles: Vec::new(),
            next_id: 0,
            last_u_press: Instant::now(),
            last_d_press: Instant::now(),
            last_l_press: Instant::now(),
            last_r_press: Instant::now(),
            last_rt_press: Instant::now(),
        }
    }

    pub fn handle_event(&mut self, event: &Event, texture: Rc<Texture<'a>>) {
        let mut rng = rand::thread_rng();
        let path = match rng.gen_range(0..=2) {
            0 => DirectionPath::GoStraight,
            1 => DirectionPath::TurnLeft,
            2 => DirectionPath::TurnRight,
            _ => DirectionPath::GoStraight, // Par défaut
        };
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if self.last_u_press.elapsed() >= Duration::from_secs(2) {
                    self.create_vehicle(Direction::North(path), texture, path);
                    self.last_u_press = Instant::now()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if self.last_d_press.elapsed() >= Duration::from_secs(2) {
                    self.create_vehicle(Direction::South(path), texture, path);
                    self.last_d_press = Instant::now()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                if self.last_l_press.elapsed() >= Duration::from_secs(2) {
                    self.create_vehicle(Direction::West(path), texture, path);
                    self.last_l_press = Instant::now()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                if self.last_rt_press.elapsed() >= Duration::from_secs(2) {
                    self.create_vehicle(Direction::East(path), texture, path);
                    self.last_rt_press = Instant::now()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                // self.print_statistics();
                std::process::exit(0);
            }
            _ => {}
        }
    }

    pub fn create_vehicle(
        &mut self,
        direction: Direction,
        texture: Rc<Texture<'a>>,
        _path: DirectionPath,
    ) {
        let position = match direction {
            Direction::North(path) => match path {
                DirectionPath::TurnLeft => Position { x: 400.0, y: 800.0 },
                DirectionPath::TurnRight => Position { x: 486.0, y: 800.0 },
                _ => Position { x: 443.0, y: 800.0 },
            }, // ↑
            Direction::South(path) => match path {
                DirectionPath::TurnLeft => Position {
                    x: 350.0,
                    y: -110.0,
                },
                DirectionPath::TurnRight => Position {
                    x: 270.0,
                    y: -110.0,
                },
                _ => Position {
                    x: 310.0,
                    y: -110.0,
                },
            }, // ↓
            Direction::East(path) => match path {
                DirectionPath::TurnLeft => Position { x: 0.0, y: 378.0 },
                DirectionPath::TurnRight => Position { x: 0.0, y: 460.0 },
                _ => Position { x: 0.0, y: 422.0 },
            }, // →
            Direction::West(path) => match path {
                DirectionPath::TurnLeft => Position { x: 800.0, y: 330.0 },
                DirectionPath::TurnRight => Position { x: 800.0, y: 250.0 },
                _ => Position { x: 800.0, y: 290.0 },
            }, // ←
        };
        // Utilisation de filter pour vérifier la distance
        let close_vehicles: Vec<&Vehicle> = self
            .vehicles
            .iter()
            .filter(|vehicle| {
                vehicle.direction == direction && vehicle.position.distance(&position) <= 130.0
            })
            .collect();

        if close_vehicles.is_empty() {
            let vehicle = Vehicle::new(self.next_id, position, direction, texture);
            self.vehicles.push(vehicle);
            self.next_id += 1;
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.vehicles.len() {
            self.vehicles.retain(|cars| !cars.finish);
            let (left, right) = self.vehicles.split_at_mut(i);
            let vehicle = &mut right[0];
            vehicle.avoid_collision(left);
            vehicle.update_position();
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        background_texture: &sdl2::render::Texture,
    ) {
        canvas.clear();
        canvas
            .copy(&background_texture, None, None)
            .expect("Failed to copy background texture");
        for vehicle in &self.vehicles {
            vehicle.draw(canvas);
        }
        canvas.present();
    }

    pub fn print_statistics(&self) {
        let mut statistics = String::new();
        statistics.push_str(&format!("Nombre de véhicules: {}\n", self.vehicles.len()));
        if let Some(max_velocity) = self
            .vehicles
            .iter()
            .max_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap())
        {
            statistics.push_str(&format!("Vitesse maximale: {}\n", max_velocity.velocity));
        }
        if let Some(min_velocity) = self
            .vehicles
            .iter()
            .min_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap())
        {
            statistics.push_str(&format!("Vitesse minimale: {}\n", min_velocity.velocity));
        }
        if let Some(max_time) = self.vehicles.iter().max_by(|a, b| {
            a.time_in_intersection
                .partial_cmp(&b.time_in_intersection)
                .unwrap()
        }) {
            statistics.push_str(&format!(
                "Temps maximal dans l'intersection: {}\n",
                max_time.time_in_intersection
            ));
        }
        if let Some(min_time) = self.vehicles.iter().min_by(|a, b| {
            a.time_in_intersection
                .partial_cmp(&b.time_in_intersection)
                .unwrap()
        }) {
            statistics.push_str(&format!(
                "Temps minimal dans l'intersection: {}\n",
                min_time.time_in_intersection
            ));
        }

        show_simple_message_box(
            MessageBoxFlag::INFORMATION,
            "!!STATISTICS!!",
            &statistics,
            None,
        )
        .unwrap();
    }
}
