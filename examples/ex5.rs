//! F_19 上の楕円曲線 y^2 = x^3 + 2 で P = (4, 3) の 2P, 3P
use impecc::point::{Curve, Point};

fn fmt_point(p: &Point<19>) -> String {
    match p.coord() {
        Some((x, y)) => format!("({}, {})", x.value(), y.value()),
        None => "O".to_string(),
    }
}

fn main() {
    let curve = Curve::<19>::new(0.into(), 2.into());
    let p = Point::new(4.into(), 3.into(), curve);
    println!(" P = {}", fmt_point(&p));
    println!("2P = {}", fmt_point(&(2 * p)));
    println!("3P = {}", fmt_point(&(3 * p)));
}
