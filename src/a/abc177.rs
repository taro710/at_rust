pub fn answer() {
    proconio::input! {
        d: i32,
        t: i32,
        s: i32,
    }

    if d > t * s {
        print!("No")
    } else if d <= t * s {
        println!("Yes")
    }
}
