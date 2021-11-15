use rand::{thread_rng, Rng};

pub fn rand_from_weighted_items(items: &Vec<f32>) -> usize {
    let mut total_weight: f32 = 0.0;

    for item in items {
        total_weight += item;
    }

    let target: f32 = thread_rng().gen_range(0.0..total_weight);

    let mut accumulator: f32 = 0.0;
    let mut i = 0;

    while i < items.len() {
        accumulator += items[i];

        if accumulator >= target as f32 {
            return i;
        }
        i += 1;
    }

    return items.len() - 1;
}
