#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println! takes reference of expression
    println!("Simple Debug print: rect1 is {rect1:?}");
    println!("Easier to read Debug print: rect1 is {rect1:#?}");
    // dbg! takes ownership of expression
    // dbg! macro prints to the standard error console stream (stderr) instead of stdout
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Accesses the `width` and `height` fields of the `Rectangle` instance. Note that accessing fields
// of a borrowed struct instance does not move the field values.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
