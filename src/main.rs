//a simple temperature unit converter
use std::io;
use std::str;
//program
fn main() {
    loop {
    //introduces the user to their options for unit_type
        println!("Select the unit you would like to convert from:", );
        println!("Type 1 for Fahrenheit, 2 for Celsius, or 3 for Kelvin", );
    //allows user input
        let mut unit = String::new();
    //reads user input
        io::stdin().read_line(&mut unit)
            .expect("Failed to read line");
        let unit = unit.trim();
    //states units used
        match &*unit {
            //F
            "1" => {
                println!("You are converting from Fahrenheit", );
                //input unit to convert to
                println!("Input what you would like to convert to:", );
                println!("Type 1 for Celsius or 2 for Kelvin", );
                let mut to_unit = String::new();
                io::stdin().read_line(&mut to_unit)
                    .expect("Failed to read line");
                let to_unit = to_unit.trim();
                //determines what equation to use
                match &*to_unit {
                    //F=>C
                    "1" => {
                        println!("You are converting to Celsius", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v - 32.0;
                        let v = v * 5.0 / 9.0;
                        println!("This is equal to {} degrees in Celsius
                        ", v);
                    }
                    //F=>K
                    "2" => {
                        println!("You are converting to Kelvin", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v + 459.67;
                        let v = v * 5.0 / 9.0;
                        //says result
                        println!("This is equal to {} degrees in Kelvin
                        ", v);
                    }
                    _ => {
                        println!("Invalid choice", );
                    }
                }
            }
            //C
            "2" => {
                println!("You are converting from Celsius", );
                //input unit to convert to
                println!("Type 1 for Fahrenheit or type 2 for Kelvin:", );
                let mut to_unit = String::new();
                io::stdin().read_line(&mut to_unit)
                    .expect("Failed to read line");
                let to_unit = to_unit.trim();
                //determines which equation to use
                match &*to_unit {
                    //C=>F
                    "1" => {
                        println!("You are converting to Fahrenheit", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v * 18.0 / 10.0;
                        let v = v + 32.0;
                        println!("This is equal to {} degrees in Fahrenheit
                        ", v);
                    }
                    //C=>K
                    "2"=> {
                        println!("You are converting to Kelvin", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v + 273.15;
                        //says result
                        println!("This is equal to {} degrees in Kelvin
                        ", v);
                    }
                    _ => {
                        println!("Invalid choice", );
                    }
                }
            }
            //K
            "3" => {
                println!("You are converting from Kelvin", );
                //input unit to convert to
                println!("Input what you would like to convert to:", );
                println!("Type 1 for Fahrenheit or 2 for Celsius", );
                let mut to_unit = String::new();
                io::stdin().read_line(&mut to_unit)
                    .expect("Failed to read line");
                let to_unit = to_unit.trim();
                //determines which equation to use
                match &*to_unit {
                    //K=>F
                    "1" => {
                        println!("You are converting to Fahrenheit", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v * 18.0 / 10.0;
                        let v = v - 459.67;
                        println!("This is equal to {} degrees in Fahrenheit
                        ", v);
                    }
                    //K=>C
                    "2" => {
                        println!("You are converting to Celsius", );
                        //input
                        println!("Input the temperature you have", );
                        let mut v = String::new();
                        io::stdin().read_line(&mut v)
                            .expect("Failed to read line");
                        //math
                        let v = v.trim().parse::<f32>().expect("That's not a number!");
                        let v = v - 273.15;
                        //says result
                        println!("This is equal to {} degrees in Celsius
                        ", v);
                    }
                    _ => {
                        println!("Invalid choice", );
                    }
                }
            }
            _ => {
                println!("Invalid choice", );
            }
        }
    //asks if another conversion is required
        println!("
        Do you have another temperature to convert?
        y=Yes, n=No", );
        let mut ask = String::new();
        io::stdin().read_line(&mut ask)
            .expect("Failed to read line");
        let ask = ask.trim();
        match &*ask {
            "y" => {
                println!("Ok then.", );
            }
            "n" => {
                break;
            }
            _ => {
                println!("Invalid choice", );
            }
        }
    }
}
