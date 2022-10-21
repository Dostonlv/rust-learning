use std::io;
fn main() {
    // test()
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let x: i32 = input_line.trim().parse().expect("Input not an integer");
    let y: i32 = input.trim().parse().expect("Input not an integer");
    add_numbers(x, y);
}
// fn test(){
//     println!("Test");
// }
fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}
