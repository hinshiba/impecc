//! 素体上の四則演算
use impecc::primefield::PrimeFieldU32;

fn main() {
    let (a, b) = (PrimeFieldU32::<7>::from(3), PrimeFieldU32::<7>::from(6));
    println!("3 + 6 ≡ {} (mod 7)", (a + b).value());
    println!("3 - 6 ≡ {} (mod 7)", (a - b).value());

    let (a, b) = (PrimeFieldU32::<11>::from(2), PrimeFieldU32::<11>::from(6));
    println!("2 × 6 ≡ {} (mod 11)", (a * b).value());

    let (a, b) = (PrimeFieldU32::<13>::from(2), PrimeFieldU32::<13>::from(6));
    println!("2 ÷ 6 ≡ {} (mod 13)", (a / b).value());
}
