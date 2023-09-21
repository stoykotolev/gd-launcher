use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
};

fn check_number(secured_numbers: &Vec<u128>, target: u128) -> bool {
    let mut checked_numbers_set: HashSet<u128> = HashSet::new();
    for &secured_number in secured_numbers {
        if secured_number > target {
            continue;
        }

        let remaining = target - secured_number;

        if checked_numbers_set.contains(&remaining) {
            return true;
        }
        checked_numbers_set.insert(secured_number);
    }

    false
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt").expect("no such file");
    let file_reader = BufReader::new(file);

    let mut secured_numbers: Vec<u128> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Ok(line_content) => {
                let number = line_content.parse::<u128>().expect("The line to be parsed");

                if secured_numbers.len() >= 100 {
                    if !check_number(&secured_numbers, number) {
                        println!("{number} is an insecure number");
                        break;
                    }
                    secured_numbers.remove(0);
                }

                secured_numbers.push(number);
            }
            Err(err) => eprintln!("Couldn't get line: {:?}", err),
        }
    }

    Ok(())
}
