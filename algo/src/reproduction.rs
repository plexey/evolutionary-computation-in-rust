use crate::constants::POP_LIMIT;
use crate::mutation::{should_mutate, substitution};
use crate::selection::rand_from_weighted_items;
use crate::structs::{Dot, Gene, Point};
use rand::prelude::SliceRandom;

fn shuffle_genes(mut genes: Vec<Gene>) -> Vec<Gene> {
    genes.shuffle(&mut rand::thread_rng());
    genes
}

fn copy_genes(genes: &Vec<Gene>) -> Vec<Gene> {
    let mut genes_copy: Vec<Gene> = Vec::new();

    for item in genes {
        let new_point = Point {
            x: item.value.x,
            y: item.value.y,
        };
        let new_gene = Gene { value: new_point };
        genes_copy.push(new_gene);
    }

    genes_copy
}

fn subset_parent_genes(genes: Vec<Gene>) -> Vec<Gene> {
    let shuffled_genes = shuffle_genes(genes);

    let n = &shuffled_genes.len() / 2;
    let mut subset: Vec<Gene> = Vec::new();

    for item in &shuffled_genes[0..n] {
        let new_point = Point {
            x: item.value.x,
            y: item.value.y,
        };
        let new_gene = Gene { value: new_point };
        subset.push(new_gene);
    }

    subset
}

pub fn map_to_fitness_weights(dots: &Vec<Dot>) -> Vec<f32> {
    let mut weights: Vec<f32> = Vec::new();

    for dot in dots {
        weights.push(dot.fitness_score);
    }

    weights
}

pub fn crossover(parent_a_genes: Vec<Gene>, parent_b_genes: Vec<Gene>) -> Vec<Gene> {
    let parent_a_genes_subset = subset_parent_genes(parent_a_genes);
    let parent_b_genes_subset = subset_parent_genes(parent_b_genes);

    let combined_parent_genes: Vec<Gene> = parent_a_genes_subset
        .into_iter()
        .chain(parent_b_genes_subset.into_iter())
        .collect();

    let mut child_genes = shuffle_genes(combined_parent_genes);

    if should_mutate() {
        child_genes = substitution(child_genes);
    }

    child_genes
}

pub fn spawn_generation(dots: Vec<Dot>) -> Vec<Dot> {
    let mut new_generation: Vec<Dot> = Vec::new();

    let mut i = 0;
    let fitness_weights = map_to_fitness_weights(&dots);

    while i < POP_LIMIT {
        let dot_a_index = rand_from_weighted_items(&fitness_weights);
        let dot_b_index = rand_from_weighted_items(&fitness_weights);

        let parent_a_genes = copy_genes(&dots[dot_a_index].genes);
        let parent_b_genes = copy_genes(&dots[dot_b_index].genes);

        let child_genes = crossover(parent_a_genes, parent_b_genes);

        let child = Dot {
            genes: child_genes,
            position: Point { x: 0, y: 0 },
            fitness_score: 0.0,
        };

        new_generation.push(child);

        i += 1;
    }

    new_generation
}
