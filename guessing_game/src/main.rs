extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Guess struct to restrict value of guess: 1 <= guess <= 100
pub struct Guess {
	value: u32,
}

impl Guess {
	pub fn new(value: u32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100. Got {}.", value);
		}
		Guess {
			value
		}
	}

	pub fn value(&self) -> u32 {
		self.value
	}
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    

    loop {
    	println!("Please input your guess.");

    	let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line");

	    //convert guess string to u32 and check that guess is u32
	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => continue,
	    };
	    
	    //convert u32 to Guess type
	    let guess: Guess =  Guess::new(guess);

	    //println!("You guessed: {}", guess);

	    match guess.value().cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal => {
	    		println!("You win!");
	    		break;
	    	}
	    }
    }
	    
}
