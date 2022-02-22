pub fn answer() {
    proconio::input! {
        a:u32,
        b:u32,
    }

    let energy_difference = 32i32.pow(a - b);

    println!("{}", energy_difference);
}
