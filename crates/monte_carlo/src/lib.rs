
use rand::distr::{Distribution, Uniform, uniform::SampleRange};

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

pub fn left_rectangle<F>(func: F, left_limit: f64, right_limit: f64, count: i32) -> f64
where 
    F: Fn(f64) -> f64,
{
    let step = (right_limit-left_limit).abs() / (count as f64);
    (0..count).map(|s|{
        func(step.mul_add(s as f64, left_limit))
    }).sum::<f64>()*step
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!();
    }
}
