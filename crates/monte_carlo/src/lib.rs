use rand::distr::{Distribution, Uniform};

pub fn monte_carlo<F>(func: F, left_limit: f64, right_limit: f64, count: i32) -> f64
where
    F: Fn(f64) -> f64,
{
    let between = Uniform::try_from(left_limit..=right_limit);
    let mut rng = rand::rng();
    (right_limit - left_limit) / (count as f64)
        * (0..count)
            .map(|_| func(between.unwrap().sample(&mut rng)))
            .sum::<f64>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!();
    }
}
