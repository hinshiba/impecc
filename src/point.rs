use core::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use crate::primefield::PrimeFieldU32;

/// 短 Weierstrass 形式 y^2 = x^3 + ax + b．標数 2, 3 の体は想定しない
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Curve<const ORDER: u32> {
    a: PrimeFieldU32<ORDER>,
    b: PrimeFieldU32<ORDER>,
}

impl<const ORDER: u32> Curve<ORDER> {
    pub fn new(a: PrimeFieldU32<ORDER>, b: PrimeFieldU32<ORDER>) -> Self {
        // 判別式が 0 だと特異点(尖点・結節点)を持ち，群にならない
        let disc = PrimeFieldU32::from(4) * a * a * a + PrimeFieldU32::from(27) * b * b;
        if disc == PrimeFieldU32::from(0) {
            panic!("y^2 = x^3 + {}x + {} is singular", a, b)
        }
        Self { a, b }
    }

    pub fn on_curve(&self, x: PrimeFieldU32<ORDER>, y: PrimeFieldU32<ORDER>) -> bool {
        y * y == x * x * x + self.a * x + self.b
    }

    pub fn a(&self) -> PrimeFieldU32<ORDER> {
        self.a
    }

    pub fn b(&self) -> PrimeFieldU32<ORDER> {
        self.b
    }
}

impl<const ORDER: u32> fmt::Display for Curve<ORDER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "y^2 = x^3 + {}x + {}", self.a, self.b)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point<const ORDER: u32> {
    /// None は無限遠点 O (群の単位元)
    coord: Option<(PrimeFieldU32<ORDER>, PrimeFieldU32<ORDER>)>,
    curve: Curve<ORDER>,
}

impl<const ORDER: u32> Point<ORDER> {
    pub fn new(x: PrimeFieldU32<ORDER>, y: PrimeFieldU32<ORDER>, curve: Curve<ORDER>) -> Self {
        if !curve.on_curve(x, y) {
            panic!(
                "({}, {}) is not on y^2 = x^3 + {}x + {}",
                x, y, curve.a, curve.b
            )
        }
        Self {
            coord: Some((x, y)),
            curve,
        }
    }

    pub fn infinity(curve: Curve<ORDER>) -> Self {
        Self { coord: None, curve }
    }

    pub fn is_infinity(&self) -> bool {
        self.coord.is_none()
    }

    /// 無限遠点なら None
    pub fn coord(&self) -> Option<(PrimeFieldU32<ORDER>, PrimeFieldU32<ORDER>)> {
        self.coord
    }

    pub fn curve(&self) -> Curve<ORDER> {
        self.curve
    }
}

impl<const ORDER: u32> Neg for Point<ORDER> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if let Some((x, y)) = self.coord {
            self.coord = Some((x, PrimeFieldU32::from(0) - y));
        }
        self
    }
}

impl<const ORDER: u32> AddAssign for Point<ORDER> {
    fn add_assign(&mut self, rhs: Self) {
        if self.curve != rhs.curve {
            panic!(
                "cannot add points on different curves: {} and {}",
                self.curve, rhs.curve
            )
        }
        let (Some((x1, y1)), Some((x2, y2))) = (self.coord, rhs.coord) else {
            // どちらかが無限遠点
            if self.is_infinity() {
                *self = rhs;
            }
            return;
        };
        let lambda = if x1 != x2 {
            // ECA
            // 2 点を通る直線の傾き
            (y2 - y1) / (x2 - x1)
        } else if y1 == y2 && y1 != PrimeFieldU32::from(0) {
            // ECD
            // 接線の傾き
            (PrimeFieldU32::from(3) * x1 * x1 + self.curve.a) / (PrimeFieldU32::from(2) * y1)
        } else {
            // P + (-P)，または y = 0 の点の 2 倍
            self.coord = None;
            return;
        };
        let x3 = lambda * lambda - x1 - x2;
        let y3 = lambda * (x1 - x3) - y1;
        self.coord = Some((x3, y3));
    }
}

impl<const ORDER: u32> Add for Point<ORDER> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const ORDER: u32> MulAssign<u32> for Point<ORDER> {
    fn mul_assign(&mut self, rhs: u32) {
        // rhsをbit列と見て倍にしながら加算していく
        let mut acc = Self::infinity(self.curve);
        let mut base = *self;
        let mut k = rhs;
        while k > 0 {
            if k & 1 == 1 {
                acc += base;
            }
            base = base + base;
            k >>= 1;
        }
        *self = acc;
    }
}

impl<const ORDER: u32> Mul<u32> for Point<ORDER> {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const ORDER: u32> Mul<Point<ORDER>> for u32 {
    type Output = Point<ORDER>;

    fn mul(self, rhs: Point<ORDER>) -> Self::Output {
        rhs * self
    }
}

impl<const ORDER: u32> fmt::Display for Point<ORDER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.coord {
            Some((x, y)) => write!(f, "({}, {})", x, y),
            None => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // y^2 = x^3 + 2x + 2 over F_17，G = (5, 1) の位数は 19
    type F = PrimeFieldU32<17>;

    fn curve() -> Curve<17> {
        Curve::new(F::from(2), F::from(2))
    }

    fn pt(x: u32, y: u32) -> Point<17> {
        Point::new(F::from(x), F::from(y), curve())
    }

    #[test]
    fn known_multiples() {
        let g = pt(5, 1);
        assert_eq!(g + g, pt(6, 3));
        assert_eq!(2 * g, pt(6, 3));
        assert_eq!(g * 3, pt(10, 6));
        assert_eq!(g * 18, pt(5, 16));
        assert!((g * 19).is_infinity());
    }

    #[test]
    fn inverse_sums_to_infinity() {
        let g = pt(5, 1);
        assert!((g + -g).is_infinity());
    }

    #[test]
    fn identity() {
        let g = pt(5, 1);
        let o = Point::infinity(curve());
        assert_eq!(g + o, g);
        assert_eq!(o + g, g);
        assert_eq!(o + o, o);
    }

    #[test]
    fn scalar_mul_matches_repeated_add() {
        let g = pt(5, 1);
        let mut acc = Point::infinity(curve());
        for k in 0..40 {
            assert_eq!(g * k, acc, "k = {}", k);
            if let Some((x, y)) = acc.coord() {
                assert!(curve().on_curve(x, y));
            }
            acc += g;
        }
    }

    #[test]
    fn doubling_point_with_zero_y() {
        // y^2 = x^3 + x over F_17 で (0, 0) は位数 2
        let c = Curve::new(F::from(1), F::from(0));
        let p = Point::new(F::from(0), F::from(0), c);
        assert!((p + p).is_infinity());
    }

    #[test]
    #[should_panic(expected = "is not on")]
    fn not_on_curve_panics() {
        pt(5, 2);
    }

    #[test]
    #[should_panic(expected = "singular")]
    fn singular_curve_panics() {
        Curve::new(F::from(0), F::from(0));
    }
}
