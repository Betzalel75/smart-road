pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
use crate::DirectionPath;
use crate::Position;
use crate::Vehicle;
use crate::Direction;
use std::rc::Rc;
use sdl2::messagebox::*;
use sdl2::render::*;
pub use rand::Rng;
pub struct Simulation<'a> {
    pub vehicles: Vec<Vehicle<'a>>,
    pub next_id: u32,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Simulation {
            vehicles: Vec::new(),
            next_id: 0,
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
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                self.create_vehicle(Direction::North(path), texture, path);
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.create_vehicle(Direction::South(path), texture, path);
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                self.create_vehicle(Direction::West(path), texture, path);
            }
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.create_vehicle(Direction::East(path), texture, path);
            }
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
                // self.print_statistics();
            }
            _ => {}
        }
    }

    pub fn create_vehicle(&mut self, direction: Direction, texture: Rc<Texture<'a>>, _path: DirectionPath) {
        
        let (position, _dir) = match direction {
            Direction::North(path) => (Position{x:443.0, y:800.0}, Direction::North(path)), // ↑
            Direction::South(path) => (Position{x:310.0, y:0.0}, Direction::South(path)), // ↓
            Direction::East(path)  => (Position{x:0.0, y:422.0}, Direction::East(path)), // →
            Direction::West(path)  => (Position{x:800.0, y:290.0}, Direction::West(path)), // ←
            // _ => Position{x:0.0, y:0.0},
        };
        let vehicle = Vehicle::new(self.next_id, position, direction, texture);
        self.vehicles.push(vehicle);
        self.next_id += 1;
    }

    pub fn update(&mut self) {
        for vehicle in &mut self.vehicles.clone() {
            vehicle.update_position(self.vehicles.clone());
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, background_texture: &sdl2::render::Texture) {
        canvas.clear();
        canvas.copy(&background_texture, None, None).expect("Failed to copy background texture");
        for vehicle in &self.vehicles {
            vehicle.draw(canvas);
        }
        canvas.present();
    }

    pub fn print_statistics(&self) {
        let mut statistics = String::new();
        statistics.push_str(&format!("Nombre de véhicules: {}\n", self.vehicles.len()));
        if let Some(max_velocity) = self.vehicles.iter().max_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap()) {
            statistics.push_str(&format!("Vitesse maximale: {}\n", max_velocity.velocity));
        }
        if let Some(min_velocity) = self.vehicles.iter().min_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap()) {
            statistics.push_str(&format!("Vitesse minimale: {}\n", min_velocity.velocity));
        }
        if let Some(max_time) = self.vehicles.iter().max_by(|a, b| a.time_in_intersection.partial_cmp(&b.time_in_intersection).unwrap()) {
            statistics.push_str(&format!("Temps maximal dans l'intersection: {}\n", max_time.time_in_intersection));
        }
        if let Some(min_time) = self.vehicles.iter().min_by(|a, b| a.time_in_intersection.partial_cmp(&b.time_in_intersection).unwrap()) {
            statistics.push_str(&format!("Temps minimal dans l'intersection: {}\n", min_time.time_in_intersection));
        }
        
        show_simple_message_box(
            MessageBoxFlag::INFORMATION,
            "!!STATISTICS!!",
            &statistics,
            None,
        ).unwrap();
    }
}
