use std::io;

struct Convertor {
    scale: String,
    function: fn(f64) -> f64,
}

fn main() {
    loop {
        let prompt = String::from(
            "Select the unit you would like to convert from:\nType 1 for Fahrenheit, 2 for Celsius, or 3 for Kelvin"
            );
        match get_user_input::<i32>(&prompt) {
            1 => convert_choice("Fahrenheit", 
                                Convertor {
                                    scale: String::from("Celisus"),
                                    function: |x: f64| (x - 32.0) * 5.0 / 9.0,
                                }, Convertor {
                                    scale: String::from("Kelvin"),
                                    function: |x: f64| (x + 459.67) * 5.0 / 9.0,
                                }),

            2 => convert_choice("Celsius", 
                                Convertor {
                                    scale: String::from("Fahrenheit"),
                                    function: |x: f64| (x * 18.0 / 10.0) + 32.0,
                                }, Convertor {
                                    scale: String::from("Kelvin"),
                                    function: |x: f64| x + 273.15,
                                }),
            3 => convert_choice("Kelvin",
                                Convertor {
                                    scale: String::from("Fahrenheit"),
                                    function: |x: f64| (x * 18.0 / 10.0) - 459.67,
                                }, Convertor {
                                    scale: String::from("Celisus"),
                                    function: |x: f64| x - 273.15,
                                }),
            _ => println!("Invalid Choice"),
        }

        let prompt = String::from(
            "Do you have another temperature to convert?\ny=Yes, n=No"
            );
        match get_user_string(&prompt).trim() {
            "y" => println!("Ok then."),
            "n" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn convert(input: Convertor) {
    let prompt = format!(
        "You are converting to {}\nInput the temperature you have", input.scale
        );
    let temp = get_user_input::<f64>(&prompt);
    println!("This is equal to {:.2} degrees in {}", (input.function)(temp), input.scale);
}

fn convert_choice(from_scale: &str, option_1: Convertor, option_2: Convertor) {
    loop {
        let prompt = format!(
            "You are converting from {}\nInput what you would like to convert to:\nType 1 for {} or 2 for {}",
            from_scale,
            option_1.scale,
            option_2.scale
            );
        match get_user_input::<i32>(&prompt) {
            1 => convert(option_1),
            2 => convert(option_2),
            _ => {
                println!("Invalid choice");
                continue;
            },
        }
        break;
    }
}

fn get_user_string(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Reading stdin");
    println!();
    input
}

fn get_user_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = get_user_string(prompt);
        match input.trim().parse::<T>() {
            Ok(n) => return n,
            _ => println!("Invalid Input"),
        };
    }
}
