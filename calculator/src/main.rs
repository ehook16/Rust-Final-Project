//This code is a simple calculator app that works through the command line. It reads the input of operations you want to perform (restricted to 2 numbers)
// and performs the input, printing the final answer. It utilizes borrowing and the scope rules of Rust. 
//Opterations can be performed on 32 bit floats and they work for positive or negative numbers.
//To start, we used the initial outline of the app in this tutorial: https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/ 
//but we changed it to function as a calculator.


fn main() {
    let first_num = std::env::args().nth(1).expect("Please specify a number"); //These statements read the input from the command line. To run, enter a statement of the format cargo run -- 4 + 3
    let operation = std::env::args().nth(2).expect("Please specify an operation");
    let second_num = std::env::args().nth(3).expect("Please specify another number");
    // let first_num = "4.5";
    // let operation = "-";
    // let second_num = "-8.3";

    if operation == "+" { //Based on the input, we perform one of five operations

        let sum1 = first_num.to_string();
        let float1: f32 = sum1.parse().unwrap();
        let sum2 = second_num.to_string();
        let float2: f32 = sum2.parse().unwrap();
        let sum = &float1 + &float2; //The & symbols signify borrowing. Have to borrow in this situation because exclusive ownership means we can't access the memory locations unless we borrow
        println!("{:?}+{:?}={:?}", float1, float2, sum);
    
    }

    if operation == "-" {
        let dif1 = first_num.to_string();
        let float1: f32 = dif1.parse().unwrap();
        let dif2 = second_num.to_string();
        let float2: f32 = dif2.parse().unwrap();
        let dif = &float1 - &float2; 
        println!("{:?}-{:?}={:?}", float1, float2, dif);
    }

    if operation == "*" {
        let mult1 = first_num.to_string();
        let float2: f32 = mult1.parse().unwrap();
        let mult2 = second_num.to_string();
        let float1: f32 = mult2.parse().unwrap();
        let mult = &float1 ** &float2;
        println!("{:?}*{:?}={:?}", float1, float2, mult);
    }

    if operation == "/" {
        let quot1 = first_num.to_string();
        let float1: f32 = quot1.parse().unwrap();
        let quot2 = second_num.to_string();
        let float2: f32 = quot2.parse().unwrap();
        let quot = &float1 / &float2; 
        println!("{:?}/{:?}={:?}", float1, float2, quot);
    }

    if operation == "^" {
        let pow1 = first_num.to_string();
        let float1: f32 = pow1.parse().unwrap();
        let pow2 = second_num.to_string();
        let float2: f32 = pow2.parse().unwrap();
        let pow = f32::powf(float1, float2); //powf built in function in Rust that calculates the exponent
        println!("{:?}^{:?}={:?}", float1, float2, pow);
    }

    
}
