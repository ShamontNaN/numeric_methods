use common::Errors;

pub fn bisect<F>(
    func: F,
    mut left_limit: f64,
    mut right_limit: f64,
    precision: f64,
) -> Result<f64, Errors>
where
    F: Fn(f64) -> f64,
{
    if func(left_limit).signum() == func(right_limit).signum() {
        Err(Errors::IncorrectRange)
    }
    if func(left_limit) == 0.0 {
        Ok(left_limit)
    } else if func(right_limit) == 0.0 {
        Ok(right_limit)
    } else {
        let mut temp = 0.0;
        while (right_limit - left_limit).abs() > precision {
            temp = left_limit + (right_limit - left_limit) / 2.0;
            if func(left_limit).signum() != func(temp).signum() {
                right_limit = temp;
            } else {
                left_limit = temp;
            }
        }
        Ok(temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = bisect(|x| 5.0 * x - 20.0, 4.0, 5.0, 0.005);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn incorrect_range() {
        let result = bisect(|x| 5.0 * x - 20.0, 5.0, 6.0, 0.005);
        assert_eq!(result, 5.0);
    }
}
