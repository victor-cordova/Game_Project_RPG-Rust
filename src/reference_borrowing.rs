pub fn reference_borrowing () {
    let s1 = get_value();
    let s2 = String::from("This is s2");
    let s3 = take_and_give(&s2);
    // s3 = "asdas".to_string();
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
}

fn get_value () -> String {
    let s1_in = String::from("This is s1");
    s1_in
}

fn take_and_give(var: &str) -> String {
    var.to_string()
}