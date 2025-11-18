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
        return Err(Errors::IncorrectRange);
    }
    if func(left_limit) == 0.0 {
        return Ok(left_limit);
    } 
    if func(right_limit) == 0.0 {
        return Ok(right_limit);
    }
    let mut mid = 0.0;
    while (right_limit - left_limit).abs() > precision {
        mid = left_limit + (right_limit - left_limit) / 2.0;
        if func(left_limit).signum() != func(mid).signum() {
            right_limit = mid;
        } else {
            left_limit = mid;
        }
    }
    Ok(mid)
}

#[cfg(test)]
mod tests {
    use super::*;

    let 
    #[test]
    fn it_works() {
        let result = bisect(|x| 5.0 * x - 20.0, 4.0, 5.0, 0.005);
        assert!(result.is_ok());
    }

    #[test]
    fn incorrect_range() {
        let result = bisect(|x| 5.0 * x - 20.0, 5.0, 6.0, 0.005);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::IncorrectRange);
    }
}
