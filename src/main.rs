use std::env;

fn valid_commands(command: &String) -> bool {
    let available_commands = ["-h", "-n", "-i"];
    for temp in available_commands {
        if command == temp {
            return true;
        }
    }
    return false;
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let enviroment_variables = env::vars();
    let mut variable_name = arguments[arguments.len() - 1].clone();

    match arguments.len() {
        2 => {
            if variable_name == "-h" {
                println!("Envgrep returns enviroment variables and their value similar to the argument the user gave. \n\tenvgrep [options] <NAME> \nThe follwoing options are valid: \n'-h' Prints this help window \n'-n' Only prints the names of matching variables \n'-i' Ignores casing for the input")
            } else if variable_name == "-n" || variable_name == "-i" {
                println!("ERROR: Invalid arguments, try 'envgrepp -h' for help.")
            } else {
                for (key, value) in enviroment_variables {
                    if let Some(_) = key.find(&variable_name) {
                        println!("{} = {}", key, value);
                    }
                }
            }
        }
        3 => {
            if !valid_commands(&variable_name) {
                if arguments[1] == "-n" {
                    for (key, _) in enviroment_variables {
                        if let Some(_) = key.find(&variable_name) {
                            println!("{}", key);
                        }
                    }
                } else if arguments[1] == "-i" {
                    variable_name = variable_name.to_lowercase();
                    for (key, value) in enviroment_variables {
                        if let Some(_) = key.to_lowercase().find(&variable_name) {
                            println!("{} = {}", key, value);
                        }
                    }
                } else if arguments[1] == "-h" {
                    println!("Returns the found variables for the given argument.")
                } else {
                    println!("ERROR: Invalid arguments, try 'envgrep -h' for help.")
                }
            } else {
                println!("ERROR: Invalid arguments, try 'envgrep -h' for help.")
            }
        }
        4 => {
            if arguments[1] == arguments[2] || arguments[1] == "-h" || arguments[2] == "-h" {
                println!("ERROR: Invalid argument combination.")
            } else {
                variable_name = variable_name.to_lowercase();
                for (key, value) in enviroment_variables {
                    if let Some(_) = key.to_lowercase().find(&variable_name) {
                        println!("{} = {}", key, value);
                    }
                }
            }
        }
        _ => println!("ERROR: Invalid number of arguments, try 'envgrep -h' for help."),
    }
}
