use std::io;

fn main() {
    loop {
        println!("Type n'th POSITIVE fibonacci number to find:");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let value: u64 = match option.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!("fibonacci ({}) => {}", value, fibonacci(value))
    }
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
