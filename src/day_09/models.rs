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
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            panic!("Cannot divide by zero");
        }
        Self::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl std::ops::Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let n = self.0 * rhs.1 + self.1 * rhs.0;
        let d = self.1 * rhs.1;

        Self::new(n, d)
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.0 * rhs.1 - self.1 * rhs.0;
        let d = self.1 * rhs.1;

        Self::new(n, d)
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
    cache: HashMap<(usize, usize), i128>,
}

impl BarycentricWeights {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_uncached(&self, n: usize, j: usize) -> Rational {
        let mut total: i128 = 1;

        let j = j as i128;
        let n = n as i128;

        for i in 0..j {
            total *= j - i;
        }

        for i in (j + 1)..n {
            total *= j - i;
        }

        Rational::new(1, total)
    }

    pub fn get(&mut self, n: usize, j: usize) -> Rational {
        if let Some(r) = self.cache.get(&(n, j)) {
            return Rational::new(1, *r);
        }

        let r = self.get_inner(n, j);
        self.cache.insert((n, j), r);
        Rational::new(1, r)
    }

    fn get_inner(&mut self, n: usize, j: usize) -> i128 {
        if j == 0 {
            let operator = if n % 2 == 0 { -1 } else { 1 };
            -(*self
                .cache
                .entry((n - 1, 0))
                .or_insert_with(|| ((1..(n - 1)).product::<usize>() as i128) * -operator)
                * (n - 1) as i128)
        } else {
            let i = j as i128;
            let k = n as i128;

            -(*self.cache.get(&(n, j - 1)).unwrap() * i) / (k - i)
        }
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
        let n = points.len();

        let mut denominator = Rational::from(0);
        let mut numerator = Rational::from(0);

        for (i, &y) in points.iter().enumerate() {
            let weight = self.weights.get(n, i);

            let x = x.checked_sub_unsigned(i as u64).unwrap() as i128;

            let common = weight / Rational::from(x);
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

    fn get_uncached(n: usize, j: usize) -> Rational {
        let mut total: i128 = 1;

        let j = j as i128;
        let n = n as i128;

        for i in 0..j {
            total *= j - i;
        }

        for i in (j + 1)..n {
            total *= j - i;
        }

        Rational::new(1, total)
    }

    #[test]
    fn test_barycentric_weight() {
        let mut weights = BarycentricWeights::new();

        for n in 3..20 {
            for j in 0..n {
                assert_eq!(weights.get(n, j), get_uncached(n, j), "{}, {}", n, j);
            }
        }
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
    fn test_barycentric_weights() {
        let mut weights = BarycentricWeights::new();

        let n = 9;
        for i in 0..n {
            println!("{:?} {:?}", weights.get(n, i), get_uncached(n, i));
        }
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
