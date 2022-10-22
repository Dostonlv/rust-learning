use std::io;
fn main() {
    //let x: u8 = 9; u8 -> //0-255
    // let x: i8 = 9; //
    // let y: i8 = 10; // i8 ->  -128 - 127
    // if there is a remainder in the number to be, we declare a variable with a float
    // let x:f64=255.0;
    // let y:f64= 10.0;
    // let x = (i32::MAX as i64) +1;
    // let y=10_i32;
    // let z = x as i32/y;
    // println!("{}",z);


    // String to Number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");
    let int_input: i64 =input.trim().parse().unwrap();
    println!("{}",int_input+2);
}
