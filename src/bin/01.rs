use std::collections::VecDeque;

use advent_of_code_23::read_file;

const TEST_FILE1: &str = "assets/01-example1.txt";
const TEST_FILE2: &str = "assets/01-example2.txt";
const INPUT_FILE: &str = "assets/01-input.txt";

fn main() {
    let example = read_file(TEST_FILE1);
    assert_eq!(parse1(&example), 142);

    let out1 = parse1(&read_file(INPUT_FILE));
    println!("First out is {}", out1);

    let example = read_file(TEST_FILE2);
    assert_eq!(parse2(&example), 281);

    let out2 = parse2(&read_file(INPUT_FILE));
    println!("Second out is {}", out2);
}

fn find_first_last_digit(input: &str) -> u32 {
    let cvec: Vec<char> = input.chars().collect();
    let first = input.find(char::is_numeric).unwrap();
    let second = input.rfind(char::is_numeric).unwrap();

    let a = cvec[first];
    let b = cvec[second];
    format!("{}{}", a, b).parse::<u32>().unwrap()
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_any_digit_str(stack: &VecDeque<char>) -> Option<usize> {
    for (i, word) in WORDS.iter().enumerate() {
        let mut vec: Vec<char> = Vec::with_capacity(word.len());
        for si in stack.len() - word.len()..stack.len() {
            let stack_char = stack[si];
            vec.push(stack_char);
        }
        let v: String = vec.iter().collect();
        if word.to_string() == v {
            return Some(i + 1);
        }
    }
    None
}

fn find_digit_or_str(input: &str) -> u32 {
    let mut stack: VecDeque<char> = VecDeque::from([' '; 5]);
    let mut values = Vec::new();

    for c in input.chars() {
        if stack.len() == 5 {
            stack.pop_front();
        }
        stack.push_back(c);
        let a = find_any_digit_str(&stack);
        if let Some(a) = a {
            values.push(char::from_digit(a as u32, 10).unwrap());
        } else {
            if c.is_numeric() {
                values.push(c);
            }
        }
    }
    format!("{}{}", values[0], values.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn parse1(input: &str) -> u32 {
    parse(input, find_first_last_digit)
}

fn parse(input: &str, f: fn(&str) -> u32) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let out = f(line);
        sum += out;
    }
    sum
}

fn parse2(input: &str) -> u32 {
    parse(input, find_digit_or_str)
}
