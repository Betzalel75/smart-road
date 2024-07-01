pub mod func {
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::{Direction, DirectionPath, Vehicle, VEHICLE_HEIGHT, VEHICLE_WIDTH};

    use super::quatree::{Quadtree, AABB};
    pub fn bigger(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {return None;}
        h.values().cloned().max_by(|a, b| a.partial_cmp(b).unwrap())
    }
    
    pub fn smallest(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {return None;}
        h.values().cloned().min_by(|a, b| a.partial_cmp(b).unwrap())
    }

pub fn check_and_prevent_collision(vehicles: &mut [Vehicle]) {
    let boundary = AABB { x: 271.0, y: 275.0, width: (VEHICLE_WIDTH *16 )as f32, height: (VEHICLE_HEIGHT * 8) as f32 }; 
    let mut quadtree = Quadtree::new(boundary, 4); // La capacité peut être ajustée selon les besoins

    for vehicle in vehicles.iter().cloned() {
        quadtree.insert(vehicle);
    }

    for vehicle in vehicles.iter_mut() {
        let range = AABB { x: vehicle.position.x - 1.0, y: vehicle.position.y - 1.0, width: 2.0, height: 2.0 }; // Définir une portée raisonnable
        let mut potential_collisions = Vec::new();
        quadtree.query(&range, &mut potential_collisions);

        for other in potential_collisions.iter_mut() {
            if vehicle.id != other.id && can_collide(vehicle, other) {
                adjust_velocity(vehicle, other);
            }
        }
    }
}

fn adjust_velocity(vehicle1: &mut Vehicle, vehicle2: &mut Vehicle) {
    if vehicle2.in_intersection() {
        if vehicle2.time_in_intersection > vehicle1.time_in_intersection {
            if vehicle1.velocity >= 3.0 {
                vehicle1.velocity = 0.5;
                vehicle1.last_stop = Instant::now();
                vehicle1.has_slowed_down = true;
            }
            vehicle2.velocity = 9.0;
            vehicle2.has_slowed_down = false;
        }
    } else if vehicle1.in_intersection() && vehicle1.time_in_intersection > vehicle2.time_in_intersection {
        if vehicle2.velocity >= 3.0 {
            vehicle2.velocity = 0.5;
            vehicle2.last_stop = Instant::now();
            vehicle2.has_slowed_down = true;
        }
        vehicle1.velocity = 9.0;
        vehicle1.has_slowed_down = false;
    }
}

fn can_collide(vehicle1: &Vehicle, vehicle2: &Vehicle) -> bool {
    match vehicle1.direction {
        Direction::North(DirectionPath::TurnLeft) => matches!(
            vehicle2.direction,
            Direction::South(DirectionPath::TurnLeft)
            | Direction::South(DirectionPath::GoStraight)
            | Direction::West(DirectionPath::TurnLeft)
            | Direction::East(DirectionPath::GoStraight)
            | Direction::East(DirectionPath::TurnLeft)
        ),
        Direction::North(DirectionPath::GoStraight) => matches!(
            vehicle2.direction,
            Direction::South(DirectionPath::TurnLeft)
            | Direction::West(DirectionPath::GoStraight)
            | Direction::West(DirectionPath::TurnLeft)
            | Direction::East(DirectionPath::GoStraight)
        ),
        Direction::South(DirectionPath::TurnLeft) => matches!(
            vehicle2.direction,
            Direction::West(DirectionPath::TurnLeft)
            | Direction::West(DirectionPath::GoStraight)
            | Direction::North(DirectionPath::TurnLeft)
            | Direction::East(DirectionPath::GoStraight)
            | Direction::East(DirectionPath::TurnLeft)
        ),
        Direction::South(DirectionPath::GoStraight) => matches!(
            vehicle2.direction,
            Direction::East(DirectionPath::TurnLeft)
            | Direction::East(DirectionPath::GoStraight)
            | Direction::North(DirectionPath::TurnLeft)
            | Direction::West(DirectionPath::GoStraight)
        ),
        Direction::East(DirectionPath::TurnLeft) => matches!(
            vehicle2.direction,
            Direction::South(DirectionPath::TurnLeft)
            | Direction::South(DirectionPath::GoStraight)
            | Direction::West(DirectionPath::TurnLeft)
            | Direction::West(DirectionPath::GoStraight)
            | Direction::North(DirectionPath::TurnLeft)
        ),
        Direction::East(DirectionPath::GoStraight) => matches!(
            vehicle2.direction,
            Direction::West(DirectionPath::TurnLeft)
            | Direction::South(DirectionPath::GoStraight)
            | Direction::North(DirectionPath::TurnLeft)
            | Direction::North(DirectionPath::GoStraight)
        ),
        Direction::West(DirectionPath::TurnLeft) => matches!(
            vehicle2.direction,
            Direction::South(DirectionPath::TurnLeft)
            | Direction::South(DirectionPath::GoStraight)
            | Direction::East(DirectionPath::TurnLeft)
            | Direction::North(DirectionPath::GoStraight)
            | Direction::North(DirectionPath::TurnLeft)
        ),
        Direction::West(DirectionPath::GoStraight) => matches!(
            vehicle2.direction,
            Direction::South(DirectionPath::TurnLeft)
            | Direction::South(DirectionPath::GoStraight)
            | Direction::East(DirectionPath::TurnLeft)
            | Direction::North(DirectionPath::GoStraight)
        ),
        _ => false,
    }
}
}

