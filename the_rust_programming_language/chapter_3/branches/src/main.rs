fn main() {
    evaluate_number(3);
    evaluate_number(7);
    if_inside_let_statement();
    //simple_loop_example();
    break_loop_example();
    advanced_loop_breaking();
    while_loop();
    loop_array_unsafe();
    loop_array_safe();
    for_range();
}

fn evaluate_number(number: i32) {
    if number < 5 {
        println!("Condition was true for number: {number}");
    } else {
        println!("Condition was false for number: {number}");
    }
}

fn if_inside_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

// dead code.
fn simple_loop_example() {
    loop { 
        println!("again!");
    }
}

fn break_loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn advanced_loop_breaking() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_array_unsafe() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}


fn loop_array_safe() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
