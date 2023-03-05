// pub fn lifetime_topic() {
//     let string1: String = String::from( "abcd");
//     let string2: String = String::from("xyz");
//     let result: &str = longest(string1.as_str(), string2.as_str());

//     println! ("The longest string is {}", result);
// }

struct Person<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Person<'a> {
    fn return_name (&self, other_name: &str ) -> &str {
        self.name
    }
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

pub fn lifetime_topic(){
    let string1 = String::from("abcd");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);


    {
        let name_1: &'static str = "victor";
        // let person = Person {
        //     age: 22,
        //     name: &name_1,
        // };
        // person.name = &mut "andres";
    }
    // println!("{}", name_1);
    

}

fn return_something<'a>(x: i32, y: &'a str, a:&str) -> &'a str {
    
    y
}

// pub fn lifetime_topic () {
//     let r: &i32;
//     {
//         let x: i32 = 50;
//         r = &x;
//     }

//     println!("{}", r);
// }