//! F_19 における 0..18 の平方根
use impecc::primefield::PrimeFieldU32;

type F19 = PrimeFieldU32<19>;

fn main() {
    for a in 0..19 {
        let roots = F19::from(a).sqrt();
        let roots = if roots.is_empty() {
            "なし".to_string()
        } else {
            roots
                .iter()
                .map(|r| r.value().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };
        println!("√{:>2} = {}", a, roots);
    }
}
