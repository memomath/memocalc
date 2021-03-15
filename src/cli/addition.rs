use colored::Colorize;
use rand::Rng;
use std::io;
use std::io::Write;

pub fn start() {
    println!("Number of Digits?");

    let mut number_of_numbers_b = String::new();
    std::io::stdin()
        .read_line(&mut number_of_numbers_b)
        .unwrap();
    let number_of_numbers = (&*number_of_numbers_b).trim().parse::<i32>().unwrap();

    let mut ranges: Vec<i32> = Vec::new();

    for i in 0..number_of_numbers {
        println!("Range of number {}", i + 1);
        let mut range = String::new();

        std::io::stdin().read_line(&mut range).unwrap();
        let rangei = (&*range).trim().parse::<i32>().unwrap();

        ranges.push(rangei);
    }

    let mut rng = rand::thread_rng();

    loop {
        let mut numbers: Vec<i32> = Vec::new();

        for i in 0..number_of_numbers {
            let num = rng.gen_range(0..(ranges[i as usize] + 1));
            numbers.push(num);

            if i == number_of_numbers - 1 {
                print!("{} = ", num);
                io::stdout().flush().unwrap();
            } else {
                print!("{} + ", num);
                io::stdout().flush().unwrap();
            }
        }

        let sum: i32 = numbers.iter().sum();

        let mut inputed_b = String::new();
        std::io::stdin().read_line(&mut inputed_b).unwrap();
        let inputed = (&*inputed_b).trim().parse::<i32>().unwrap();

        if inputed == sum {
            println!("{}", "Correct".green().bold());
        } else {
            println!(
                "{}",
                "Incorrect!".red().bold(),
            ); 
            println!("{}{}", "Correct Answer: ".green().bold(), sum.to_string().green().bold());
        }
    }
}