pub mod quatree{
use crate::Vehicle;

#[derive(Clone)]
pub struct AABB{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    fn contains(&self, vehicle: &Vehicle) -> bool {
        vehicle.position.x >= self.x &&
        vehicle.position.x < self.x + self.width &&
        vehicle.position.y >= self.y &&
        vehicle.position.y < self.y + self.height
    }

    fn intersects(&self, other: &AABB) -> bool {
        !(other.x > self.x + self.width ||
          other.x + other.width < self.x ||
          other.y > self.y + self.height ||
          other.y + other.height < self.y)
    }
}

pub struct Quadtree<'a> {
    boundary: AABB,
    capacity: usize,
    vehicles: Vec<Vehicle<'a>>,
    divided: bool,
    northeast: Option<Box<Quadtree<'a>>>,
    northwest: Option<Box<Quadtree<'a>>>,
    southeast: Option<Box<Quadtree<'a>>>,
    southwest: Option<Box<Quadtree<'a>>>,
}

impl<'a> Quadtree<'a> {
    pub fn new(boundary: AABB, capacity: usize) -> Self {
        Quadtree {
            boundary,
            capacity,
            vehicles: Vec::new(),
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        let ne = AABB { x: x + w, y, width: w, height: h };
        let nw = AABB { x, y, width: w, height: h };
        let se = AABB { x: x + w, y: y + h, width: w, height: h };
        let sw = AABB { x, y: y + h, width: w, height: h };

        self.northeast = Some(Box::new(Quadtree::new(ne, self.capacity)));
        self.northwest = Some(Box::new(Quadtree::new(nw, self.capacity)));
        self.southeast = Some(Box::new(Quadtree::new(se, self.capacity)));
        self.southwest = Some(Box::new(Quadtree::new(sw, self.capacity)));

        self.divided = true;
    }

    pub fn insert(&mut self, vehicle: Vehicle<'a>) -> bool {
        if !self.boundary.contains(&vehicle) {
            // println!("Insertion of vehicle is not allowed");
            return false;
        }
        // println!("Insertion of vehicle is allowed");
        if self.vehicles.len() < self.capacity {
            self.vehicles.push(vehicle);
            return true;
        } else {
            if !self.divided {
                self.subdivide();
            }

            if self.northeast.as_mut().unwrap().insert(vehicle.clone()) { return true; }
            if self.northwest.as_mut().unwrap().insert(vehicle.clone()) { return true; }
            if self.southeast.as_mut().unwrap().insert(vehicle.clone()) { return true; }
            if self.southwest.as_mut().unwrap().insert(vehicle) { return true; }
        }

        false
    }

    pub fn query(&self, range: &AABB, found: &mut Vec<Vehicle<'a>>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for v in &self.vehicles {
            if range.contains(v) {
                found.push(v.clone());
            }
        }

        if self.divided {
            self.northeast.as_ref().unwrap().query(range, found);
            self.northwest.as_ref().unwrap().query(range, found);
            self.southeast.as_ref().unwrap().query(range, found);
            self.southwest.as_ref().unwrap().query(range, found);
        }
    }
}

}