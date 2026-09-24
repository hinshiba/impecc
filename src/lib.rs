pub mod point;
pub mod primefield;

#[cfg(test)]
mod tests {
    use crate::{
        point::{Curve, Point},
        primefield::PrimeFieldU32,
    };

    #[test]
    fn primefield_test() {
        assert_eq!(
            Into::<PrimeFieldU32<5>>::into(12),
            Into::<PrimeFieldU32<5>>::into(2)
        );
    }
}
