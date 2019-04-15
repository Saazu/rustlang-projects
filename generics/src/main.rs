/*
//non generic function
fn largest_i32(list: &[i32]) -> i32 {
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest {
			largest = item
		}
	}
	largest
}
struct Point<T, U> {
	x: T,
	y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let integer_and_float = Point {x: 5, y: 4.0 };
}

#[derive(Debug)]
struct Point <T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

fn main() {
	let p = Point { x: 5, y: 10 };
	println!("p.x is {:?}", p.x());	
}
*/

struct Point<T, U> {
	x: T,
	y: U,
}

impl<T, U> Point<T, U> {
	fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}

fn main (){
	let p1 = Point { x: 5, y: 10.4};
	let p2 = Point{x: "Hello", y: 'c'};

	let p3 = p1.mixup(p2);
	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}