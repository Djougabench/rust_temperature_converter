use std::io;

/**************************************************************************
 f to c = f-32/1.8
 c to f = (c x 1.8)+ 32

***************************************************************************/

fn main() {
    println!("Welcome to the temperature converter (Fharenheit to Celcius");

    loop {
        println!("\nEnter one of the following commands:");
        println!("- 0 : CONVERT FROM CELSIUS TO FHAREINHEIT. ");
        println!("- 1 : CONVERT FROM FHAREINHEIT TO CELSIUS. ");
        println!("- 2 : QUIT");

        /*variable to store the User input*/
        let mut command = String::new();

        /*The match expression to check if the input writen by the user
         is one of the  commandand catch the error
        */
        match io::stdin().read_line(&mut command) {
            Ok(_) => {}
            Err(_) => {
                println!("failed to read command.");
                continue;
            }
        }
        /*convert the string from the input to an integer
         and catch the error ,if the command is valid
        */
        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid command.");
                continue;
            }
        };
        /*assignement of each comand number with The match expression
         to a function through an arm.
         possibility to quit thanks to the break
        */
        match command {
            0 => celsius_to_fahrenheit(),
            1 => fahrenheit_to_celsius(),
            2 => break,
            _ => println!("Invalid command."),
        }
        /*function to convert celsius to fahrenheit*/
        fn celsius_to_fahrenheit() {
            println!(" Enter the temperature un celsius.");

            /*variable to store the User input*/
            let mut temperature = String::new();

            /*The match expression to check if the input writen by the user
             is one of the  command
             and catch the error
            */
            match io::stdin().read_line(&mut temperature) {
                Ok(_) => {}
                Err(_) => {
                    println!(" failed to read temperature.");
                    return;
                }
            }

            let temperature: f64 = match temperature.trim().parse() {
                Ok(t) => t,
                Err(_) => {
                    println!("Invalid temperature.");
                    return;
                }
            };

            /*variable to store the calculation of the conversion
             *Celsius to Fahreinheit
             *
             */
            let converted = temperature * 1.8 + 32.0;
            println!("{} Celsius = {} Fahreinheit ", temperature, converted)
        }

        /*function to convert  fahrenheit to celsius*/
        fn fahrenheit_to_celsius() {
            println!("Enter the temperature in Frhreinheit.");
            let mut temperature = String::new();

            match io::stdin().read_line(&mut temperature) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to read temperature.");
                    return;
                }
            }
            /*convert the string from the input to an integer
             and catch the error ,if the temperature is valid
            */
            let temperature: f64 = match temperature.trim().parse() {
                Ok(t) => t,
                Err(_) => {
                    println!("Invalid temperature.");
                    return;
                }
            };
            /*variable to store the calculation of the conversion
             *Fahreinheit to Celsius
             *
             */
            let converted = (temperature - 32.0) / 1.8;
            println!("{} Fahreinheit = {} Celsius", temperature, converted)
        }
    }
}
