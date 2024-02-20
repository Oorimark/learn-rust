// regular struct
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// tuple struct
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main(){
    let color = Color(0, 0, 0);
    let (x, y, z) = color;

    let user_1 = User {
        name: String::from("John Doe"),
        email: String::from("johndoe@gmail.com"),
        active: true,
        sign_in_count: 2,
    };

    let user_2 = User {
        ..user_1,
        email: String::from("user2@gmail.com");
    };
}

fn build_user (name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
        sign_in_count: 1,
    }
}
