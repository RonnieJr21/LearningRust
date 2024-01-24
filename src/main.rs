// this is a simple calculator
use chrono;
use core::str;
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let cmd_line_args: Vec<String> = std::env::args().collect();
    if cmd_line_args.len() > 4 || cmd_line_args.len() <= 0 {
        usage();
    }

    println!("{:?}", cmd_line_args);

    let math: Calculator = Calculator {
        num1: cmd_line_args[1].parse().unwrap(),
        operation: Calculator::parse_input(&cmd_line_args[2]).unwrap(),
        num2: cmd_line_args[3].parse().unwrap(),
    };

    let result = LocalFile {
        title: String::from("Calculator_Results.txt"),
        content: vec![
            math.num1.to_string(),
            cmd_line_args[2].to_string(),
            math.num2.to_string(),
            String::from(" = "),
            Calculator::do_operation(&math).to_string(),
        ],
        date: chrono::offset::Local::now().timestamp().to_string(),
    };

    Calculator::store_results(result);

    println!("= {}", Calculator::do_operation(&math));
}

// get the users input
fn usage() {
    println!("To use the program please enter your numbers and operator in this order: <number 1> <operation> <number 2>");
}
struct Calculator {
    num1: f32,
    num2: f32,
    operation: Operations,
}

enum Operations {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Calculator {
    fn parse_input(args: &str) -> Result<Operations, &'static str> {
        match args {
            "+" => Ok(Operations::Add),
            "-" => Ok(Operations::Subtract),
            "x" => Ok(Operations::Multiply),
            "/" => Ok(Operations::Divide),
            _ => Err("There was an error"),
        }
    }

    fn do_operation(values: &Calculator) -> f32 {
        match values.operation {
            Operations::Add => values.num1 + values.num2, /* Adding the values */
            Operations::Divide => values.num1 / values.num2, /* Dividing the values */
            Operations::Multiply => values.num1 * values.num2, /* Multiplying the values */
            Operations::Subtract => values.num1 - values.num2, /* Subtracting the values */
        }
    }

    fn store_results(file: LocalFile) {
        let mut data_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file.title)
            .expect("cannot open file");
        let file_data = "Date: ".to_string()
            + &file.date
            + &" Result: ".to_string()
            + &file.content.as_slice().concat();

        let _ = data_file.write("\n".as_bytes());
        let _ = data_file.write(file_data.as_bytes());
    }
}

struct LocalFile {
    title: String,
    content: Vec<String>,
    date: String,
}
