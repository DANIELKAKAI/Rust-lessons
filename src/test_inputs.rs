use std::io;

pub fn test_inputs() {
    let mut input1 = String::new();

    println!("Enter first number:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    let num1: i32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Not a valid integer!");
            return;
        }
    };

    println!("Enter second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let num2: i32 = match input2.trim().parse() {
        Ok(num2) => num2,
        Err(_) => {
            println!("Error: Not a valid integer!");
            return;
        }
    };

    let result = num1 / num2;

    println!("{}/{} is {}", num1, num2, result);
}
