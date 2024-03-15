use crate::digit;
use std::cmp::Ordering;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Natural {
    digits: Vec<digit::Digit>
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.degree() < other.degree() {
            return Some(Ordering::Less);
        }
        if self.degree() > other.degree() {
            return Some(Ordering::Greater);
        }

        for p in (0..self.degree()+1).rev() {
            if self.coefficient(p) < other.coefficient(p) {
                return Some(Ordering::Less);
            }
            if self.coefficient(p) > other.coefficient(p) {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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

    pub fn one() -> Self {
        "1".parse().unwrap()
    }

    pub fn increment(&mut self) {
        *self += Self::one();
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

    pub fn set_coefficient(&mut self, power: usize, coefficient: digit::Digit) {
        self.digits[power] = coefficient;
    }
}

impl std::ops::AddAssign for Natural {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
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

impl std::ops::Sub for Natural {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        let n = std::cmp::max(self.degree(), other.degree()) + 1;
        let mut digits = vec![];
        for p in 0..n {
            let a = self.coefficient(p);
            let b = other.coefficient(p);
            let bd = a - b;
            if bd.borrow {
                // We need to borrow
                let mut pp = p;
                loop {
                    pp += 1;
                    let c = self.coefficient(pp);
                    if c == digit::Digit::Zero {
                        continue;
                    }
                    self.set_coefficient(pp, (c - digit::Digit::One).difference);
                    break;
                }
                // Unwind the borrow -- All zeros will become nines.
                for ppp in (p+1)..pp {
                    self.set_coefficient(ppp, digit::Digit::Nine);
                }
                digits.push(bd.difference);
            } else {
                let difference = (a - b).difference;
                digits.push(difference);
            }
        }

        // Remove leading zeros
        loop {
            if digits.len() > 1 && digits[digits.len() - 1] == digit::Digit::Zero {
                digits.pop();
            } else {
                break;
            }
        }

        Self::Output{ digits }
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

impl std::ops::Div for Natural {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let mut n = Natural::one();
        let mut product = other.clone();
        while self >= product {
            n.increment();
            product += other.clone();
        }
        n - Natural::one()
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
        let x: Natural = "3".parse().unwrap();
        let y: Natural = "45".parse().unwrap();
        assert_eq!(x * y, "135".parse().unwrap());
    }

    #[test]
    fn mul_symmetric() {
        let x: Natural = "45".parse().unwrap();
        let y: Natural = "3".parse().unwrap();
        assert_eq!(x * y, "135".parse().unwrap());
    }

    #[test]
    fn mul_80bit() {
        let two_1: Natural = "2".parse().unwrap();
        let two_2 = two_1.clone() * two_1.clone();
        let two_4 = two_2.clone() * two_2;
        let two_8 = two_4.clone() * two_4;
        let two_16 = two_8.clone() * two_8;
        let two_32 = two_16.clone() * two_16.clone();
        let two_64 = two_32.clone() * two_32;
        let two_80 = two_64 * two_16;
        assert_eq!(&format!("{}", two_80), "1208925819614629174706176");
    }

    #[test]
    fn equal() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "1099511627776".parse().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn unequal() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "1099512627776".parse().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn ordered_tiny() {
        let a: Natural = "10".parse().unwrap();
        let b: Natural = "9".parse().unwrap();
        assert!(a > b);
    }

    #[test]
    fn ordered_small() {
        let a: Natural = "11".parse().unwrap();
        let b: Natural = "10".parse().unwrap();
        assert!(a > b);
    }

    #[test]
    fn ordered_large() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "1099511626000".parse().unwrap();
        assert!(a > b);
    }

    #[test]
    fn subtract() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "1099511626000".parse().unwrap();
        assert_eq!(a - b, "1776".parse().unwrap());
    }

    #[test]
    fn increment() {
        let mut a = Natural::zero();
        a.increment();
        assert_eq!(a, Natural::one());
    }

    #[test]
    fn div() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "2199023255552".parse().unwrap();
        assert_eq!(b / a, "2".parse().unwrap());
    }

    #[test]
    fn div2() {
        let a: Natural = "1099511627776".parse().unwrap();
        let b: Natural = "2199023255552".parse().unwrap();
        assert_eq!(a / b, "0".parse().unwrap());
    }

    #[test]
    fn div3() {
        let a: Natural = "16".parse().unwrap();
        let b: Natural = "5".parse().unwrap();
        assert_eq!(a / b, "3".parse().unwrap());
    }
}
