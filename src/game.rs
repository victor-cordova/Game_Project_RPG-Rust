use csv::{ReaderBuilder, StringRecord};
use std::{fs}; //Es la dependencia file system, para poder acceder a archivos del sistema
use std::io::{stdin};
//const es una variable que no varía y se puede acceder desde cualquier parte.
const FILE_NAME: &str = "history.csv";

pub fn game() {
    let content: String = fs::read_to_string(FILE_NAME).unwrap();
    // print!("{}", content);
    let reader = ReaderBuilder::new().
        delimiter(b';').
        from_reader(content.as_bytes());
    let iter = reader.records().next().unwrap();
    // let letter = 's';
    // reader.is_done()
    // for result in reader.records() {
    //     println!("{:?}", result);
    // }
    // println!("{:?}", );
    // let asssd = reader.records()[1][1];
    // let ssssss = reader.records()[2][1];
    // let s: &str = 5;
}