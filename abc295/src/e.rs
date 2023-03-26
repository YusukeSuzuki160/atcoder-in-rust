use std::vec::Vec;
use proconio::input;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign};
const MOD: u64 = 998244353;
fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64,
        a: [u64; n],
    }
    let mut binom = vec![vec![ModInteger::new(0, MOD); n as usize + 1]; n as usize + 1];
    for i in 0..=n {
        binom[i as usize][0] = ModInteger::new(1, MOD);
        for j in 1..=i {
            binom[i as usize][j as usize] = binom[i as usize - 1][j as usize] + binom[i as usize - 1][j as usize - 1];
        }
    }
    /*for i in 0..=n {
        for j in 0..=i {
            println!("{}C{} = {}", i, j, binom[i as usize][j as usize].value)
        }
    }*/
    let mut ans = ModInteger::new(0, MOD);
    for i in 1..=m {
        let mut rem = n as i64 + 1 - k as i64;
        let mut zero = 0;
        for j in &a {
            if *j >= i {
                rem -= 1;
            } else if *j == 0 {
                zero += 1;
            }
        }
        if rem < 0 {
            ans += ModInteger::new(1, MOD);
            continue;
        }
        if rem > zero {
            continue;
        }
        let p = ModInteger::new(m as u128+ 1 - i as u128, MOD) / ModInteger::new(m as u128, MOD);
        let mut p_pow = vec![ModInteger::new(1, MOD); zero as usize + 1];
        let mut q_pow = vec![ModInteger::new(1, MOD); zero as usize + 1];
        for j in 0..zero {
            p_pow[j as usize + 1] = p_pow[j as usize] * p;
            q_pow[j as usize + 1] = q_pow[j as usize] * (ModInteger::new(1, MOD) - p);
        }
        for j in rem..=zero {
            ans += binom[zero as usize][j as usize] * p_pow[j as usize] * q_pow[zero as usize - j as usize];
        }
    }
    println!("{}", ans.value);
}

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

impl std::fmt::Display for ModInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}