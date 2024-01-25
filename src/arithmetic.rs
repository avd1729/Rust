use std::io;

fn main() {
    let x: u8 = 55; // 0 - 255
    let y: u8 = 6; // -128 - 127

    let z = x + y;
    println!("{}", z); // 61

    let a = x / y;
    println!("{}", a); // 9

    let b: f32 = 55.0;
    let c: f32 = 6.0;

    let d = b / c;
    println!("{}", d); // 9.166667

    let e = b % c;
    println!("{}", e); // 1

    // Type casting

    let _f = 56.0f32;
    let _g = 10u8;

    let _h = 127 as i64;

    let i = _h / (_g as i64);
    println!("{}", i); // 12

    // String to int

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("!!");

    let input2: i64 = input.trim().parse().unwrap();

    println!("{}", input);
    println!("{}", input2 + 2);
}