use crate::structs::Point;

pub const POP_LIMIT: u32 = 200;
pub const INITIAL_GENE_LENGTH: u32 = 50;
pub const START: Point = Point { x: 300, y: 300 };
pub const GOAL: Point = Point { x: 1500, y: 1500 };
pub const MUTATION_RATE: f32 = 0.4;
pub const GENERATION_LIMIT: u32 = 30;

pub const WORLD_WIDTH: i32 = 1600;
pub const WORLD_HEIGHT: i32 = 1600;

pub const MAX_STEP_MOVEMENT: i32 = 40;
pub const MIN_STEP_MOVEMENT: i32 = -40;