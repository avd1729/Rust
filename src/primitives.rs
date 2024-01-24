fn main(){

    let x:i32 = 2; // int
    let y:u32 = 45; // unsigned

    let z:f32 = 99.9; // default is f64 - double precision

    let a:bool = false;

    let b:char = ';';

    // Compound types

    // Tuples
    let tup = (1,2,'c',false,true);
    println!("{}",tup.2); // indexing

    let mut tup2 = (1,2,'c',false,true);
    tup2.0 = 'j';
    println!("{}",tup2.0);

    // Arrays
    let mut arr = [1,2,3,4,5];
    arr[0] = 9;
    println!("{}",arr[0]);

    let mut arr2:[i32;5];

}