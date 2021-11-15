use rand::{thread_rng, Rng};

use crate::{constants::MUTATION_RATE, helpers::create_gene, structs::Gene};

pub fn should_mutate() -> bool {
    let flt: f32 = thread_rng().gen_range(0.0..1.0);
    let val = flt < MUTATION_RATE;
    val
}

pub fn substitution(mut genes: Vec<Gene>) -> Vec<Gene> {
    let mutant_gene = create_gene();
    let rand_index = thread_rng().gen_range(0..genes.len() - 1);
    genes[rand_index] = mutant_gene;
    genes
}
