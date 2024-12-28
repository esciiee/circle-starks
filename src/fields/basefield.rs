use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const PRIME: u32 = (1 << 31) - 1;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct BaseField(pub u32);

impl BaseField {
    pub fn new(value: u32) -> Self {
        BaseField(value % PRIME)
    }

    pub fn pow(&self, exp: u128) -> Self {
        let mut res = BaseField::new(1);
        let mut base = self.clone();
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base = base.square();
            exp >>= 1;
        }
        res
    }

    pub fn square(self) -> Self {
        self * self
    }

    pub fn inverse(self) -> BaseField {
        assert!(self != BaseField::new(0));
        let t0 = Self::sqn::<2>(self) * self;
        let t1 = Self::sqn::<1>(t0) * t0;
        let t2 = Self::sqn::<3>(t1) * t0;
        let t3 = Self::sqn::<1>(t2) * t0;
        let t4 = Self::sqn::<8>(t3) * t3;
        let t5 = Self::sqn::<8>(t4) * t3;
        Self::sqn::<7>(t5) * t2
    }

    /// Computes `v^(2*n)`.
    fn sqn<const N: usize>(mut v: BaseField) -> BaseField {
        for _ in 0..N {
            v = v.square();
        }
        v
    }
}

impl Add for BaseField {
    type Output = BaseField;

    fn add(self, other: BaseField) -> BaseField {
        BaseField((self.0 + other.0) % PRIME)
    }
}

impl AddAssign for BaseField {
    fn add_assign(&mut self, other: BaseField) {
        *self = *self + other;
    }
}

impl Sub for BaseField {
    type Output = BaseField;

    fn sub(self, other: BaseField) -> BaseField {
        BaseField((self.0 + PRIME - other.0) % PRIME)
    }
}

impl SubAssign for BaseField {
    fn sub_assign(&mut self, other: BaseField) {
        *self = *self - other;
    }
}

impl Mul for BaseField {
    type Output = BaseField;

    fn mul(self, other: BaseField) -> BaseField {
        BaseField(((self.0 as u64 * other.0 as u64) % PRIME as u64) as u32)
    }
}

impl MulAssign for BaseField {
    fn mul_assign(&mut self, other: BaseField) {
        *self = *self * other;
    }
}

impl Neg for BaseField {
    type Output = BaseField;

    fn neg(self) -> BaseField {
        BaseField(PRIME - self.0)
    }
}

impl std::ops::Div for BaseField {
    type Output = BaseField;

    fn div(self, other: BaseField) -> BaseField {
        self * other.inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};

    const fn mul_p(a: u32, b: u32) -> u32 {
        ((a as u64 * b as u64) % PRIME as u64) as u32
    }

    const fn add_p(a: u32, b: u32) -> u32 {
        (a + b) % PRIME
    }

    const fn sub_p(a: u32, b: u32) -> u32 {
        if a >= b {
            a - b
        } else {
            PRIME - b + a
        }
    }

    const fn neg_p(a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            PRIME - a
        }
    }

    #[test]
    fn test_basic_ops() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10000 {
            let x: u32 = rng.gen::<u32>() % PRIME;
            let y: u32 = rng.gen::<u32>() % PRIME;
            assert_eq!(
                BaseField::new(add_p(x, y)),
                BaseField::new(x) + BaseField::new(y)
            );
            assert_eq!(
                BaseField::new(mul_p(x, y)),
                BaseField::new(x) * BaseField::new(y)
            );
            assert_eq!(BaseField::new(neg_p(x)), -BaseField::new(x));
        }
    }

    #[test]
    fn test_sub_ops() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10000 {
            let x: u32 = rng.gen::<u32>() % PRIME;
            let y: u32 = rng.gen::<u32>() % PRIME;
            assert_eq!(
                BaseField::new(sub_p(x, y)),
                BaseField::new(x) - BaseField::new(y)
            );
        }
    }

    #[test]
    fn test_div_ops() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10000 {
            let x: u32 = rng.gen::<u32>() % PRIME;
            let y: u32 = rng.gen::<u32>() % (PRIME - 1) + 1; // avoid division by zero
            let bx = BaseField::new(x);
            let by = BaseField::new(y);
            assert_eq!(bx / by, bx * by.inverse());
        }
    }

    #[test]
    fn test_pow_ops() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..1000 {
            let x: u32 = rng.gen::<u32>() % PRIME;
            let exp: u128 = rng.gen::<u128>() % 1000; // limiting exponent size for test
            let bx = BaseField::new(x);
            let mut expected = BaseField::new(1);
            for _ in 0..exp {
                expected *= bx;
            }
            assert_eq!(bx.pow(exp), expected);
        }
    }

    #[test]
    fn test_inverse() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10000 {
            let x: u32 = rng.gen::<u32>() % (PRIME - 1) + 1;
            let bx = BaseField::new(x);
            assert_eq!(bx * bx.inverse(), BaseField::new(1));
        }
    }
}
