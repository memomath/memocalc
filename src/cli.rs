use colored::Colorize;

mod addition;
mod subtraction;

fn start_mode(mode: &str){
    match mode {
        "addition" => {
            addition::start_addition()
        }

        "subtraction" => {
            subtraction::start_subtraction()
        }
        _ => {
            error("Mode not found","mentical <mode types>")
        }
    }
}


pub fn init() {
    let mut args: Vec<String> = std::env::args().collect();

    match args.get_mut(1) {
        Some(x) => match &*x.to_lowercase() {
            "--help" | "-h" => {
                print_help_message();
            }
            "--version" | "-v" => {
                println!("{}", "1.0.0");
            }
            _ => { 
                start_mode(&*x.to_lowercase());
            }
        },
        None => {
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", "Select Mode: ".green());
            println!("addition {}", "+".green());
            println!("subtraction {}", "-".green());
            println!("\n")

            
        }
    }
}

pub fn print_help_message() {}

pub fn error(error_message: &str, usage: &str) {
    let colored_error: &str = &"ERROR:";

    return println!(
        "
{} {}
USAGE:
{}
For more information, try the command {}
",
        colored_error.red().bold(),
        error_message,
        usage,
        "--help".green().bold()
    );
}
