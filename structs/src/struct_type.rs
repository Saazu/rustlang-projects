struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user (email: String, username, String) {
	User {
		username,
		email,
		sign_in_count: 1,
		active: true,
	}
}

user1 = build_user("aadvark@g.com", "addy");

user2 = User {
	email: String::from("aadvark3@g.com"),
	username: String::from("addy3"),
	..user1
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);