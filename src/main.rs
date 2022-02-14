use std::io::stdin;

fn main() {
    proconio::input! {
        a:i32,
        b:i32,
        c:i32
    }

    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
