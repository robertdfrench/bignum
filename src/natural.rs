use crate::digit;

#[derive(Clone,Debug,PartialEq)]
struct Natural {
    digits: Vec<digit::Digit>
}

impl std::str::FromStr for Natural {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];
        for c in s.chars() {
            let d: digit::Digit = c.try_into()?;
            digits.insert(0, d)
        }
        match digits.len() {
            0 => Err("We cannot have a zero-digit number"),
            _ => Ok(Self{ digits })
        }
    }
}

impl std::fmt::Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

// We assume that the digits vector always has at least one digit in it
impl Natural {
    pub fn zero() -> Self {
        "0".parse().unwrap()
    }

    pub fn degree(&self) -> usize {
        self.digits.len() - 1
    }

    pub fn coefficient(&self, power: usize) -> digit::Digit {
        if power > self.degree() {
            return digit::Digit::Zero;
        }

        self.digits[power]
    }
}

impl std::ops::Add for Natural {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let n = std::cmp::max(self.degree(), other.degree()) + 1;
        let mut digits = vec![];
        let mut cs: digit::CarrySum = Default::default();
        for p in 0..n {
            let a = self.coefficient(p);
            let b = other.coefficient(p);
            cs = cs.add_two(a, b);
            digits.push(cs.sum.clone());
        }

        if cs.carry {
            digits.push(digit::Digit::One);
        }

        Self{ digits }
    }
}

impl std::ops::Mul for Natural {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut summands = vec![];

        for (i, a) in self.digits.iter().enumerate() {
            let mut cp: digit::CarryProduct = Default::default();
            let mut digits = vec![];
            for _ in 0..i {
                digits.push(digit::Digit::Zero);
            }
            for b in &other.digits {
                cp = cp.mul_two(*a, *b);
                digits.push(cp.product.clone());
            }
            if cp.carry != digit::Digit::Zero {
                digits.push(cp.carry);
            }
            summands.push(Self{ digits });
        }

        let mut total = Natural::zero();
        for summand in summands {
            total = total + summand;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ack() {
        let x: Natural = "123".parse().unwrap();
        assert_eq!(x.to_string(), "123");
    }

    #[test]
    fn degree() {
        let x: Natural = "123".parse().unwrap();
        assert_eq!(x.degree(), 2);
    }

    #[test]
    fn coefficient() {
        let x: Natural = "123".parse().unwrap();
        assert_eq!(x.coefficient(1), digit::Digit::Two);
    }

    #[test]
    fn add() {
        let x: Natural = "9".parse().unwrap();
        let y: Natural = "99".parse().unwrap();
        assert_eq!(x + y, "108".parse().unwrap());
    }

    #[test]
    fn add_large() {
        let x: Natural = "4294967295".parse().unwrap();
        let x64: u64 = 4294967295;
        let y: Natural = "8234567891".parse().unwrap();
        let y64: u64 = 8234567891;
        assert_eq!(format!("{}", (x + y)), format!("{}", x64 + y64));
    }

    #[test]
    fn mul() {
        let x: Natural = "9".parse().unwrap();
        let y: Natural = "99".parse().unwrap();
        assert_eq!(x * y, "891".parse().unwrap());
    }

    #[test]
    fn mul2() {
        let x: Natural = "2".parse().unwrap();
        let y: Natural = "22".parse().unwrap();
        assert_eq!(x * y, "44".parse().unwrap());
    }

    #[test]
    fn mul3() {
        let x: Natural = "3".parse().unwrap();
        let y: Natural = "45".parse().unwrap();
        assert_eq!(x * y, "135".parse().unwrap());
    }

    #[test]
    fn mul3r() {
        let x: Natural = "45".parse().unwrap();
        let y: Natural = "3".parse().unwrap();
        assert_eq!(x * y, "135".parse().unwrap());
    }

    #[test]
    fn mul4() {
        let x: Natural = "44".parse().unwrap();
        let y: Natural = "44".parse().unwrap();
        assert_eq!(x * y, "1936".parse().unwrap());
    }

    #[test]
    fn mul5() {
        let x: Natural = "1".parse().unwrap();
        let y: Natural = "1".parse().unwrap();
        assert_eq!(x * y, "1".parse().unwrap());
    }

    #[test]
    fn mul6() {
        let x: Natural = "9".parse().unwrap();
        let y: Natural = "1".parse().unwrap();
        assert_eq!(x * y, "9".parse().unwrap());
    }

    #[test]
    fn mul6r() {
        let x: Natural = "1".parse().unwrap();
        let y: Natural = "9".parse().unwrap();
        assert_eq!(x * y, "9".parse().unwrap());
    }

    #[test]
    fn mul7() {
        let x: Natural = "99".parse().unwrap();
        let y: Natural = "1".parse().unwrap();
        assert_eq!(x * y, "99".parse().unwrap());
    }

    #[test]
    fn mul7r() {
        let x: Natural = "1".parse().unwrap();
        let y: Natural = "99".parse().unwrap();
        assert_eq!(x * y, "99".parse().unwrap());
    }

    #[test]
    fn mul_large() {
        let two_21: Natural = "2097152".parse().unwrap();
        let two_19: Natural = "524288".parse().unwrap();
        let two_40: Natural = "1099511627776".parse().unwrap();
        assert_eq!(two_21 * two_19, two_40);
    }

    #[test]
    fn mul_repeat() {
        let two_1: Natural = "2".parse().unwrap();
        let two_2 = two_1.clone() * two_1.clone();
        let two_4 = two_2.clone() * two_2;
        let two_8 = two_4.clone() * two_4;
        let two_16 = two_8.clone() * two_8;
        let two_32 = two_16.clone() * two_16;
        let two_64 = two_32.clone() * two_32;
        let two_65 = two_64 * two_1;
        assert_eq!(&format!("{}", two_65), "36893488147419103232");
    }
}
