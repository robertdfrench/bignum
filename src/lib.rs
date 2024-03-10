mod single;

#[derive(Debug,PartialEq)]
struct Number {
    digits: Vec<single::Digit>
}

impl std::str::FromStr for Number {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];
        for c in s.chars() {
            let d: single::Digit = c.try_into()?;
            digits.insert(0, d)
        }
        match digits.len() {
            0 => Err("We cannot have a zero-digit number"),
            _ => Ok(Self{ digits })
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

// We assume that the digits vector always has at least one digit in it
impl Number {
    pub fn degree(&self) -> usize {
        self.digits.len() - 1
    }

    pub fn coefficient(&self, power: usize) -> single::Digit {
        if power > self.degree() {
            return single::Digit::Zero;
        }

        self.digits[power]
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        let n = std::cmp::max(self.degree(), other.degree()) + 1;
        let mut digits = vec![];
        let mut cs: single::CarrySum = Default::default();
        for p in 0..n {
            let a = self.coefficient(p);
            let b = other.coefficient(p);
            cs = cs.add_two(a, b);
            digits.push(cs.sum.clone());
        }

        if cs.carry {
            digits.push(single::Digit::One);
        }

        Self{ digits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ack() {
        let x: Number = "123".parse().unwrap();
        assert_eq!(x.to_string(), "123");
    }

    #[test]
    fn degree() {
        let x: Number = "123".parse().unwrap();
        assert_eq!(x.degree(), 2);
    }

    #[test]
    fn coefficient() {
        let x: Number = "123".parse().unwrap();
        assert_eq!(x.coefficient(1), single::Digit::Two);
    }

    #[test]
    fn add() {
        let x: Number = "9".parse().unwrap();
        let y: Number = "99".parse().unwrap();
        assert_eq!(x + y, "108".parse().unwrap());
    }

    #[test]
    fn add_large() {
        let x: Number = "4294967295".parse().unwrap();
        let x64: u64 = 4294967295;
        let y: Number = "8234567891".parse().unwrap();
        let y64: u64 = 8234567891;
        assert_eq!(format!("{}", (x + y)), format!("{}", x64 + y64));
    }
}
