// Learn functions in rust

use std::io;
use std::io::Write;
use std::process;

fn main(){
    // Set the variables up

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let x = get_num_input();

    print!("Enter another number: ");
    io::stdout().flush().unwrap();
    let y = get_num_input();

    print!("Enter an operator (+ - / *): ");
    io::stdout().flush().unwrap();
    let op = get_op_input();

    let (sumop, difop, quotop, prodop) = ("+\n".to_string(), "-\n".to_string(), "/\n".to_string(), "*\n".to_string());

    if op == sumop {
        let sum = add(x, y);
        println!("The sum of {0} and {1} is: {2}", x, y, sum);
    } else if op == difop {
        let dif = sub(x, y);
        println!("The differance of {0} and {1} is: {2}", x, y, dif);
    } else if op == quotop {
        let quot = div(x, y);
        println!("The quotient of {0} and {1} is: {2}", x, y, quot);
    } else if op == prodop {
        let prod = mul(x, y);
        println!("The product of {0} and {1} is: {2}", x, y, prod);
    } else {
        eprintln!("Invalid operator.");
        process::exit(1);
    }

}

fn add(n1: u32, n2: u32) -> u32 {
    return n1 + n2;
}

fn sub(n1: u32, n2: u32) -> u32 {
    return n1 - n2;
}

fn div(n1: u32, n2: u32) -> f32 {
    return (n1 as f32) / (n2 as f32);
}

fn mul(n1: u32, n2: u32) -> u32 {
    return n1 * n2;
}

fn get_num_input() -> u32 { // Make sure to snake_case your functions ig
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {error}"),
    }

    let input_as_int: u32 = input.trim().parse().unwrap(); // Have to use trim() if using read_line
                                                           // also as is only for primative types
                                                           // :(
    return input_as_int;
}

fn get_op_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {error}"),
    }

    return input;
}

