use std::io::{stdin};
use regex::{Regex, Captures};

struct RegexOperators {
    sum: Regex,
    substract: Regex,
    divide: Regex,
    multiply: Regex,
}

struct Expression <'a>{
    operation: &'a str,
    paren_1: &'a str,
    paren_2: &'a str,
    var_1: i16,
    var_2: i16,
    symbol: &'a str
}

pub fn exercise_calculator_v2() {
    println!("Digit the equation.");

    //factorizar a uno solo y que los demas solo sean obtenidos, por un replace.
    let operators = RegexOperators {
        sum: Regex::new(r"(\(?)(\d+)(\+)(\d+)(\)?)").unwrap(), 
        substract: Regex::new(r"(\(?)(\d+)(\-)(\d+)(\)?)").unwrap(), 
        divide: Regex::new(r"(\(?)(\d+)(/)(\d+)(\)?)").unwrap(), 
        multiply: Regex::new(r"(\(?)(\d+)(\*)(\d+)(\)?)").unwrap(), 
    };
    
    let mut equation: String = String::new();
    stdin().read_line(&mut equation).unwrap();

    let mut answer: String = equation.replace(" ", "");

    calculator(&mut answer, &operators.divide);
    calculator(&mut answer, &operators.multiply);
    calculator(&mut answer, &operators.sum);
    calculator(&mut answer, &operators.substract);

    println!("The answer is: {answer}");
}

fn destructure_caps<'a>(caps: &'a Captures) -> Expression<'a> {
    Expression {
        operation: caps.get(0).unwrap().as_str(),
        paren_1: caps.get(1).unwrap().as_str(),
        paren_2: caps.get(5).unwrap().as_str(),
        symbol: caps.get(3).unwrap().as_str(),
        var_1: caps.get(2).unwrap().as_str().parse().unwrap(),
        var_2: caps.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn calculate (var_1: i16, var_2: i16, symbol: &str) -> i16{
    match symbol {
        "+" => var_1 + var_2,
        "-" => var_1 - var_2,
        "*" => var_1 * var_2,
        "/" => if var_2 == 0 {
            0
        } else {
            var_1 / var_2
        },
        _ => 0,
    }
}

fn get_operation (expression: &Expression, result: i16) -> String {
    let operation: String;
    if expression.paren_1 == "(" && expression.paren_2 == ")"{
        operation = result.to_string();
    } else {
        //format sirve para concatenar los strings
        operation = format!("{}{}{}", expression.paren_1, &result.to_string(), expression.paren_2);
    }
    return operation;
}

fn calculator (equation: &mut String, operator: &Regex){
    let mut new_equation: String = equation.clone();

    loop {
        let caps: Option<Captures> = operator.captures(&new_equation);
        if caps.is_none() {
            break;
        }
        let caps_unwraped: Captures = caps.unwrap();
        let expression: Expression = destructure_caps(&caps_unwraped);
        let result: i16 = calculate(expression.var_1, expression.var_2, expression.symbol);
        let operation: String = get_operation(&expression, result);
        
        new_equation = new_equation.replace(expression.operation, &operation);
    }

    equation.clear();
    equation.push_str(&new_equation);
}