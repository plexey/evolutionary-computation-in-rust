use crate::{constants::GOAL, helpers::compute_distance};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Gene {
    pub value: Point,
}

#[derive(Debug)]
pub struct Dot {
    pub genes: Vec<Gene>,
    pub position: Point,
    pub fitness_score: f32,
}

impl Dot {
    fn offset_position(&mut self, x_offset: i32, y_offset: i32) {
        self.position.x = x_offset;
        self.position.y = y_offset;
    }

    pub fn compute_distance_from_goal(&self) -> f32 {
      let distance = compute_distance(&self.position, &GOAL);
      distance
    } 

    pub fn express_genes(&mut self) {
        let mut x_offset = 0;
        let mut y_offset = 0;

        for gene in self.genes.iter() {
            x_offset += gene.value.x;
            y_offset += gene.value.y;
        }

        self.offset_position(x_offset, y_offset);
    }
}
