// #![allow(dead_code)]
#[derive(Debug)]

// std::Display;
enum PositionsV2 {
    First,
    Second,
    Third,
    Others
}

struct Result {
    position: PositionsV2,
    name: String
}

impl Result {
    fn congratulate(&self) -> String {
        let mut message = String::from("Congratulations for finishing the race");
        match self.position{
            PositionsV2::First | PositionsV2::Second | PositionsV2::Third => {
                let added = format!(", {} you got the {:?} place you are amazing",self.name, self.position);

                message.push_str(&added);

                return message;
            }
            _ => {
                message.push_str(" you rock");

                return message;
            }
        }
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

pub fn enum_topic () {
    let name: Option<String> = Some(String::from("victor"));
    let x: i32 = 5;
    let y: Option<i32> = Some(20);

    let sum = x + y.unwrap();

    // let name_2  = Option<T> {
    //     name
    // };

    println!("{:?}", sum);

    let some_value: Option<i32> = Some (3);
    match some_value {
        Some (3) => println! ("three"),
        _ => (),
    }
    if let Some (3) = some_value {
    println! ("three");
    }


    
}