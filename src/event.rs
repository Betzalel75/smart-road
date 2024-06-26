pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

use crate::Vehicle;
use crate::Direction;

use sdl2::pixels::Color;

pub struct Simulation {
    pub vehicles: Vec<Vehicle>,
    pub next_id: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            vehicles: Vec::new(),
            next_id: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                self.create_vehicle(Direction::North);
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.create_vehicle(Direction::South);
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                self.create_vehicle(Direction::West);
            }
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.create_vehicle(Direction::East);
            }
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.print_statistics();
                std::process::exit(0);
            }
            _ => {}
        }
    }

    pub fn create_vehicle(&mut self, direction: Direction) {
        let position = match direction {
            Direction::North => (400.0, 800.0),
            Direction::South => (400.0, 0.0),
            Direction::East  => (0.0, 400.0),
            Direction::West  => (800.0, 400.0),
            _ => (0.0, 0.0),
        };
        let vehicle = Vehicle::new(self.next_id, position, direction);
        self.vehicles.push(vehicle);
        self.next_id += 1;
    }

    pub fn update(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.update_position();
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        for vehicle in &self.vehicles {
            vehicle.draw(canvas);
        }
        canvas.present();
    }

    pub fn print_statistics(&self) {
        println!("Nombre de véhicules: {}", self.vehicles.len());
        if let Some(max_velocity) = self.vehicles.iter().max_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap()) {
            println!("Vitesse maximale: {}", max_velocity.velocity);
        }
        if let Some(min_velocity) = self.vehicles.iter().min_by(|a, b| a.velocity.partial_cmp(&b.velocity).unwrap()) {
            println!("Vitesse minimale: {}", min_velocity.velocity);
        }
        if let Some(max_time) = self.vehicles.iter().max_by(|a, b| a.time_in_intersection.partial_cmp(&b.time_in_intersection).unwrap()) {
            println!("Temps maximal dans l'intersection: {}", max_time.time_in_intersection);
        }
        if let Some(min_time) = self.vehicles.iter().min_by(|a, b| a.time_in_intersection.partial_cmp(&b.time_in_intersection).unwrap()) {
            println!("Temps minimal dans l'intersection: {}", min_time.time_in_intersection);
        }
    }
}
