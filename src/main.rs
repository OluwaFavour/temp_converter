use std::io::{self, Write};

/// Converts temperature between Fahrenheit and Celsius
enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

/// Implement the Temperature enum
impl Temperature {
    /// Converts the temperature from Fahrenheit to Celsius
    fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            Temperature::Celsius(c) => *c,
        }
    }

    /// Converts the temperature from Celsius to Fahrenheit
    fn to_fahrenheit(&self) -> f64 {
        match self {
            Temperature::Fahrenheit(f) => *f,
            Temperature::Celsius(c) => c * 9.0 / 5.0 + 32.0,
        }
    }
}

/// Reads a line from standard input and trims it
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user
///
/// # Returns
///
/// The trimmed input string
///
/// # Examples
///
/// ```
/// let input = read_input("Enter your name: ");
/// ```
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// Parses the temperature scale and value from user input
///
/// # Returns
///
/// The temperature enum
fn get_temperature_input() -> Result<Temperature, String> {
    let temperature_scale_str =
        read_input("Enter the temperature scale (F for Fahrenheit, C for Celsius): ");
    let temperature_value_str = read_input("Enter the temperature value: ");

    let temperature_scale = temperature_scale_str
        .chars()
        .next()
        .ok_or("Invalid scale input")?;
    let temperature_value: f64 = temperature_value_str
        .parse()
        .map_err(|_| "Invalid temperature value")?;

    match temperature_scale.to_ascii_uppercase() {
        'F' => Ok(Temperature::Fahrenheit(temperature_value)),
        'C' => Ok(Temperature::Celsius(temperature_value)),
        _ => Err("Invalid temperature scale, please enter F or C".to_string()),
    }
}

fn main() {
    // Loop until the user provides valid input
    loop {
        // Get the temperature input from the user
        match get_temperature_input() {
            // If the input is valid, convert the temperature and print the result
            Ok(temperature) => {
                match temperature {
                    Temperature::Fahrenheit(_) => {
                        println!(
                            "The temperature in Celsius is: {:.2} °C",
                            temperature.to_celsius()
                        );
                    }
                    Temperature::Celsius(_) => {
                        println!(
                            "The temperature in Fahrenheit is: {:.2} °F",
                            temperature.to_fahrenheit()
                        );
                    }
                }
                break;
            }
            // If the input is invalid, print an error message and ask the user to try again
            Err(e) => {
                println!("Error: {}", e);
                println!("Please try again.\n");
            }
        }
    }
}
