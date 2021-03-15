use colored::Colorize;

mod addition;
mod subtraction;

fn start_mode(mode: &str) -> bool {
    match mode {
        "addition" => {
            addition::start();
            return true;
        }

        "subtraction" => {
            subtraction::start();
            return true;
        }
        _ => {
            return false;
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
            println!("\n");

            loop {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let mut line = &*line;

                line = line.trim();

                if start_mode(line) {
                    break;
                }
            }
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
