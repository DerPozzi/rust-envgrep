use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let enviroment_variables = env::vars();
    let variable_name = arguments[arguments.len() - 1].clone();

    match arguments.len() {
        2 => {
            if variable_name != "-h" {
                for (key, value) in enviroment_variables {
                    if let Some(_) = key.find(&variable_name) {
                        println!("{} = {}", key, value);
                    }
                }
            } else {
                println!("Envgrep returns enviroment variables and their value that are similar to the name the user gave. \n\tenvgrep [options] <NAME> \nThe follwoing options are valid: \n'-h' Prints this help window \n'-n' Only prints matching names of matching variables \n'-i' Ignores casing for the input")
            }
        }
        3 => println!("{} - {}", arguments[1], arguments[2]),
        4 => println!("{} - {} - {}", arguments[1], arguments[2], arguments[3]),
        _ => println!("ERROR: Invalid number of arguments, try 'envgrep -h' for help."),
    }
}
