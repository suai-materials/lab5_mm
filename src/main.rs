use rand::random;
use std::io::repeat;

fn main() {
    let p_a = 0.86;
    let p_b = 0.75;
    let p_c = 0.8;
    let p_d = 0.85;
    let p_e = 0.9;

    let p = (1. - (1. - p_a) * (1. - p_b)) * (1. - (1. - p_c) * (1. - p_d) * (1. - p_e));
    println!("Аналитически найденная вероятность: {}", p);

    let n = 70;
    let (mut a, mut b, mut c, mut d, mut e) = (1., 1., 1., 1., 1.);
    let mut good: i32 = 0;

    for i in 1..=n {
        (a, b, c, d, e) = (random(), random(), random(), random(), random());
        println!("Испытание №{i}: A = {a}, B = {b}, C={c}, D={d}, E={e}");
        if a <= p_a || b <= p_b {
            println!("Первый блок: ✓");
        } else {
            println!("Первый блок: ❌");
        }
        if c <= p_c || d <= p_d || e <= p_e {
            println!("Второй блок: ✓");
        } else {
            println!("Второй блок: ❌");
        }
        if (a <= p_a || b <= p_b) && (c <= p_c || d <= p_d || e <= p_e) {
            println!("Цепь: ✓");
            good += 1;
        } else {
            println!("Цель: ❌");
        }
    }
    println!("Аналитически найденная вероятность: {}", p);
    println!("P* = {}", f64::from(good) / f64::from(n));
    println!("|P - P*| = {}", (p - f64::from(good) / f64::from(n)).abs());
}
