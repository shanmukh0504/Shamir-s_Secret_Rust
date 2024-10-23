#[derive(Debug)]
pub struct Fraction {
    pub num: i128,
    pub den: i128,
}

impl Fraction {
    pub fn new(n: i128, d: i128) -> Self {
        Self { num: n, den: d }.reduce()
    }

    fn reduce(self) -> Self {
        let gcd = gcd(self.num, self.den);
        Self {
            num: self.num / gcd,
            den: self.den / gcd,
        }
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den).reduce()
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + self.den * other.num,
            self.den * other.den,
        )
        .reduce()
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_add() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);
        let result = a + b;
        assert_eq!(result.num, 5);
        assert_eq!(result.den, 6);
    }

    #[test]
    fn test_fraction_mul() {
        let a = Fraction::new(2, 3);
        let b = Fraction::new(3, 4);
        let result = a * b;
        assert_eq!(result.num, 1);
        assert_eq!(result.den, 2);
    }
}
