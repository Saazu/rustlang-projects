use std::io;

fn main() {
    println!("Enter your temperature in Celsius: ");
    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
    	.expect("Failed to read. Deep OS error");

    let temp: i32 = temp.trim().parse().expect("Shadwoing/muting failed");

    let fahrenheit = c_to_f(temp);

    println!("fahrenheit temperature is: {}", fahrenheit);
    strings();

}

fn c_to_f (x: i32) -> i32 {
	x * 32
}

fn strings() {
	let mut s = String::from("hello");
	s.push_str(", world!"); //append string
	println!("{}", s);
}