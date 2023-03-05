struct User {
    name: String,
    email: String,
    age: u16,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn get_birthdate (&self) -> u16 {
        2023 - self.age
    }
}

struct Colors(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle { 
            width: side, 
            height: side
        }
    }
}

pub fn struct_topic() {
    let user_1 = User {
        active: false,
        email: String::from("victor@gmail.com"),
        name: String::from("victor"),
        sign_in_count: 1,
        age: 22,
    };

    // let user_2 = User {
    //     name: String::from("andres"),
    //     ..user_1
    // };
    // user_1.email;

    println!("{}", user_1.name);
    // let name: String = user_1.name;

    let color_2 = Colors(1, 5, 20);
    // let color_2 = Colors(1, 5, 20);
    let point_1 = Point(1, 5, 20);
    let point_2 = (16, 50, 20);
        // println!("{}", user_1.name);
    // print_colors(color_2);
    // print_colors(point_2);
    let rect = Rectangle {
        height: 32,
        width: 50,
    };

    // println!("{}", Rectangle::get_area(&rect));

    println!("The are of the rectangle es {}", rect.get_area());
    println!("The are of the rectangle es {}", user_1.get_birthdate());

    let square_1 = Rectangle::square(5);

    println!("The are of the square is {}", square_1.get_area());
    // let 
}
