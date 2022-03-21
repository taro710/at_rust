pub fn answer() {
    proconio::input! {
        n: i32,
    }

    if n % 2 == 0 {
        println!("White")
    } else if n % 2 == 1 {
        println!("Black")
    }
}
