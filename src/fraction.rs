use num_bigint::BigInt;
use num_traits::Zero;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Fraction {
    pub num: BigInt,
    pub den: BigInt,
}

impl Fraction {
    /// Creates a new Fraction and reduces it immediately.
    pub fn new(n: BigInt, d: BigInt) -> Self {
        if d.is_zero() {
            panic!("Denominator cannot be zero.");
        }
        Self { num: n, den: d }.reduce()
    }

    /// Reduces the fraction by dividing both the numerator and denominator by their GCD.
    fn reduce(self) -> Self {
        let gcd_value = gcd(&self.num, &self.den);
        let num = &self.num / &gcd_value;
        let den = &self.den / &gcd_value;

        // Ensure the denominator is always positive
        if den < BigInt::zero() {
            Self {
                num: -num,
                den: -den,
            }
        } else {
            Self { num, den }
        }
    }
}

/// Implementation of multiplication for fractions.
impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den).reduce()
    }
}

/// Implementation of addition for fractions.
impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den.clone() + self.den.clone() * other.num,
            self.den * other.den,
        )
        .reduce()
    }
}

/// GCD function for BigInt. This function operates using the Euclidean algorithm.
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();

    while !b.is_zero() {
        let temp = b.clone();
        b = a % temp.clone();
        a = temp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn test_fraction_add() {
        let a = Fraction::new(BigInt::from(1), BigInt::from(2));
        let b = Fraction::new(BigInt::from(1), BigInt::from(3));
        let result = a + b;
        assert_eq!(result.num, BigInt::from(5));
        assert_eq!(result.den, BigInt::from(6));
    }

    #[test]
    fn test_fraction_mul() {
        let a = Fraction::new(BigInt::from(2), BigInt::from(3));
        let b = Fraction::new(BigInt::from(3), BigInt::from(4));
        let result = a * b;
        assert_eq!(result.num, BigInt::from(1));
        assert_eq!(result.den, BigInt::from(2));
    }
}
