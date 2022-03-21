use std::f64::consts::PI;

pub fn answer() {
    proconio::input! {
        k: i32,
        a: i32,
        b: i32,
    }

    if (b / k * k) < a {
        print!("NG")
    } else if (b / k * k) >= a {
        print!("OK")
    }
}
