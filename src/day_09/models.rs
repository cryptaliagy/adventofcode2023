use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Rational(pub i128, pub i128);

impl Rational {
    pub fn new(n: i128, d: i128) -> Self {
        let gcd = gcd(n, d);

        Self(n / gcd, d / gcd)
    }

    pub fn into(self) -> Result<i64, ()> {
        if self.1 != 1 {
            return Err(());
        }

        Ok(self.0 as i64)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

impl std::ops::Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let gcd1 = gcd(self.0, rhs.1);
        let gcd2 = gcd(self.1, rhs.0);

        let n = (self.0 / gcd1) * (rhs.0 / gcd2);
        let d = (self.1 / gcd2) * (rhs.1 / gcd1);
        Self::new(n, d)
    }
}

impl std::ops::Div for Rational {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * Self(rhs.1, rhs.0)
    }
}

impl std::ops::Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let gcd1 = gcd(self.1, rhs.1);

        let other_numerator = rhs.0 * (self.1 / gcd1);
        let self_numerator = self.0 * (rhs.1 / gcd1);

        let n = self_numerator + other_numerator;

        let max_den = self.1.max(rhs.1);
        let min_den = self.1.min(rhs.1);

        let res = min_den.checked_mul(max_den / gcd1);

        if res.is_none() {
            println!("{} {} {} {}", self.0, self.1, rhs.0, rhs.1);
        }

        Self::new(n, min_den * (max_den / gcd1))
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + Self(-rhs.0, rhs.1)
    }
}

impl std::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl std::cmp::Eq for Rational {}

impl From<i128> for Rational {
    fn from(value: i128) -> Self {
        Self(value, 1)
    }
}

struct BarycentricWeights {
    cache: HashMap<(i128, i128), Rational>,
}

impl BarycentricWeights {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, n: i128, j: i128) -> Rational {
        if let Some(r) = self.cache.get(&(n, j)) {
            return *r;
        }

        let r = self.get_inner(n, j);
        self.cache.insert((n, j), r);
        r
    }

    fn get_inner(&self, n: i128, j: i128) -> Rational {
        let mut total = 1;

        for i in 0..j {
            total *= j - i;
        }

        for i in (j + 1)..n {
            total *= j - i;
        }

        Rational::new(1, total)
    }
}

pub struct Interpolator {
    weights: BarycentricWeights,
}

impl Interpolator {
    pub fn new() -> Self {
        Self {
            weights: BarycentricWeights::new(),
        }
    }

    pub fn interpolate(&mut self, points: &[i128], x: i64) -> Result<i64, ()> {
        let n = points.len() as i128;

        let mut denominator = Rational::from(0);
        let mut numerator = Rational::from(0);

        for (i, &y) in points.iter().enumerate() {
            let weight = self.weights.get(n, i as i128);

            let common = weight / Rational::from(x as i128 - i as i128);
            denominator = denominator + common;

            numerator = numerator + (common * Rational::from(y));
        }

        (numerator / denominator).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use std::ops::Range;

    #[test]
    fn test_barycentric_weight() {
        let mut weights = BarycentricWeights::new();

        assert_eq!(weights.get(3, 0), Rational::new(1, 2));
    }

    fn test_interpolate_polynomial_random_target(
        interpolator: &mut Interpolator,
        d: i128,
        range: Range<i32>,
        polynomial: &dyn Fn(i128) -> i128,
    ) {
        let points = (0..d).map(polynomial).collect::<Vec<_>>();

        let x = rand::thread_rng().gen_range(range) as i64;

        let y = interpolator.interpolate(&points, x).unwrap();

        assert_eq!(y, polynomial(x as i128) as i64);
    }

    #[test]
    fn test_interpolate_linear() {
        let mut interpolator = Interpolator::new();

        for _ in 0..20 {
            let coefficient = rand::thread_rng().gen_range(1..100);
            let constant = rand::thread_rng().gen_range(1..100);

            let range = 4..1000;

            test_interpolate_polynomial_random_target(&mut interpolator, 2, range, &|x| {
                coefficient * x + constant
            });
        }
    }

    #[test]
    fn test_interpolate_quadratic() {
        let mut interpolator = Interpolator::new();

        for _ in 0..20 {
            let a_coefficient = rand::thread_rng().gen_range(1..100);
            let b_coefficient = rand::thread_rng().gen_range(1..100);
            let constant = rand::thread_rng().gen_range(1..100);

            let range = 4..100;

            test_interpolate_polynomial_random_target(&mut interpolator, 3, range, &|x| {
                a_coefficient * x.pow(2) + b_coefficient * x + constant
            });
        }
    }

    #[test]
    fn test_interpolate_cubic() {
        let mut interpolator = Interpolator::new();

        for _ in 0..20 {
            let a_coefficient = rand::thread_rng().gen_range(1..100);
            let b_coefficient = rand::thread_rng().gen_range(1..100);
            let c_coefficient = rand::thread_rng().gen_range(1..100);
            let constant = rand::thread_rng().gen_range(1..100);

            let range = 4..100;

            test_interpolate_polynomial_random_target(&mut interpolator, 4, range, &|x| {
                a_coefficient * x.pow(3) + b_coefficient * x.pow(2) + c_coefficient * x + constant
            });
        }
    }

    #[test]
    fn test_interpolate_quartic() {
        let mut interpolator = Interpolator::new();

        for _ in 0..20 {
            let a_coefficient = rand::thread_rng().gen_range(1..100);
            let b_coefficient = rand::thread_rng().gen_range(1..100);
            let c_coefficient = rand::thread_rng().gen_range(1..100);
            let d_coefficient = rand::thread_rng().gen_range(1..100);
            let constant = rand::thread_rng().gen_range(1..100);

            let range = 4..100;

            test_interpolate_polynomial_random_target(&mut interpolator, 5, range, &|x| {
                a_coefficient * x.pow(4)
                    + b_coefficient * x.pow(3)
                    + c_coefficient * x.pow(2)
                    + d_coefficient * x
                    + constant
            });
        }
    }

    #[test]
    fn test_interpolate_quintic() {
        let mut interpolator = Interpolator::new();

        for _ in 0..20 {
            let a_coefficient = rand::thread_rng().gen_range(-100..100);
            let b_coefficient = rand::thread_rng().gen_range(-100..100);
            let c_coefficient = rand::thread_rng().gen_range(-100..100);
            let d_coefficient = rand::thread_rng().gen_range(-100..100);
            let e_coefficient = rand::thread_rng().gen_range(-100..100);
            let constant = rand::thread_rng().gen_range(-100..100);

            let range = 4..1000;

            test_interpolate_polynomial_random_target(&mut interpolator, 6, range, &|x| {
                a_coefficient * x.pow(5)
                    + b_coefficient * x.pow(4)
                    + c_coefficient * x.pow(3)
                    + d_coefficient * x.pow(2)
                    + e_coefficient * x
                    + constant
            });
        }
    }
}
