pub fn answer() {
    proconio::input! {
        n:i32,
        x:i32,
        t:i32,
    }

    if n % x == 0 {
        println!("{}", n / x * t);
    } else {
        println!("{}", (n / x + 1) * t);
    }
}
