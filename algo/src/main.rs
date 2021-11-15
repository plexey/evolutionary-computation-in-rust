use structs::{Dot, Point};

use crate::constants::{GENERATION_LIMIT, GOAL, POP_LIMIT, START, WORLD_HEIGHT, WORLD_WIDTH};
use crate::fitness::update_fitness;
use crate::helpers::{express_genes, spawn_dots};
use crate::reproduction::spawn_generation;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod constants;
mod fitness;
mod helpers;
mod mutation;
mod reproduction;
mod selection;
mod structs;

fn find_fittest_dot(dots: &Vec<Dot>) -> &Dot {
    let mut best_fitness_score = 0.0;
    let mut fittest_dot_index = 0;

    for (i, dot) in dots.iter().enumerate() {
        if dot.fitness_score > best_fitness_score {
            best_fitness_score = dot.fitness_score;
            fittest_dot_index = i;
        }
    }

    let fittest_dot = &dots[fittest_dot_index];

    fittest_dot
}

fn find_fittest_path(dots: &Vec<Dot>) -> Vec<Point> {
    let fittest_dot = find_fittest_dot(&dots);

    let mut path: Vec<Point> = vec![START];

    let mut current_position: Point = START;

    for gene in &fittest_dot.genes {
        current_position.x += gene.value.x;
        current_position.y += gene.value.y;

        let new_point = Point {
            x: current_position.x,
            y: current_position.y,
        };

        path.push(new_point);
    }

    path
}

fn main() {
    let remove_file_result = std::fs::remove_file("../data/paths.txt");

    if remove_file_result.is_err() {
        println!("File does not exist!");
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("../data/paths.txt")
        .unwrap();

    let mut dots = spawn_dots(POP_LIMIT);

    let mut i = 0;

    let headers = json!({
        "type": "headers",
        "worldHeight": &WORLD_HEIGHT,
        "worldWidth": &WORLD_WIDTH,
        "start": &START,
        "goal": &GOAL
    });

    if let Err(e) = writeln!(file, "{}", headers.to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }

    while i < GENERATION_LIMIT {
        dots = express_genes(dots);
        dots = update_fitness(dots);

        let fittest_path = find_fittest_path(&dots);
        let fittest_path_json = serde_json::to_string(&fittest_path).unwrap();

        if let Err(e) = writeln!(file, "{}", fittest_path_json) {
            eprintln!("Couldn't write to file: {}", e);
        }

        dots = spawn_generation(dots);
        i += 1;
    }
}
