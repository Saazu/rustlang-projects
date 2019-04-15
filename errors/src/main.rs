use std::fs::File;
use std::io;
use std::io:Read;
//use std::io::ErrorKind;

fn read_username_from_file() -> Result<String, io::Error> {
	//propagating errors. Long method
	/*
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e), 
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
	*/

	//propagating errors, shortcut with "?"
	/*
	let f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
	*/

	let mut s = String::new();

	File::open("hello.txt")?.read_to_string(&mut s)?;
}

fn main() {
	let f = File::open("hello.txt").expect("There was a problem opening the file");

	/*
	let f = match f {
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
			match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => {
					panic!("Tried to create file but there was a problem: {:?}", e)
				},
			}
		},
		Err(error) => {
			panic!("There was a problem opening the file: {:?}")
		},
	};
	*/
}
