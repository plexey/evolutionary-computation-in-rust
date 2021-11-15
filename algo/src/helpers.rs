use rand::Rng;

use crate::{
    constants::{INITIAL_GENE_LENGTH, MAX_STEP_MOVEMENT, MIN_STEP_MOVEMENT},
    structs::{Dot, Gene, Point},
};

pub fn rand_val_in_range(min: i32, max: i32) -> i32 {
    let val = rand::thread_rng().gen_range(min..max);
    val
}

pub fn create_point() -> Point {
    let x = rand_val_in_range(MIN_STEP_MOVEMENT, MAX_STEP_MOVEMENT);
    let y = rand_val_in_range(MIN_STEP_MOVEMENT, MAX_STEP_MOVEMENT);
    let point = Point { x, y };
    point
}

/////////// GENES

pub fn create_gene() -> Gene {
    let gene = Gene {
        value: create_point(),
    };
    gene
}

pub fn create_genes() -> Vec<Gene> {
    let mut genes: Vec<Gene> = Vec::new();
    let mut i = 0;

    while i < INITIAL_GENE_LENGTH {
        let gene = create_gene();
        genes.push(gene);
        i += 1
    }

    genes
}

////////////////// DOTS

pub fn spawn_dot() -> Dot {
    Dot {
        genes: create_genes(),
        position: Point { x: 0, y: 0 },
        fitness_score: 0.0,
    }
}

pub fn spawn_dots(quantity: u32) -> Vec<Dot> {
    let mut dots: Vec<Dot> = Vec::new();
    let mut i: u32 = 0;

    while i < quantity {
        let dot = spawn_dot();
        dots.push(dot);
        i += 1;
    }

    dots
}

pub fn express_genes(mut dots: Vec<Dot>) -> Vec<Dot> {
    for dot in dots.iter_mut() {
        dot.express_genes();
    }
    dots
}

/////////////////// UTILS

pub fn compute_distance(p1: &Point, p2: &Point) -> f32 {
    let horrizontal_distance = (p2.x - p1.x).pow(2);
    let vertical_distance = (p2.y - p1.y).pow(2);
    let combined = horrizontal_distance + vertical_distance;
    (combined as f32).sqrt()
}

pub fn compute_ratio(value: &f32) -> f32 {
    value / 100.00
}
