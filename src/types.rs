// use core::num;

pub fn data_types () {
    let mut x = 5;
    // let y = &x;

    println!("{:p}", &x);
    // println!("{:p}", &y);

    x= 8+20;

    println!("{:p}", &x);
    // println!("{:p}", &y);

    // let numbers1 = (8, 8, 4, "asd");
    // let numbers2 = numbers1;

    // println!("{:p}", &numbers1);
    // println!("{:p}", &numbers2);

    let variables1 = (8, 8, 4, "asd");
    let variables2 = variables1;
    
    println!("{:p}", &variables1);
    println!("{:p}", &variables2);

    let new_tuple: (i8, i8) = (5, 5);
    println!("{:?}", new_tuple);
    // new_tuple.0 = 's';
    println!("{:?}", new_tuple);
}