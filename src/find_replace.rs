use regex::Regex;
use std::env;
use std::fs;
use std::process;
use text_colorizer::{self, Colorize};

struct Arguments {
    find: String,
    replace: String,
    input: String,
    output: String,
}

pub fn run() {
    let args_vector: Vec<String> = env::args().skip(1).collect();
    let len = args_vector.len();
    if len != 3 && len != 4 {
        help();
    }
    let output = match args_vector.get(3) {
        Some(v) => String::from(v),
        None => args_vector[2].to_string(),
    };

    let args = Arguments {
        find: args_vector[0].to_string(),
        replace: args_vector[1].to_string(),
        input: args_vector[2].to_string(),
        output,
    };
    find_and_replace(&args)
}

fn find_and_replace(args: &Arguments) {
    let data = match fs::read_to_string(&args.input) {
        Ok(data) => data,
        Err(e) => {
            let message = format!("Failed to read from file {}, {:?}", &args.input, e);
            error(&message);
        }
    };
    let replace_data = match replace(&args.find, &args.replace, &data) {
        Ok(data) => data,
        Err(e) => {
            let message = format!("Fail to replace text {:?}", e);
            error(&message)
        }
    };
    match fs::write(&args.output, replace_data) {
        Ok(_) => {}
        Err(e) => {
            let message = format!("Fail to write to {}, {:?}", &args.output, e);
            error(&message)
        }
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    let replacement = String::from(regex.replace_all(data, rep));
    Ok(replacement)
}

fn help() {
    eprintln!("{}:", "Usage".green());
    eprintln!(" <Search string> <Replacement string> <INPUT FILE>");
    eprintln!(" <Search string> <Replacement string> <INPUT FILE> <OUTPUT FILE>");
    process::exit(1);
}

fn error(message: &str) -> ! {
    eprintln!("{}: {}", "Error".red().bold(), message);
    process::exit(1)
}
