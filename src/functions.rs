fn main() {
    println!("Hello, World!");
    test_one();
    add_numbers(99, 99);

    let num = {
        let x = 3;
        x + 1
    };
    println!("{}", num);

    // let result = multiply_numbers(2, 3);
    // println!("{}", result);

    let result = divide_numbers(2, 3);
    println!("{}", result);
}

fn test_one() {
    println!("Test has been called..")
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result - 10;
    }
    result
}

fn multiply_numbers(x: i32, y: i32) -> i32 {
    x * y // no semi colon
}

fn divide_numbers(x: i32, y: i32) -> i32 {
    return x / y; // fine without semi colon also

    // let result = x/y;
    // result
}