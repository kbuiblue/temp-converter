use std::io;

fn convert_to_f(celcius: i32) -> i32 {
    return (celcius * 9/5) + 32;
}

fn convert_to_c(fahrenheit: i32) -> i32 {
    return (fahrenheit - 32) * 5/9;
}

fn main() {
    println!("Which conversion are you doing?
                1. Celcius to Fahrenheit
                2. Fahrenheit to Celcius");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let trimmed = user_input.trim();

    let num_option = match trimmed.parse::<i32>() {
        Ok(i) if i == 1 || i == 2 => i,
        _ => {
            println!("Invalid choice");
            return;
        },
    };

    println!("Enter your temperature: ");

    user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let trimmed = user_input.trim();

    let temp_input: i32 = match trimmed.parse::<i32>() {
        Ok(i) => i,
        _ => {
            println!("Invalid temperature");
            return;
        }
    };

    if num_option == 1 {
        let f_temp = convert_to_f(temp_input);
        println!("{temp_input}C is {f_temp}F");
    } else if num_option == 2 {
        let c_temp = convert_to_c(temp_input);
        println!("{temp_input}F is {c_temp}C");
    }
}
