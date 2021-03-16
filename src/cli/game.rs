use colored::Colorize;
use rand::Rng;
use std::io;
use std::io::Write;
use std::time::{Instant};

//start function
pub fn start(sign: &str, ans: &dyn Fn(Vec<i32>)->i32, num_gen: &dyn Fn(i32)-> i32) {
    println!("{}", "Number of Digits?".green());

    let mut number_of_digits_buffer = String::new(); //string to collect input

    std::io::stdin() //input
        .read_line(&mut number_of_digits_buffer) //store in variable
        .unwrap();
    let number_of_digits = (&*number_of_digits_buffer).trim().parse::<i32>().unwrap(); //parse raw input into i32

    let mut ranges: Vec<i32> = Vec::new(); //vector of ranges

    //for 0 to the number of numbers, do
    for i in 0..number_of_digits {
        let number = i + 1;
        println!("{}: {}", "Range of number".green(), number.to_string().green());
        let mut range = String::new(); //string to store the range input

        std::io::stdin()
            .read_line(&mut range)  //store input in range variable
            .unwrap();

        let range_int = (&*range).trim().parse::<i32>().unwrap(); //parse into a i32
        ranges.push(range_int); //push range_int to the vector 
    }

    //let mut rng = rand::thread_rng(); //generate random numbers based on the range

    //Loop the questions
    loop {
        let mut numbers: Vec<i32> = Vec::new(); // make vector to store current equation numbers
        // well how are you going to make this modular?
        //  i have an idea
        for i in 0..number_of_digits {
            let num = num_gen(ranges[i as usize]); // generate a number
            numbers.push(num); // add number to vector

            if i == number_of_digits - 1 {  //checks if its at the last inputed digit
                print!("{} = ", num);
            } else {
                print!("{} {} ", num, sign);
            }
            io::stdout().flush().unwrap(); //allows you to not have to print a new line
        }

        let sum: i32 = ans(numbers); //adds every element in the vector

        let timestamp = Instant::now();

        let mut answer_buffer = String::new(); //raw input for answer
        std::io::stdin().read_line(&mut answer_buffer).unwrap(); //store in answer_buffer var

        let answer_string: &str = (&*answer_buffer).trim(); //answer input as a string to check for string commands
        
        if answer_string == "exit" {
            std::process::exit(0x0100);
        }

        let answer_int = answer_string.parse::<i32>().unwrap(); //answer input as a i32 to check for int commands
        let duration = timestamp.elapsed(); //duration from when the timestamp was started


        //if the answer is correct
        if answer_int == sum {

            println!("{} ({:?})", "Correct!".green().bold(), duration);

        } else if answer_int != sum { //if the answer is incorrect,
            println!("{} ({:?})", "Incorrect!".red().bold(), duration);
            println!(
                "{}{}",
                "Correct Answer: ".green().bold(),
                sum.to_string().green().bold()
            );
        } else {
            continue;
        }
    }
}
