use crate::structs::Point;

pub const POP_LIMIT: u32 = 400;
pub const INITIAL_GENE_LENGTH: u32 = 200;
pub const START: Point = Point { x: 0, y: 0 };
pub const GOAL: Point = Point { x: 2000, y: 1000 };
pub const MUTATION_RATE: f32 = 0.5;
pub const GENERATION_LIMIT: u32 = 25;

pub const WORLD_WIDTH: i32 = 2000;
pub const WORLD_HEIGHT: i32 = 1000;

pub const MAX_STEP_MOVEMENT: i32 = 30;
pub const MIN_STEP_MOVEMENT: i32 = -30;