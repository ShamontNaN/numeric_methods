#![feature(random)]
use std::random::random;

pub fn monte_carlo<F>(func:F, left_limit:f64, right_limit:f64, count_of_random_value:i32)
where
    F: Fn(f64) -> f64
{
    
}

#[cfg(test)]
mod tests {
    use std::f64::consts::E;

    use super::*;

    #[test]
    fn it_works() {
        let result = monte_carlo(|x| E.powf(x)/(1.0+E.powf(2.0*x)), 0.0, 1.0, 10000);
        
    }
}
