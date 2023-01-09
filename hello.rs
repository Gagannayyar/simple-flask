fn main() {
    let user1 = User{
        active: true,
        username: "Someone".to_string(),
        sign_in_count: 0
    };
    println!("{}",user1.username);

    let user2 = build_user(String::from("SomeoneElse"));
    println!("{}",user2.username)
}

struct User {
    active: bool,
    username: String,
    sign_in_count: u32 
}

fn build_user(username: String) -> User {
    User{
        username,
        active: true,
        sign_in_count: 1
    }
}

