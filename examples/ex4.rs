//! F_19 上の楕円曲線 y^2 = x^3 + 2 の全点
use impecc::point::{Curve, Point};

fn fmt_point(p: &Point<19>) -> String {
    match p.coord() {
        Some((x, y)) => format!("({}, {})", x.value(), y.value()),
        None => "O".to_string(),
    }
}

fn main() {
    let curve = Curve::<19>::new(0.into(), 2.into());
    let points = curve.points();
    for p in &points {
        println!("{}", fmt_point(p));
    }
    println!("{}", fmt_point(&Point::infinity(curve)));
    println!("#E = {}", points.len() + 1);
}
