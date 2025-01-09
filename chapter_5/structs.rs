// Struct to create users.
// Here we are deliberately using `String` instead of `&str`
//      Reason being: because we want each instance of this struct to own all of its data and for
//      that data to be valid for as long as the entire struct is valid.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Function to build a user.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Create new user using provided Struct.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someuseremail@provider.domain"),
        sign_in_count: 1,
    };

    // Using a function to build a new user.
    let user2 = build_user(
        "newuser2email@provider.domain".to_string(),
        "newuser2".to_string(),
    );

    // Building user3 from user1 but changing email field.
    let user3 = User {
        email: String::from("another@provider.domain"),
        ..user1
    };

    // Use dot notation to get data and change data.
    println!(
        "user1 email: {}, user2 email: {}, user3 email {}",
        user1.email, user2.email, user3.email
    );

    // Can't use user1 as the username was moved to user3.
    println!(
        "user2 name: {}, user3 name {}",
        user2.username, user3.username
    );
    user1.email = String::from("somenewuseremail@provider.domain");
    println!("user1 email: {}", user1.email);

    // Using Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Example of borrowing and moving data.
    let mut a = Point(1, 0, 2);
    a.0 += 1;
    let b = Point(a.0, 2, a.2);
    a.0 += 1;
    // This outputs `2` because at the point of creating `b` the value is `2`. The value of `b`
    // doesn't change later on because `a` changed.
    println!("{}", b.0);
}
