pub fn answer() {
    proconio::input! {
        x: i32,
        y: i32,
    }

    if y % 2 == 0 && x * 2 <= y && y <= x * 4 {
        print!("Yes")
    } else {
        print!("No")
    }
}
