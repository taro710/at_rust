use std::f64::consts::PI;

pub fn answer() {
    proconio::input! {
        r:f64,
    }

    println!("{}", 2_f64 * r * PI)
}
