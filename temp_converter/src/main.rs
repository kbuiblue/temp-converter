use std::io;

fn convert_to_f(celcius: f64) -> f64 {
    return (celcius * 9.0/5.0) + 32.0;
}

fn convert_to_c(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0/9.0;
}

fn get_num_option() -> i32 {
    println!("Which conversion are you doing?
                1. Celcius to Fahrenheit
                2. Fahrenheit to Celcius");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

        match user_input.trim().parse::<i32>() {
            Ok(i) if i == 1 || i == 2 => i,
            _ => {
                println!("Invalid choice, try again");
                return 0;
            },
        }
}

fn get_temp_input() -> Option<f64> {
    println!("Enter your temperature: ");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    match user_input.trim().parse::<f64>() {
        Ok(i) => Some(i),
        _ => {
            None
        }
    }
}

fn main() {
    let mut num_option = get_num_option();

    while num_option == 0 {
        num_option = get_num_option();
    }

    loop {
        let temp_input = get_temp_input();

        match temp_input {
            Some(temp) => {
                if num_option == 1 {
                    let f_temp = convert_to_f(temp);
                    println!("{:.2}C is {:.2}F", temp, f_temp);
                } else if num_option == 2 {
                    let c_temp = convert_to_c(temp);
                    println!("{:.2}F is {:.2}C", temp, c_temp);
                }
                break;
            },
            None => {
                println!("Invalid temperature. Try again.");
            },
        }
    }
}
