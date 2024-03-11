//! Single Digit Decimal Arithmetic

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq,PartialOrd,Ord)]
pub enum Digit {
    #[default]
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}


#[derive(Debug,Default,PartialEq,Eq)]
pub struct CarrySum {
    pub carry: bool,
    pub sum: Digit
}

impl CarrySum {
    fn new(carry: bool, sum: Digit) -> Self {
        Self { carry, sum }
    }

    pub fn add_two(&self, a: Digit, b: Digit) -> Self {
        let mut sum = a.as_u8() + b.as_u8();
        if self.carry {
            sum += 1;
        }
        // Sum can be no more than 19 at this point
        if sum <= 9 {
            Self::new(false, sum.try_into().unwrap())
        } else {
            Self::new(true, (sum - 10).try_into().unwrap())
        }
    }
}

impl std::ops::Add<Digit> for Digit {
    type Output = CarrySum;

    fn add(self, rhs: Self) -> Self::Output {
        let cs: CarrySum = Default::default();
        cs.add_two(self, rhs)
    }
}

#[derive(Debug,Default,PartialEq,Eq)]
pub struct CarryProduct {
    pub carry: Digit, // Less than 9
    pub product: Digit
}

impl CarryProduct {
    fn new(carry: Digit, product: Digit) -> Self {
        Self { carry, product }
    }

    pub fn mul_two(&self, a: Digit, b: Digit) -> Self {
        let mut product = a.as_u8() * b.as_u8();
        product += self.carry.as_u8();
        // Product can be no more than 89 at this point
        if product <= 9 {
            Self::new(Digit::Zero, product.try_into().unwrap())
        } else {
            let remainder: u8 = product % 10;
            let carry: u8 = (product - remainder) / 10;
            Self::new(carry.try_into().unwrap(), remainder.try_into().unwrap())
        }
    }
}

impl std::ops::Mul for Digit {
    type Output = CarryProduct;

    fn mul(self, rhs: Self) -> Self::Output {
        let cp: CarryProduct = Default::default();
        cp.mul_two(self, rhs)
    }
}

impl std::convert::TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err("not a digit")
        }
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl Digit {
    fn as_u8(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

impl std::convert::TryFrom<u8> for Digit {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            _ => Err("not a digit")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zero() {
        let zero: Digit = '0'.try_into().unwrap();
        assert_eq!(zero, Digit::Zero)
    }

    #[test]
    fn parse_00000000() {
        let zero: Digit = (0 as u8).try_into().unwrap();
        assert_eq!(zero, Digit::Zero)
    }

    #[test]
    fn ordered() {
        assert!(Digit::Zero < Digit::One)
    }

    #[test]
    fn can_add() {
        assert_eq!(Digit::Two + Digit::Three, CarrySum::new(false, Digit::Five));
    }

    #[test]
    fn can_carry() {
        assert_eq!(Digit::Nine + Digit::Three, CarrySum::new(true, Digit::Two));
    }

    #[test]
    fn can_mul() {
        assert_eq!(Digit::Two * Digit::Three, CarryProduct::new(Digit::Zero, Digit::Six));
    }

    #[test]
    fn can_mul0() {
        assert_eq!(Digit::Two * Digit::Zero, CarryProduct::new(Digit::Zero, Digit::Zero));
    }

    #[test]
    fn can_mul1() {
        assert_eq!(Digit::Two * Digit::One, CarryProduct::new(Digit::Zero, Digit::Two));
    }

    #[test]
    fn can_mul_carry() {
        assert_eq!(Digit::Six * Digit::Seven, CarryProduct::new(Digit::Four, Digit::Two));
    }
}
