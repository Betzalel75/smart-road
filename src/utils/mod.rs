
pub mod func {
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::{Vehicle, Direction, DirectionPath};

    pub fn bigger(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {
            return None;
        }
        let mut biggest = 0.0;
        for &value in h.values() {
            if value > biggest {
                biggest = value;
            }
        }
        Some(biggest)
    }
    
    pub fn smallest(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {
            return None;
        }
        let mut small = f32::MAX;
        for val in h.values() {
            if *val < small {
                small = *val;
            }
        }
        Some(small)
    }

pub fn check_and_prevent_collision(vehicles: &mut [Vehicle]) {
    for i in 0..vehicles.len() {
        for j in 0..vehicles.len() {
            if i != j {
                if can_collide(&vehicles[i], &vehicles[j]) { 
                        if vehicles[j].time_in_intersection > vehicles[i].time_in_intersection && vehicles[j].in_intersection(){
                            if vehicles[i].velocity >= 3.0 {
                                vehicles[i].velocity = 1.0; // Le véhicule i ralentit
                                vehicles[i].has_slowed_down = true;
                                vehicles[j].has_slowed_down = false;
                                vehicles[i].last_stop = Instant::now();
                            }
                            vehicles[j].velocity = 9.0; // Le véhicule j accélère
                        } else if vehicles[j].time_in_intersection < vehicles[i].time_in_intersection && vehicles[i].in_intersection(){
                            if vehicles[j].velocity >= 3.0 {
                                vehicles[j].velocity = 1.0; // Le véhicule j ralentit
                                vehicles[j].has_slowed_down = true;
                                vehicles[i].has_slowed_down = false;
                                vehicles[j].last_stop = Instant::now();
                            }
                            vehicles[i].velocity = 9.0; // Le véhicule i accélère
                        }
                }
            }
        }
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