use colored::Colorize;

mod addition;
mod division;
mod game;
mod multiplication;
mod subtraction;

//start, with the selected mode
fn start_mode(mode: &str) -> bool {
    //modes possible
    match mode {
        "addition" | "+" | "add" => {
            game::start("+", &addition::operation, &addition::num_gen); //start addition prompt
            return true;
        }

        "subtraction" | "-" | "subtract" => {
            game::start("+", &subtraction::operation, &subtraction::num_gen); //start subtraction prompt
            return true;
        }

        "multiplication" | "*" | "x" | "X" | "multiply" => {
            game::start("*", &multiplication::operation, &multiplication::num_gen); //start subtraction prompt
            return true;
        }

        "division" | "/" | "÷" | "divide" => {
            game::start("/", &division::operation, &division::num_gen);
            return true;
        }

        _ => {
            println!("{}: {}", "Error".red().bold(), "Unknown command");
            return false;
        }
    }
}

//initialize cli
pub fn init() {
    //vector to store args
    let mut args: Vec<String> = std::env::args().collect();

    //switch args, check for certain cases
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
            //print prompt
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", "Select Mode: ".green());
            println!("addition {}", "+".green());
            println!("subtraction {}", "-".green());
            println!("multiplication {}", "*".green());
            println!("division {}", "/".green());
            println!("------------------");

            loop {
                //store input in chosen_mode variable
                let mut chosen_mode = String::new();
                std::io::stdin().read_line(&mut chosen_mode).unwrap();

                //reference to chosen_mode
                let mut chosen_mode = &*chosen_mode;

                chosen_mode = chosen_mode.trim();

                //check if start mode is one of the chosen modes, then break;
                if start_mode(chosen_mode) {
                    break;
                }
            }
        }
    }
}

//function to print the help message
pub fn print_help_message() {
    return println!(
        "
Memocalc Version 1.0.0

USAGE:
memocalc <action>

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

ACTIONS:
    addition, +, add              Practive addition problems
    subtraction, -, subtract      Practice subtraction problems
    multiplication, *, multiply   Practice multiplication problems
    division, /, divide           Practice division problems

ARGS:
    <action>         The type of action performed
        "
    );
}

//function to print a cli error
pub fn _error(error_message: &str, usage: &str) {
    let colored_error: &str = &"ERROR:";

    println!(
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
