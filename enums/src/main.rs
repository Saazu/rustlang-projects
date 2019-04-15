/*
enum UsState {
	Alaska,
	Alabama,
	Mississippi,
	Maine,
	NewYork,
	Connecticut,
	Pennsylvania,
	California,
	Massachussets,
	RhodeIsland,
	Nevada,
	Oregon,
	Kentucky,
	Louisiana,
	NorthDakota,
	SouthDakota,
	Arizona,
	NorthCarolina,
	SouthCarolina,
	Georgia,
	Michigan,
	Minnesota,
	NewHampshire,
}

enum Coin {
	Penny,
	Nickle,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny!");
			1
		},
		Coin::Nickle => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		},
	}
}
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1)
	}
}

fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("{:?} {:?} {:?}", five, six, none);
}