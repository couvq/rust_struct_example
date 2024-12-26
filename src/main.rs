struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user = User {
        email: String::from("<EMAIL>"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{},{}, {}, {}", user.email, user.username, user.active, user.sign_in_count);
    user.active = false; // user must be marked mutable to be able to do this or else we get a compile error
    println!("{},{}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    let user2  = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user // we also can copy fields from another struct using the .. notation (spread operator in javascript)
    };
    println!("{},{}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect);
}
