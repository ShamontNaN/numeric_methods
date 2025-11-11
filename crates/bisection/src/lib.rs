pub fn bisect<F>(func:F, mut left_limit:f64, mut right_limit:f64, precision:f64) -> f64
where
    F:Fn(f64) -> f64
{
    if func(left_limit) == 0.0
    {
        left_limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = bisect(|x| 5.0*x-20.0, 4.0, 5.0, 0.005);
        assert_eq!(result, 4.0);    
    }
}
