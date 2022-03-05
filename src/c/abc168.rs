use std::f64::consts::PI;

pub fn answer() {
    proconio::input! {
        a:f64,
        b:f64,
        h:f64,
        m:f64,
    }

    // 短針が1時間に進む角度 = 360 / 12 = 30 度
    // 短針が1分間に進む角度 = 360 / 12 / 60 = 0.5 度
    // 長針が1分間に進む角度 = 360 / 60 = 6 度

    // H時M分における 短針と長針のなす角度 = (30*H + 0.5*M) - 6*M

    let long = 30. * h + 0.5 * m;
    let short = 6. * m;
    let radian = (long - short) * PI / 180.;

    let ans_pow2 = a.powi(2) + b.powi(2) - 2. * a * b * radian.cos();

    println!("{}", ans_pow2.sqrt())
}
