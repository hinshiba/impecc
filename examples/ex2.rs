//! F_19 における 1..18 の平方
use impecc::primefield::PrimeFieldU32;

type F19 = PrimeFieldU32<19>;

fn main() {
    for a in 1..19 {
        let x = F19::from(a);
        println!("{:>2}^2 ≡ {:>2} (mod 19)", a, (x * x).value());
    }
}
