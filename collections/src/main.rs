use std::collections::HashMap;

fn main() {
	/*
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
    

    let mut v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("{:?}", third); //prints "3"

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third); //prints "Some(3)"

    //let does_not_exist = &v[100]; //program panics and crashes
    //let does_not_exist = v.get(100); returns None if reference does not exist

    for i in &v {
    	println!("{}", i);
    }

    for i in &mut v {
    	*i += 50;
    }
    println!("{:?}", v);
    
    #[derive(Debug)]
    enum SpreadsheetCell {
    	Int(i32),
    	Float(f64),
    	Text(String),
    }

    let row = vec![
    	SpreadsheetCell::Int(3),
    	SpreadsheetCell::Text(String::from("blue")),
    	SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
    */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("1. {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let new_scores: HashMap<_, _> = teams.iter()
    									.zip(initial_scores.iter())
    									.collect();
   	println!("2. {:?}", new_scores);
}
