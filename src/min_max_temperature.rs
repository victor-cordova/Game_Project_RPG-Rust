use std::io::{stdin};

fn main() {
    let temperatures: [i8; 5] = [-5, 6, -110, 5, 110];    
    let min_temperature: i8 = get_min_value(temperatures);
    let max_temperature: i8 = get_max_value(temperatures);

    println!("the minimun temperature is: {min_temperature}");
    println!("the maximum temperature is: {max_temperature}");
}

fn get_min_value(values:[i8; 5]) -> ic8{
    let mut prev:i8 = values[0];

    for value in values {
        if prev > value {
            prev = value;
        }
    }
    return prev;
}

fn get_max_value(values:[i8; 5]) -> i8{
    let mut prev:i8 = values[0];

    for value in values {
        if prev < value {
            prev = value;
        }
    }
    return prev;
}