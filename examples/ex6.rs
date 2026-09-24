//! F_19 上の楕円曲線 y^2 = x^3 + 8 で x 座標が 1 の点 P, Q (y が小さい方が P) と 19P
use impecc::point::{Curve, Point};

fn fmt_point(p: &Point<19>) -> String {
    match p.coord() {
        Some((x, y)) => format!("({}, {})", x.value(), y.value()),
        None => "O".to_string(),
    }
}

fn main() {
    let curve = Curve::<19>::new(0.into(), 8.into());
    // y 昇順で返る
    let [p, q] = curve.points_with_x(1.into())[..] else {
        panic!("x = 1 の点がちょうど 2 個ではない")
    };
    println!("  P = {}", fmt_point(&p));
    println!("  Q = {}", fmt_point(&q));
    println!("19P = {}", fmt_point(&(19 * p)));
}
