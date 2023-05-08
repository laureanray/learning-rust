struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
};

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "asdado",
        sign_in_count: 1,
    }
}

