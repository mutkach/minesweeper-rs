use macroquad::rand::RandomRange;

pub fn random_sample(pop_size: usize, sample_size: usize) -> Vec<usize> {
    let mut result = Vec::from_iter(0..sample_size);
    for i in 0..sample_size {
        let roll = RandomRange::gen_range(i, pop_size);
        if roll < sample_size {
            result.swap(i, roll);
        } else {
            result[i] = roll;
        }
    }
    result
}
