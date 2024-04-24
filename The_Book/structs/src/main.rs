struct User {
    username : String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn main() {
    let mut user1 = User{
        username: String::from("vamshireddy01"),
        email: String::from("vamshireddy@email.com"),
        sign_in_count: 1,
        active: true
    };

    user1.username = String::from("vamshireddy02");

    let user2 = build_user(
        String::from("something@email.com"),
        String::from("somethingUserName")
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true
    }
}
