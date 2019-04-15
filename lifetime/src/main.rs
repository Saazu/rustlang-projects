/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

//doesn't work. str2 does not have a lifetime beyond inner {...}
fn main() {
	let str1 = String::from("String is too long");

	let result;
	{
		let str2 = String::from("xyz");
		result = longest(str1.as_str(), str2.as_str());
	}
	
	println!("The longest string is {}", result);
}
*/
#[derive(Debug)]
struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn main() {
	let novel = String::from("Call me Ozymandias. My visage under the desert sun...");
	let first_sentence = novel.split('.')
		.next()
		.expect("Could not find a '.'");
	let i = ImportantExcerpt { part: first_sentence };
	println!("{:?}", i.part);
}