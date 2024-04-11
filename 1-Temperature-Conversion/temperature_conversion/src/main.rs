// Temperature Converter
//  - Asks user whether they would like to convert from C to F
//    or F to C.
//  - Uses one of two functions to get the temperature input and 
//    convert it to the appropriate value.

// Crates
use std::io;

// Main
fn main() {
    println!("Temperature Converter");

    let mut choice: char = 'L';
    let _conversion_type: char = 'N'; // starts as N as in None

    while choice == 'L' || choice == 'l' {
        println!("\nInput C to convert from Fahrenheit to Celsius or F to convert from Celsius to Fahrenheit, or Q to quit.");
        
        // get user input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // parse the input
        let chars: Vec<_> = user_input.chars().collect();
        let user_input = chars[0];

        let temperature_input: f64;

        if user_input == 'f' || user_input == 'F' {
            println!("Your choice is Celsius to Fahrenheit.");
            println!("Input the temperature you want in °F:");
            temperature_input = get_string_return_float();
            celsius_to_fahrenheit(temperature_input);

        } else if user_input == 'c' || user_input == 'C' {
            println!("Your choice is Fahrenheit to Celsius.");
            println!("Input the temperature you want in °C:");
            temperature_input = get_string_return_float();
            fahrenheit_to_celsius(temperature_input);

        } else if user_input == 'Q' || user_input == 'q' {
            println!("Thank you for using the temperature converter!");
            choice = 'Q';
        }
        
    }
    
    fn fahrenheit_to_celsius(temp: f64) {
        if temp.is_nan() {
            println!("A numerical value was not entered. Please enter a number.");
        } else {
            let temp_in_celsius: f64 = (temp - 32.0) * 5.0 / 9.0;
            println!("{temp}°F is {temp_in_celsius}°C\n");
        }
    }

    fn celsius_to_fahrenheit(temp: f64) {
        if temp.is_nan() {
            let temp_in_f = (temp * 9.0 / 5.0) + 32.0;
            println!("{temp}°C is {temp_in_f}°F\n");
        }
    }
    
    fn get_string_return_float() -> f64{
        let mut temperature_string = String::new();
        io::stdin()
            .read_line(&mut temperature_string)
            .expect("Failed to read line");
        let temperature_string = temperature_string.trim();
        let temperature_input: Result<f64, _> = temperature_string.parse();
        match temperature_input {
            Ok(temperature) => {
                return temperature;
            }
            Err(_) => {
                return f64::NAN;
            }
        }
    }
}

