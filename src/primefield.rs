use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrimeFieldU32<const ORDER: u32> {
    val: u32,
}

impl<const ORDER: u32> From<u32> for PrimeFieldU32<ORDER> {
    fn from(value: u32) -> Self {
        PrimeFieldU32 { val: value % ORDER }
    }
}

impl<const ORDER: u32> AddAssign for PrimeFieldU32<ORDER> {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % ORDER;
    }
}

impl<const ORDER: u32> Add for PrimeFieldU32<ORDER> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const ORDER: u32> SubAssign for PrimeFieldU32<ORDER> {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val + ORDER - rhs.val) % ORDER;
    }
}

impl<const ORDER: u32> Sub for PrimeFieldU32<ORDER> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const ORDER: u32> MulAssign for PrimeFieldU32<ORDER> {
    fn mul_assign(&mut self, rhs: Self) {
        // u32 同士の積はオーバーフローしうるため u64 で計算
        self.val = ((self.val as u64 * rhs.val as u64) % ORDER as u64) as u32;
    }
}

impl<const ORDER: u32> Mul for PrimeFieldU32<ORDER> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const ORDER: u32> PrimeFieldU32<ORDER> {
    /// 代表元 (0..ORDER)
    pub const fn value(&self) -> u32 {
        self.val
    }

    /// 平方根を昇順で返す．非平方剰余なら空
    /// 総当りであることに注意せよ
    pub fn sqrt(&self) -> Vec<Self> {
        // 総当たり
        (0..ORDER)
            .map(Self::from)
            .filter(|&x| x * x == *self)
            .collect()
    }

    /// 拡張ユークリッド互除法による乗法逆元．0 (および ORDER と互いに素でない値) は None
    pub const fn inv(&self) -> Option<Self> {
        // 不変条件: t0 * val ≡ r0, t1 * val ≡ r1 (mod ORDER)
        let (mut r0, mut r1) = (ORDER as i64, self.val as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        // r0 = gcd(ORDER, val)
        if r0 != 1 {
            return None;
        }
        Some(PrimeFieldU32 {
            val: t0.rem_euclid(ORDER as i64) as u32,
        })
    }
}

impl<const ORDER: u32> DivAssign for PrimeFieldU32<ORDER> {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().expect("division by zero");
    }
}

impl<const ORDER: u32> Div for PrimeFieldU32<ORDER> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const ORDER: u32> fmt::Display for PrimeFieldU32<ORDER> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] (F_{})", self.val, ORDER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_inv<const P: u32>(a: u32) {
        let x = PrimeFieldU32::<P>::from(a);
        let mut prod = x;
        prod *= x.inv().unwrap();
        assert_eq!(prod, PrimeFieldU32::from(1));
    }

    #[test]
    fn inv_small_field_exhaustive() {
        for a in 1..13 {
            check_inv::<13>(a);
        }
    }

    #[test]
    fn inv_large_field() {
        // u32 に収まる最大の素数
        const P: u32 = 4_294_967_291;
        for a in [1, 2, 3, 12345, P / 2, P - 2, P - 1] {
            check_inv::<P>(a);
        }
    }

    #[test]
    fn inv_zero_is_none() {
        assert_eq!(PrimeFieldU32::<13>::from(0).inv(), None);
    }

    #[test]
    fn div_roundtrip() {
        let a = PrimeFieldU32::<13>::from(5);
        let b = PrimeFieldU32::<13>::from(7);
        let mut back = a / b;
        back *= b;
        assert_eq!(back, a);
    }

    #[test]
    fn sqrt_small_field_exhaustive() {
        // 0 と (19 - 1) / 2 = 9 個の平方剰余
        let residues = (0..19)
            .map(PrimeFieldU32::<19>::from)
            .filter(|a| !a.sqrt().is_empty())
            .count();
        assert_eq!(residues, 10);
        for a in 0..19 {
            let a = PrimeFieldU32::<19>::from(a);
            for r in a.sqrt() {
                assert_eq!(r * r, a);
            }
        }
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn div_by_zero_panics() {
        let _ = PrimeFieldU32::<13>::from(1) / PrimeFieldU32::from(0);
    }
}
