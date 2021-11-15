use crate::{
    constants::{GOAL, START},
    helpers::{compute_distance, compute_ratio},
    structs::Dot,
};

pub fn compute_dot_fitness(distance_from_goal: f32, max_distance_from_goal_ratio: f32) -> f32 {
    let result = distance_from_goal / max_distance_from_goal_ratio;
    let rounded = result.round();
    let output = 1. - rounded / 100.;
    output
}

pub fn update_fitness(mut dots: Vec<Dot>) -> Vec<Dot> {
    let mut max_distance_from_goal = compute_distance(&START, &GOAL);

    for dot in &dots {
        let distance_from_goal = dot.compute_distance_from_goal();
        if distance_from_goal > max_distance_from_goal {
            max_distance_from_goal = distance_from_goal;
        }
    }

    let max_distance_from_goal_ratio = compute_ratio(&max_distance_from_goal);

    for dot in dots.iter_mut() {
        let distance_from_goal = dot.compute_distance_from_goal();
        let fitness_score = compute_dot_fitness(distance_from_goal, max_distance_from_goal_ratio);
        dot.fitness_score = fitness_score;
    }

    dots
}
