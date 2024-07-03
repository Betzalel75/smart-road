use crate::utils::func::{bigger, can_collide, smallest};
use crate::Direction;
use crate::DirectionPath;
use crate::Position;
use crate::Vehicle;
pub use rand::Rng;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
use sdl2::messagebox::*;
use sdl2::render::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
pub use std::time::{Duration, Instant};

const INPUT_DELAY: Duration = Duration::from_secs(1);
pub struct Simulation<'a> {
    pub vehicles: Vec<Vehicle<'a>>,
    next_id: u32,
    last_u_press: Instant,
    last_d_press: Instant,
    last_l_press: Instant,
    last_rt_press: Instant,
    last_random_press: Instant,
    nb_collisions: u32,
    min_velocity: HashMap<u32, f32>,
    max_velocity: HashMap<u32, f32>,
    nbr_vehicles: usize,
    max_time: HashMap<u32, f32>,
    min_time: HashMap<u32, f32>,
    close_calls: HashMap<u32, Vec<u32>>,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Simulation {
            vehicles: Vec::new(),
            next_id: 0,
            last_u_press: Instant::now(),
            last_d_press: Instant::now(),
            last_l_press: Instant::now(),
            last_rt_press: Instant::now(),
            last_random_press: Instant::now(),
            nb_collisions: 0,
            min_velocity: HashMap::new(),
            max_velocity: HashMap::new(),
            nbr_vehicles: 0,
            max_time: HashMap::new(),
            min_time: HashMap::new(),
            close_calls: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event, texture: Rc<Texture<'a>>) {
        let mut rng = rand::thread_rng();
        let path = match rng.gen_range(0..=2) {
            0 => DirectionPath::GoStraight,
            1 => DirectionPath::TurnLeft,
            2 => DirectionPath::TurnRight,
            _ => DirectionPath::GoStraight,
        };
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if self.last_u_press.elapsed() >= INPUT_DELAY {
                    self.create_vehicle(Direction::North(path), texture, path);
                    self.last_u_press = Instant::now();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if self.last_d_press.elapsed() >= INPUT_DELAY {
                    self.create_vehicle(Direction::South(path), texture, path);
                    self.last_d_press = Instant::now();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                if self.last_l_press.elapsed() >= INPUT_DELAY {
                    self.create_vehicle(Direction::West(path), texture, path);
                    self.last_l_press = Instant::now();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                if self.last_rt_press.elapsed() >= INPUT_DELAY {
                    self.create_vehicle(Direction::East(path), texture, path);
                    self.last_rt_press = Instant::now();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.print_statistics();
                std::process::exit(0);
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                let mut rnd = rand::thread_rng();
                let direction = match rnd.gen_range(1..=4) {
                    1 => Direction::North(path),
                    2 => Direction::South(path),
                    3 => Direction::East(path),
                    4 => Direction::West(path),
                    _ => Direction::East(path),
                };
                if self.last_random_press.elapsed() >= INPUT_DELAY {
                    self.create_vehicle(direction, texture, path);
                    self.last_random_press = Instant::now();
                }
            }
            Event::Quit { .. } => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    pub fn create_vehicle(
        &mut self,
        direction: Direction,
        texture: Rc<Texture<'a>>,
        path: DirectionPath,
    ) {
        if self.vehicles.len() == 12 {
            return;
        }
        let position = match direction {
            Direction::North(path) => match path {
                DirectionPath::TurnLeft => Position { x: 415.0, y: 800.0 },
                DirectionPath::TurnRight => Position { x: 500.0, y: 800.0 },
                _ => Position { x: 455.0, y: 800.0 },
            }, // ↑
            Direction::South(path) => match path {
                DirectionPath::TurnLeft => Position { x: 365.0, y: -50.0 },
                DirectionPath::TurnRight => Position { x: 285.0, y: -50.0 },
                _ => Position { x: 325.0, y: -50.0 },
            }, // ↓
            Direction::East(path) => match path {
                DirectionPath::TurnLeft => Position { x: 0.0, y: 410.0 },
                DirectionPath::TurnRight => Position { x: 0.0, y: 495.0 },
                _ => Position { x: 0.0, y: 455.0 },
            }, // →
            Direction::West(path) => match path {
                DirectionPath::TurnLeft => Position { x: 800.0, y: 360.0 },
                DirectionPath::TurnRight => Position { x: 800.0, y: 280.0 },
                _ => Position { x: 800.0, y: 320.0 },
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
            let vehicle = Vehicle::new(self.next_id, position, direction, texture, path);
            self.insert_velocity(&vehicle);
            self.vehicles.push(vehicle);
            self.next_id += 1;
            self.nbr_vehicles += 1;
        }
    }

    pub fn update(&mut self, nb: &mut u32) {
        let len = self.vehicles.len();
        for i in 0..len {
            let cars = self.vehicles.clone();
            let mut other_vehicles = Vec::with_capacity(len - 1);
            for j in 0..len {
                if i != j {
                    let vehicle_ref: *mut Vehicle = &mut self.vehicles[j];
                    other_vehicles.push(unsafe { &mut *vehicle_ref });
                }
            }

            let vehicle = &mut self.vehicles[i];
            if vehicle.in_intersection() {
                vehicle.velocity = (vehicle.distance as f32) / vehicle.time_in_intersection;
            }
            if vehicle.velocity == 0.0 && vehicle.last_stop.elapsed() >= Duration::from_millis(2500)
            {
                vehicle.velocity = 12.0;
            }
            
            other_vehicles
                .retain(|car| can_collide(car, vehicle) || car.direction == vehicle.direction);
            
            vehicle.avoid_collision(&mut other_vehicles[..]);
            vehicle.update_position(nb, cars);
        }

        for vehicle in self.vehicles.clone() {
            self.insert_velocity(&vehicle);
            if vehicle.is_out {
                self.insert_time(&vehicle);
            }
            let val = vehicle.close_calls.values();
            if val.len() > 0 {
                self.insert_close_calls(vehicle.close_calls);
            }
        }
        self.nb_collisions = *nb;
    }

    fn insert_close_calls(&mut self, map: HashMap<u32, Vec<u32>>) {
        let mut seen_pairs = HashSet::new();
        
        for (key_map, values_map) in map.iter() {
            for value_map in values_map {
                let pair = (*key_map, *value_map);
                let reverse_pair = (*value_map, *key_map);
                
                if !seen_pairs.contains(&pair) && !seen_pairs.contains(&reverse_pair) {
                    self.close_calls.entry(*key_map).or_insert(Vec::new()).push(*value_map);
                    seen_pairs.insert(pair);
                }
            }
        }
    }

    fn insert_time(&mut self, vehicle: &Vehicle) {
        let time = vehicle.start_time.elapsed().as_secs_f32();
        for key in self.max_time.keys() {
            if *key == vehicle.id {
                return;
            }
        }
        if let Some(min_time) = self.min_time.get(&vehicle.id) {
            if time < *min_time {
                self.min_time.insert(vehicle.id, time);
            }
        } else {
            self.min_time.insert(vehicle.id, time);
        }
        if let Some(max_time) = self.max_time.get(&vehicle.id) {
            if time > *max_time {
                self.max_time.insert(vehicle.id, time);
            }
        } else {
            self.max_time.insert(vehicle.id, time);
        }
    }

    fn insert_velocity(&mut self, vehicle: &Vehicle) {
        if let Some(min_velocity) = self.min_velocity.get(&vehicle.id) {
            if vehicle.velocity < *min_velocity {
                self.min_velocity.insert(vehicle.id, vehicle.velocity);
            }
        } else {
            self.min_velocity.insert(vehicle.id, vehicle.velocity);
        }
        if let Some(max_velocity) = self.max_velocity.get(&vehicle.id) {
            if vehicle.velocity > *max_velocity {
                self.max_velocity.insert(vehicle.id, vehicle.velocity);
            }
        } else {
            self.max_velocity.insert(vehicle.id, vehicle.velocity);
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        background_texture: &sdl2::render::Texture,
    ) {
        canvas.clear();
        canvas
            .copy(&background_texture, None, None)
            .expect("Failed to copy background texture");
        for vehicle in &mut self.vehicles {
            vehicle.draw(canvas);
        }
        canvas.present();
    }

    fn print_statistics(&self) {
        let mut statistics = String::new();
        statistics.push_str(&format!("Nombre de véhicules: {}\n", self.nbr_vehicles));
        if let Some(max_velocity) = bigger(&self.max_velocity) {
            statistics.push_str(&format!("Vitesse maximale: {}\n", max_velocity));
        }
        if let Some(min_velocity) = smallest(&self.min_velocity) {
            statistics.push_str(&format!("Vitesse minimale: {}\n", min_velocity));
        }
        if let Some(max_time) = bigger(&self.max_time) {
            statistics.push_str(&format!(
                "Temps maximal dans l'intersection: {:.2}\n",
                max_time
            ));
        }
        if let Some(mut min_time) = smallest(&self.min_time) {
            if min_time <= 0.0 {
                min_time = 1.0;
            }
            statistics.push_str(&format!("Temps minimal dans l'intersection: {:.2}\n",min_time));
        }
        let close_calls = &self.close_calls.len();
        if !self.close_calls.is_empty() {
            statistics.push_str(&format!("Nombre d'appels rapprochés: {}\n", close_calls));
        }

        statistics.push_str(&format!("Nombre de collisions: {}\n", self.nb_collisions));

        show_simple_message_box(
            MessageBoxFlag::INFORMATION,
            "!!STATISTICS!!",
            &statistics,
            None,
        )
        .unwrap();
    }
}
