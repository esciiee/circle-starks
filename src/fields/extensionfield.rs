use std::{
    array,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::basefield::BaseField;

/// Quadratic extension field of `BaseField`.
/// (a,b,c,d) = (a + bi) + (c + di)j
/// i^2 = -1, j^2 = 2 + i.

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct ExtensionField(pub [BaseField; 4]);

impl ExtensionField {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        ExtensionField([
            BaseField::new(a),
            BaseField::new(b),
            BaseField::new(c),
            BaseField::new(d),
        ])
    }

    pub fn pow(&self, exp: u128) -> Self {
        let mut res = ExtensionField::new(1, 0, 0, 0);
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

    fn inverse(&self) -> Self {
        assert!(*self != ExtensionField::new(0, 0, 0, 0));
        let b2 = Self::square_complex((self.0[2], self.0[3]));
        let ib2 = (-b2.1, b2.0);
        let a2 = Self::square_complex((self.0[0], self.0[1]));
        let denom = (
            a2.0 - (b2.0 + b2.0 + ib2.0),
            a2.1 - (b2.1 + b2.1 + ib2.1),
        );
        let denom_inverse = Self::inverse_complex(denom.0, denom.1);
        let (a, b) = Self::mul_complex((self.0[0], self.0[1]), denom_inverse);
        let (c, d) = Self::mul_complex((-self.0[2], -self.0[3]), denom_inverse);
        Self([a, b, c, d])
    }

    fn inverse_complex(a: BaseField, b: BaseField) -> (BaseField, BaseField) {
        assert!(a != BaseField(0) || b != BaseField(0), "0 has no inverse");
        // 1 / (a + bi) = (a - bi) / (a^2 + b^2).
        Self::mul_complex_base(a, -b, (a.square() + b.square()).inverse())
    }

    fn mul_complex_base(a: BaseField, b: BaseField, c: BaseField) -> (BaseField, BaseField) {
        (a * c, b * c)
    }

    fn mul_complex(a: (BaseField, BaseField), b: (BaseField, BaseField)) -> (BaseField, BaseField) {
        let (a0, a1) = a;
        let (b0, b1) = b;
        let c0 = a0 * b0 - a1 * b1;
        let c1 = a0 * b1 + a1 * b0;
        (c0, c1)
    }

    fn square_complex(a: (BaseField, BaseField)) -> (BaseField, BaseField) {
        Self::mul_complex(a, a)
    }
}

impl Add for ExtensionField {
    type Output = ExtensionField;

    fn add(self, other: ExtensionField) -> ExtensionField {
        ExtensionField(array::from_fn(|i| self.0[i] + other.0[i]))
    }
}

impl AddAssign for ExtensionField {
    fn add_assign(&mut self, other: ExtensionField) {
        *self = *self + other;
    }
}

impl Sub for ExtensionField {
    type Output = ExtensionField;

    fn sub(self, other: ExtensionField) -> ExtensionField {
        ExtensionField(array::from_fn(|i| self.0[i] - other.0[i]))
    }
}

impl SubAssign for ExtensionField {
    fn sub_assign(&mut self, other: ExtensionField) {
        *self = *self - other;
    }
}

impl Mul for ExtensionField {
    type Output = ExtensionField;

    fn mul(self, other: ExtensionField) -> ExtensionField {
        let (a0, b0) = Self::mul_complex((self.0[0], self.0[1]), (other.0[0], other.0[1]));
        let (a1, b1) = Self::mul_complex(
            (BaseField(2), BaseField(1)),
            Self::mul_complex((self.0[2], self.0[3]), (other.0[2], other.0[3])),
        );
        let (a, b) = (a0 + a1, b0 + b1);
        let (c0, d0) = Self::mul_complex((self.0[0], self.0[1]), (other.0[2], other.0[3]));
        let (c1, d1) = Self::mul_complex((self.0[2], self.0[3]), (other.0[0], other.0[1]));
        let (c, d) = (c0 + c1, d0 + d1);
        ExtensionField([a, b, c, d])
    }
}

impl MulAssign for ExtensionField {
    fn mul_assign(&mut self, other: ExtensionField) {
        *self = *self * other;
    }
}

impl Neg for ExtensionField {
    type Output = ExtensionField;

    fn neg(self) -> ExtensionField {
        ExtensionField(array::from_fn(|i| -self.0[i]))
    }
}

impl Div for ExtensionField {
    type Output = ExtensionField;

    fn div(self, other: ExtensionField) -> ExtensionField {
        self * other.inverse()
    }
}

impl Add<BaseField> for ExtensionField {
    type Output = ExtensionField;

    fn add(self, other: BaseField) -> ExtensionField {
        ExtensionField([self.0[0] + other, self.0[1], self.0[2], self.0[3]])
    }
}

impl Sub<BaseField> for ExtensionField {
    type Output = ExtensionField;

    fn sub(self, other: BaseField) -> ExtensionField {
        ExtensionField([self.0[0] - other, self.0[1], self.0[2], self.0[3]])
    }
}

impl Mul<BaseField> for ExtensionField {
    type Output = ExtensionField;

    fn mul(self, other: BaseField) -> ExtensionField {
        ExtensionField([
            self.0[0] * other,
            self.0[1] * other,
            self.0[2] * other,
            self.0[3] * other,
        ])
    }
}

impl Div<BaseField> for ExtensionField {
    type Output = ExtensionField;

    fn div(self, other: BaseField) -> ExtensionField {
        ExtensionField([
            self.0[0] / other,
            self.0[1] / other,
            self.0[2] / other,
            self.0[3] / other,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::basefield::PRIME;

    #[test]
    fn test_ops() {
        let qm0 = ExtensionField::new(1, 2, 3, 4);
        let qm1 = ExtensionField::new(4, 5, 6, 7);
        let m = BaseField::new(8);
        let qm = ExtensionField([m, BaseField(0), BaseField(0), BaseField(0)]);
        let qm0_x_qm1 = ExtensionField::new(PRIME - 71, 93, PRIME - 16, 50);

        assert_eq!(qm0 + qm1, ExtensionField::new(5, 7, 9, 11));
        assert_eq!(qm1 + m, qm1 + qm);
        assert_eq!(qm0 * qm1, qm0_x_qm1);
        assert_eq!(qm1 * m, qm1 * qm);
        assert_eq!(
            -qm0,
            ExtensionField::new(PRIME - 1, PRIME - 2, PRIME - 3, PRIME - 4)
        );
        assert_eq!(
            qm0 - qm1,
            ExtensionField::new(PRIME - 3, PRIME - 3, PRIME - 3, PRIME - 3)
        );
        assert_eq!(qm1 - m, qm1 - qm);
        assert_eq!(qm0_x_qm1 / qm1, ExtensionField::new(1, 2, 3, 4));
        assert_eq!(qm1 / m, qm1 / qm);
    }
}
