use csv::{ReaderBuilder, StringRecord, Reader};
use std::{io::stdin, fs, io::Error, collections::HashMap};

const FILE_NAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
enum Classes {
    Situation,
    Option,
}

#[derive(Debug)]
struct Row {
    class: Classes,
    tag: String,
    text: String,
    health: i16,
    options: Vec<Row>,
}

impl Row {
    fn new (data: StringRecord) -> Row{
        let class = data.get(0).unwrap().trim();
        let class_enum = match class {
            "SITUACION" => Classes::Situation,
            &_ => Classes::Option,
        };

        Row {
            class: class_enum,
            tag: data.get(1).unwrap().trim().to_string(),
            text: data.get(2).unwrap().trim().to_string(),
            health: data.get(3).unwrap().trim().parse::<i16>().unwrap_or(0),
            options: Vec::new(),
        }
    }
}

fn insert_data (item: Row, data: &mut HashMap<String, Row>, last_tag: &mut String) {
    match item.class {
        Classes::Situation => {
            *last_tag = item.tag.clone();

            data.insert(item.tag.clone(), item);
        },
        Classes::Option => {
            let option = data.get_mut(last_tag);
            
            match option {
                Some(item_ref) => {
                    item_ref.options.push(item);
                },
                None => {
                    println!("There is an error.");
                }
            }
        },
    }
}

fn show_situation (text: &str, health_new: &Vec<char>) {
    for i in 0..health_new.len() {
        print!("{}", health_new[i]);
    }
    println!("");
    println!("{}", text);
    println!("");
}

fn show_options (list: &Vec<Row>, arrows: &[char; 3]) {
    for (index, option) in list.iter().enumerate() {
        println!("[{}] {} {}", index, option.text, arrows[index]);
    }
    println!("");
}

fn update_health (health: &mut Vec<char>, health_change: &i16) {
    let lenght: usize = health.len();
    let mut reversed_i: usize;

    if health_change.is_negative() {
        for i in 0..lenght {
            reversed_i = (lenght-1)-i;

            if health[reversed_i] == '🧡' {
                health[reversed_i] = '🖤';
                if *health_change != -1000 {
                    break;
                }
            }
        }
    } else if *health_change != 0 {
        if health[lenght-1] == '🖤' {
            health[lenght-1] = '🧡';
        } else {
            health.push('🧡');
        }
    }
}

fn choose_path (paths: &Vec<Row>, index: usize, current_tag: &mut String) {
    let chosen = paths.get(index);
    match chosen {
        Some(value) => {
            current_tag.clear();
            current_tag.push_str(&value.tag);
            // current_tag = &value.tag;
            // current_tag = &value.tag;
        },
        None => println!("Camino incorrecto :c, elige otro :D"),
    }
}

fn add_data (reader: &mut Reader<&[u8]>, data: &mut HashMap<String, Row>, last_tag: &mut String) {
    for result in reader.records() {
        match result {
            Ok(iter) => {
                let row = Row::new(iter);
                insert_data(row, &mut *data, &mut *last_tag);
            },
            Err(error) => println!("Error: {error}"),
        };
    }
}

pub fn game() { 
    let mut current_tag = FIRST_TAG.to_owned();
    let mut last_tag = String::new();
    let mut input = String::new();
    let mut selected_index: usize;
    let arrows: [char; 3] = ['↗', '➡', '↘'];
    let mut health_vec = Vec::from(['🧡', '🧡']);
    let content_result: Result<String, Error> = fs::read_to_string(FILE_NAME);
    let mut data: HashMap<String, Row> = HashMap::new();

    match content_result {
        Ok(content) => {
            let mut reader_result = ReaderBuilder::new().
                delimiter(b';').
                from_reader(content.as_bytes());

            add_data(&mut reader_result, &mut data, &mut last_tag);
            loop {
                let situation_op = data.get(&current_tag);
                match situation_op {
                    Some(situation) => {
                        update_health(&mut health_vec, &situation.health);
                        show_situation(&situation.text, &health_vec);
                        show_options(&situation.options, &arrows);

                        if health_vec[0] == '🖤' {
                            println!("Haz perdido.");
                            break;
                        }
                        
                        stdin().read_line(&mut input).unwrap();
                        println!("");

                        selected_index = input.trim().parse().unwrap_or(99);
                        choose_path(&situation.options, selected_index, &mut current_tag);

                        input.clear();
                    }, 
                    None => {
                        println!("Ganaste!!!");
                        break;
                    },
                }
            }
        },
        Err(error) => println!("Error: {error}"),
    }
}