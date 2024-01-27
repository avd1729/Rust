fn main() {
    //let cond = 2 < 3;
    //let cond = 2 <= 3.2;

    let cond1 = (2 as f32) <= 3.2;
    println!("{}", cond1);

    let cond2 = true && cond1;
    println!("{}", cond2);

    let cond3 = true || cond2;
    println!("{}", cond3);

    let cond4 = true || cond3;
    println!("{}", !cond4);

    // ! -> && -> ||

    let food = "cookie";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "marshmellow" {
        println!("Niceee")
    } else {
        println!("Oh , thats too bad!")
    }
}
