pub struct Point {
    argument: f64,
    value: f64,
}

impl Point {
    pub fn new(argument: f64, value: f64) -> Self {
        Point { argument, value }
    }
}

pub fn lagrange_interpolation(points: &[Point], x: f64) -> f64 {
    points
        .iter()
        .enumerate()
        .map(|(j, pj)| {
            pj.value
                * points
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != j)
                    .map(|(_, pi)| (x - pi.argument) / (pj.argument - pi.argument))
                    .product::<f64>()
        })
        .sum::<f64>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        todo!();
    }
}
