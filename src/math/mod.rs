fn order_ascending(a: u64, b: u64) -> (u64, u64) {
    if a < b {
        return (a, b);
    } else {
        return (b, a);
    }
}

/// This uses Euclid's theorem to return the greatest common divisor between two numbers. If the numbers
/// are equal, the greatest common divisor is the number. Zero cannot be one of the inputs, an error will be
/// returned if a or b is equal to zero
///
/// # Examples
/// ```
/// use rs_algo::math::*;
///
/// let divisor = gcd(30, 21);
/// assert_eq!(Ok(3), divisor);
/// ```
pub fn gcd(a: u64, b: u64) -> Result<u64, String> {
    if a == b {
        return Ok(a);
    }

    if a == 0 || b == 0 {
        return Err("0 cannot be used as an input for gcd".to_string());
    }

    let (mut _b, mut _a) = order_ascending(a, b);
    loop {
        if _b == 0 {
            break;
        }

        let new_b = _a % _b;
        _a = _b;
        _b = new_b;
    }

    Ok(_a)
}

/// This will check if a and b are prime relative to each other. If two numbers are relatively prime, their GCD will
/// be equal to one
///
/// # Examples
/// ```
/// use rs_algo::math::*;
///
/// let rel_prime = relatively_prime(10, 19);
/// assert_eq!(true, rel_prime);
/// ```
pub fn relatively_prime(a: u64, b: u64) -> bool {
    return gcd(a, b) == Ok(1);
}

/// This will return a vector of factors if any exists. This will exlclude the obvious factors of one and the number itself.
/// If no factors exist (minus the obvious one and the number itself) None will be returned.
///
/// # Examples
/// ```
/// use rs_algo::math::*;
///
/// let factor = factors(9124);
/// assert_eq!(Some(vec![2, 4, 2281, 4562]), factor);
/// ```
pub fn factors(a: i64) -> Option<Vec<i64>> {
    let mut factors: Vec<i64> = Vec::new();

    if a == 0 {
        return None;
    }

    let fsquare = (a as f64).sqrt().floor();
    let limit = fsquare as i64;

    for i in 2..limit + 1 {
        if a % i == 0 {
            factors.push(i);
            factors.push(a / i);
        }
    }

    if factors.is_empty() {
        return None;
    }

    // if we dedup on a sorted array, it will remove any duplications.
    factors.sort_unstable();
    factors.dedup();
    Some(factors)
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd_test() {
        use super::*;

        assert_eq!(Ok(3), gcd(30, 21));
        assert_eq!(Ok(30), gcd(30, 30));
        assert_eq!(true, gcd(0, 21).is_err());
        assert_eq!(Ok(2), gcd(22123322, 4223234));
        assert_eq!(Ok(1), gcd(4, 11));
    }

    #[test]
    fn relative_prime_test() {
        use super::*;

        assert_eq!(false, relatively_prime(30, 21));
        assert_eq!(true, relatively_prime(4, 15));
        assert_eq!(true, relatively_prime(10, 19));
    }

    #[test]
    fn factors_test() {
        use super::*;

        assert_eq!(None, factors(11));
        assert_eq!(Some(vec![5]), factors(25));
        assert_eq!(Some(vec![2, 4, 8, 16]), factors(32));
        assert_eq!(Some(vec![2, 4, 2281, 4562]), factors(9124));
    }
}
