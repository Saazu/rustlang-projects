#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	// add code here
	fn area(&self) ->u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square (size: u32) -> Rectangle {
		Rectangle { width: size, height: size }
	}
}

fn main() {
	let rect1 = Rectangle { width: 30, height: 50 };
	let rect2 = Rectangle { width: 10, height: 40 };
	let rect3 = Rectangle { width: 60, height: 45 };

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	/*
	let rect = (30, 50);
	let width1 = 30;
	let height1 = 50;

	//rectangle from variables
	println!(
		"The area(1) of the rectangle is {} square pixels",
		area1(width1, height1)
		);

	//rectangle from unnamed tuple
	println!("The area(2) of the rectangle is {} square pixels",
		area(rect)
		);

	//rectangle from named struct
	println!("The area(3) of the Rectangle is {} square pixels",
		rect3.area()
		);

	println!("Rect is {:#?}", rect3);

	*/
}
/*
fn area1(width: u32, height: u32) -> u32 {
	width * height
}

fn area(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
*/