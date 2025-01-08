fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn numeric_operations() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    
    // remainder
    let _remainder = 43 % 5;
}

fn booleans() {
    let _t = true;
    let _f: bool = false; // with expliocit type annotation
}

fn character_types() {
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}


