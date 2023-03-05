use std::io::{stdin};
use regex::{Regex, Captures};

struct RegexOperators {
    sum: Regex,
    substract: Regex,
    divide: Regex,
    multiply: Regex,
}

struct Expression {
    equation: String,
    parenthesis_1: String,
    parenthesis_2: String,
    variable_1: i16,
    variable_2: i16,
    symbol: String
}

pub fn exercise_calculator() {
    println!("Digit the equation.");

    //factorizar a uno solo y que los demas solo sean obtenidos, por un replace.
    let operators = RegexOperators {
        sum: Regex::new(r"(\(?)(\d+)(\+)(\d+)(\)?)").unwrap(), 
        substract: Regex::new(r"(\(?)(\d+)(\-)(\d+)(\)?)").unwrap(), 
        divide: Regex::new(r"(\(?)(\d+)(/)(\d+)(\)?)").unwrap(), 
        multiply: Regex::new(r"(\(?)(\d+)(\*)(\d+)(\)?)").unwrap(), 
    };

    // operators.divide.
    
    let mut expression: String = String::new();
    stdin().read_line(&mut expression).unwrap();

    let expression_no_spaces: String = expression.replace(" ", "");

    let mut answer: String = calculator(expression_no_spaces, operators.divide);
    answer = calculator(answer, operators.multiply);
    answer = calculator(answer, operators.sum);
    answer = calculator(answer, operators.substract);

    println!("The answer is: {answer}");
}

fn destructure_caps (caps: Captures) -> Expression{
    let equation = Expression {
        parenthesis_1: caps.get(1).unwrap().as_str().to_string(),
        parenthesis_2: caps.get(5).unwrap().as_str().to_string(),
        symbol: caps.get(3).unwrap().as_str().to_string(),
        variable_1: caps.get(2).unwrap().as_str().parse().unwrap(),
        variable_2: caps.get(4).unwrap().as_str().parse().unwrap(),
        equation: caps.get(0).unwrap().as_str().to_string(),
    };

    return equation;
}

fn calculate (variable_1: i16, variable_2: i16, symbol: &str) -> i16{
    let result: i16 = match symbol {
        "+" => variable_1 + variable_2,
        "-" => variable_1 - variable_2,
        "*" => variable_1 * variable_2,
        "/" => if variable_2 == 0 {
            0
        } else {
            variable_1 / variable_2
        },
        _ => 0,
    };

    return result;
}

fn get_operation (equation: &Expression, result: i16) -> String {
    let new_operation: String;
    if equation.parenthesis_1 == "(" && equation.parenthesis_2 == ")"{
        new_operation = result.to_string();
    } else {
        //format sirve para concatenar los strings
        new_operation = format!("{}{}{}", equation.parenthesis_1, &result.to_string(), equation.parenthesis_2);
    }
    return new_operation;
}

fn calculator (expression: String, operator: Regex) -> String {
    let mut operation: String = expression.clone();

    loop {
        let caps: Option<Captures> = operator.captures(&operation);
        if caps.is_none() {
            break;
        }
        let caps_unwraped: Captures = caps.unwrap();
        let equation: Expression = destructure_caps(caps_unwraped);
        let result: i16 = calculate(equation.variable_1, equation.variable_2, equation.symbol.as_str());
        let new_operation: String = get_operation(&equation, result);
        
        operation = operation.replace(equation.equation.as_str(), &new_operation);
    }

    return operation;
}