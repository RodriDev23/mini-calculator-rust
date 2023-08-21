use std::io;

fn sum_num(x: i32, y: i32) -> i32 {
    x + y
}

fn rest_num(x: i32, y: i32) -> i32 {
    x - y
}

fn multi_num(x: i32, y: i32) -> i32 {
    x * y
}

fn div_num(x: i32, y: i32) -> i32 {
    x / y
}

fn read_user(x: i32, y: i32) {
    let mut input_user = String::new();
    println!("Please enter an option:\n1. sum\n2. rest\n3. multi\n4. div");
    io::stdin()
        .read_line(&mut input_user)
        .expect("Error reading input");
    let cleaned_input_user: i32 = input_user.trim().parse().expect("Invalid option");

    let result = match cleaned_input_user {
        1 => sum_num(x, y),
        2 => rest_num(x, y),
        3 => multi_num(x, y),
        4 => div_num(x, y),
        _ => {
            println!("Invalid option");
            return;
        }
    };

    println!("Result: {}", result);
}

fn main() {
    let mut read_string1 = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut read_string1)
        .expect("Error reading input");
    let cleaned_input1: i32 = read_string1.trim().parse().expect("Invalid number");

    let mut read_string2 = String::new();
    println!("Please enter another number:");
    io::stdin()
        .read_line(&mut read_string2)
        .expect("Error reading input");
    let cleaned_input2: i32 = read_string2.trim().parse().expect("Invalid number");

    read_user(cleaned_input1, cleaned_input2);
}
