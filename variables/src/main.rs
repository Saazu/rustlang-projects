fn main() {
	let a = [1,2,3,4,5];

	let first = a[0];
	let second = a[1];

	println!("First {}, Second {}", first, second);
}

//Testing traits... Ignore for variables
/*
pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("{}", self.author)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}

fn main() {
	let tweet = Tweet {
		username: String::from("horse"),
		content: String::from("blah blah blah"),
		reply: false,
		retweet: false,
	};
	println!("1 new tweet {}", tweet.summarize());
}