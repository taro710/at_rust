pub fn answer() {
    proconio::input! {
        sx:f64,
        sy:f64,
        gx:f64,
        gy:f64,
    }

    println!("{}", (sy * gx + sx * gy) / (sy + gy))
}
