use std::env;
use std::process;
use std::str::FromStr;

fn sum_to_zero(num: u128) -> u128 {
    let mut total_sum: u128 = 0;
    for i in (0..=num).rev() {
        total_sum += i as u128;
    }
    total_sum
}

fn is_valid_number(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_digit(10))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid Usage. Please provide an integer above 0.");
        process::exit(1);
    }

    let input = &args[1];

    if !is_valid_number(input) {
        eprintln!("Invalid Usage. Please provide an integer above 0.");
        process::exit(1);
    }

    let number = u128::from_str(input).unwrap_or_else(|_| {
        eprintln!("Invalid Usage. Please provide an integer above 0.");
        process::exit(1);
    });

    if number <= 0 {
        eprintln!("Invalid Usage. Please provide an integer above 0.");
        process::exit(1);
    }

    let result = sum_to_zero(number);
    println!("{} Poison does a total of {} damage.", number, result);
}
