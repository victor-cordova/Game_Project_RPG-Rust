use std::io::{stdin};

pub fn exercise_get_age () {
    let age: u8 = get_age();

    println!("the user is {age} years old.");
}

fn get_age() -> u8{
    let mut input_int: Option<u8>;

    loop {
        let mut input: String = String::new();

        println!("Digit your age");
        stdin().read_line(&mut input).unwrap();

        input_int = convert_int(input);

        match input_int {
            Some(value) => return value,
            None => println!("Try writing a valid age."),
        }
    }
}

fn convert_int(data: String) -> Option<u8> {
    match data.trim().parse::<u8>() {
        Ok(value) => {
            if value <= 100 {
                Some(value)
            } 
            else {
                println!("Try writing a valid age.");
                None
            }
        }
        Err (error) => {
            println!("error: {error}");
            None
        }
    }
}