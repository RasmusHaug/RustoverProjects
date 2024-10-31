fn main() {
    println!("Hello, World!");
    another_function();
    another_parameters_function(5);
    multiple_parameters_function(5, 'h');
    expressions();
    let x = return_five();
    println!("The value of x is: {x}; after returning five");
    // I could also just type `5` inside the parantheses.
    let x = return_plus_one(x);
    println!("The value of x is: {x}; after returning plus one");
}

fn another_function() {
    println!("Another Function");
}

fn another_parameters_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn return_five() -> i32 {
    5
}

fn return_plus_one(x: i32) -> i32 {
    x + 1
}
