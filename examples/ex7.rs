use std::collections::HashMap;

use impecc::point::{Curve, Point};
use impecc::primefield::PrimeFieldU32;

/// R = aP + bQ を，R の x 座標を 3 で割った余りで 3 通りに更新する (無限遠点は 0 番目とみなす)
fn walk(
    r: Point<19>,
    a: PrimeFieldU32<13>,
    b: PrimeFieldU32<13>,
    p: Point<19>,
    q: Point<19>,
) -> (Point<19>, PrimeFieldU32<13>, PrimeFieldU32<13>) {
    match r.coord().map_or(0, |(x, _)| x.value() % 2) {
        0 => (r + p, a + 1.into(), b),
        _ => (r + q, a, b + 1.into()),
    }
}

fn main() {
    let curve = Curve::<19>::new(0.into(), 2.into());

    let p = Point::new(4.into(), 3.into(), curve);
    let q = Point::new(8.into(), 1.into(), curve);

    let w0 = 2 * p + q;
    let w1 = p + 2 * q;
    println!("{w0}, {w1}");

    // Q = [s]P となるsをrho法で求める
    let (mut a, mut b) = (PrimeFieldU32::<13>::from(2), PrimeFieldU32::<13>::from(4));
    let mut r = 2 * p + 4 * q;
    let mut map = HashMap::new();
    let s = loop {
        let Some(c) = r.coord() else {
            // aP + bQ = O => a + bs ≡ 0 (mod 13)
            if b == 0.into() {
                panic!("b = 0 で無限遠点に到達し，s を復元できない");
            }
            break (PrimeFieldU32::<13>::from(0) - a) / b;
        };
        
        let key = (c.0.value(), c.1.value());
        if let Some(&(a2, b2)) = map.get(&key) {
            // 衝突
            // 同じ R = aP + bQ = a2*P + b2*Q に到達 => (a - a2) = (b2 - b)s (mod 13)
            if b2 - b == 0.into() {
                panic!("db = 0 で s を復元できない衝突だった");
            }
            break (a - a2) / (b2 - b);
        }
        map.insert(key, (a, b));
        (r, a, b) = walk(r, a, b, p, q);
    };

    assert_eq!(q, p * s.value());
    println!(
        "s = {}, Q = [{}]P = {}",
        s.value(),
        s.value(),
        p * s.value()
    );
}
