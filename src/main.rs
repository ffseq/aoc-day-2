use std::convert::TryInto;
use std::fs;

fn main() {

    let filename: &str = "input.txt";

    let mut num_valid_passwords = 0;

    let content = fs::read_to_string(filename)
        .expect("File not found");
    for line in content.lines() {
        if valid_part2(line) {
            num_valid_passwords += 1;
        }
    }
    
    println!("Number of valid passwords: {}", num_valid_passwords);
}

fn valid_part1(line: &str) -> bool {

    let (policy, pass) = split1(line, ':');
    let pass = pass.trim();
    let (policy_range, policy_char) = split1(policy, ' ');
    let policy_char = policy_char.parse().unwrap();
    let (low, high) = split1(policy_range, '-');

    let range = std::ops::Range::<u32> {
        start: low.parse().unwrap(),
        end: high.parse::<u32>().unwrap() + 1
    };

    let mut count = 0;
    for c in pass.chars() {
        if c == policy_char {
            count += 1;
        }
    }
    
    range.contains(&count)
}

fn valid_part2(line: &str) -> bool {

    let (policy, pass) = split1(line, ':');
    let pass = pass.trim();
    let (policy_range, policy_char) = split1(policy, ' ');
    let policy_char: char = policy_char.parse().unwrap();
    let (low, high) = split1(policy_range, '-');

    let first = low.parse::<i32>().unwrap() - 1;
    let second = high.parse::<i32>().unwrap() - 1;

    if at(pass, first) == policy_char {
        return at(pass, second) != policy_char;
    }
    else if at(pass, second) == policy_char {
        return at(pass, first) != policy_char;
    }

    else {
        return false
    }
        
}

fn split1(input: &str, c: char) -> (&str, &str) {
    let splitted = input.splitn(2, c).collect::<Vec<&str>>();
    (splitted[0], splitted[1])
}

fn at(string: &str, idx: i32) -> char {
    string.chars().nth(idx.try_into().unwrap()).unwrap()
}
