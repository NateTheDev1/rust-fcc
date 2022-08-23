use std::{env::{args, Args}};

fn main() {
    let mut args: Args = args();

    let first: String = args.nth(1).unwrap();
    let operator : char = args.nth(0).unwrap().chars().next().unwrap();
    let second : String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}


fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    
    // implicit returns ?? This is cool (can also just use return and semicolon)

    // if operator == '+' {
    //      first_number + second_number
    // } else if operator == '-' {
    //      first_number - second_number
    // } else if operator == '/' {
    //      first_number / second_number
    // } else if operator == '*' {
    //      first_number * second_number
    // } else {
    //     0.0
    // }

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' | 'd' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator was used.")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
}