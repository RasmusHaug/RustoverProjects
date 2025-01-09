use std::io;

fn main() {
    println!("Welcome to temperature converter!");

    loop {
        println!("Please choose the conversion:");
        println!("    1. Celsius     -> Kelvin");
        println!("    2. Celsius     -> Fahrenheit");
        println!("    3. Kelvin      -> Celsius");
        println!("    4. Kelvin      -> Fahrenheit");
        println!("    5. Fahrenheit  -> Celsius");
        println!("    6. Fahrenheit  -> Kelvin");

        let mut option = String::new();
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please insert temperature to convert from.");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => celsius_to_kelvin(temperature),
            2 => celsius_to_fahrenheit(temperature),
            3 => kelvin_to_celsius(temperature),
            4 => kelvin_to_fahrenheit(temperature),
            5 => fahrenheit_to_celsius(temperature),
            6 => fahrenheit_to_kelvin(temperature),
            _ => {
                not_implemented();
                break;
            }
        };
    }
}

fn celsius_to_kelvin(temperature: f32) {
    println!("🥶");
    println!("{temperature} Celcius is {} Kelvin.", temperature + 273.12);
}

fn celsius_to_fahrenheit(temperature: f32) {
    println!("🔥");
    println!(
        "{temperature} Celsius is {} Fahrenheit.",
        9.0 / 5.0 * temperature + 32.0
    );
}

fn kelvin_to_celsius(temperature: f32) {
    println!("{temperature} Kelvin is {} Celsius.", temperature - 273.12);
}
fn kelvin_to_fahrenheit(temperature: f32) {
    println!(
        "{temperature} Kelvin is {} Fahrenheit.",
        9.0 / 5.0 * temperature - 459.67
    );
}
fn fahrenheit_to_celsius(temperature: f32) {
    println!(
        "{temperature} Fahrenheit is {} Celsius.",
        5.0 / 9.0 * (temperature - 32.0)
    );
}
fn fahrenheit_to_kelvin(temperature: f32) {
    println!(
        "{temperature} Fahrenheit is {} Kelvin.",
        5.0 / 9.0 * (temperature + 459.67)
    );
}

fn not_implemented() {
    println!("🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀")
}
