pub mod func {
    use crate::{Direction, DirectionPath, Vehicle};
    use std::collections::HashMap;

    pub fn bigger(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {
            return None;
        }
        h.values().cloned().max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn smallest(h: &HashMap<u32, f32>) -> Option<f32> {
        if h.is_empty() {
            return None;
        }
        h.values().cloned().min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn can_collide(vehicle1: &Vehicle, vehicle2: &Vehicle) -> bool {
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
