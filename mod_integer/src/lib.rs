use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct ModInteger {
    pub value: u128,
    pub mod_num: u64,
}

impl ModInteger {
    pub fn new(rawvalue: u128, mod_num: u64) -> Self {
        let value = rawvalue % mod_num as u128;
        Self { value, mod_num }
    }


    pub fn pow(&self, n: u64) -> Self {
        let mut x = self.value;
        let mut y = 1;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                y = (y * x) % self.mod_num as u128;
            }
            x = (x * x) % self.mod_num as u128;
            n >>= 1;
        }
        Self::new(y, self.mod_num)
    }

    fn inv(&self) -> Self {
        self.pow(self.mod_num - 2)
    }

    pub fn set_value(&mut self, value: u128) {
        self.value = value % self.mod_num as u128;
    }
}

impl Add for ModInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value, self.mod_num)
    }
}

impl AddAssign for ModInteger {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % self.mod_num as u128;
    }
}

impl Sub for ModInteger {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut value = self.value;
        while value < rhs.value {
            value += self.mod_num as u128;
        }
        Self::new(value - rhs.value, self.mod_num)
    }
}

impl SubAssign for ModInteger {
    fn sub_assign(&mut self, rhs: Self) {
        let mut value = self.value;
        while value < rhs.value {
            value += self.mod_num as u128;
        }
        self.value = (value - rhs.value) % self.mod_num as u128;
    }
}

impl Mul for ModInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value, self.mod_num)
    }
}

impl MulAssign for ModInteger {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value) % self.mod_num as u128
    }
}

impl Div for ModInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl DivAssign for ModInteger {
    fn div_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.inv().value) % self.mod_num as u128;
    }
}

impl PartialEq for ModInteger {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ModInteger {}

impl PartialOrd for ModInteger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl std::fmt::Display for ModInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_integer() {
        let a = ModInteger::new(6, 5);
        let b = ModInteger::new(3, 5);
        assert_eq!(a + b, ModInteger::new(4, 5));
        assert_eq!(a - b, ModInteger::new(3, 5));
        assert_eq!(a * b, ModInteger::new(3, 5));
        assert_eq!(a / b, ModInteger::new(2, 5));
        assert_eq!(a.pow(2), ModInteger::new(1, 5));
        assert_eq!(a.pow(3), ModInteger::new(1, 5));
        assert_eq!(a < b, true);
        let c = ModInteger::new(3, 5);
        assert_eq!(b == c, true);
    }
}
