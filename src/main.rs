fn main() {

    let mut x:i32 = 4;
    println!("x is: {}",x);

    // Name shadowing

    {
        // x = 2 only in this child scope
        let x = 2;
        println!("x is: {}",x);
    }

    {
        // child scope can access parent scope
        let x = x-1;
        println!("x is: {}",x);
    }

    x = x+1;
    println!("x is: {}",x);

    let y = 3;
    println!("y is: {}",y);

    let y = 4;
    println!("y is: {}",y);

    let y = "Hello, world!"; // we can reinitialize vars in other data types
    println!("{}",y);

    const SECONDS_IN_MINUTE:u32 = 60;
    //const SECONDS_IN_MINUTE:u32 = 50;
    println!("{}",SECONDS_IN_MINUTE);

}
