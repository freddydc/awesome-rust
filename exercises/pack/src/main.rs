use regex::Regex;
use std::io;

fn main() {
    // Match:
    // 5 + 5
    // 2 * 5
    let re_sum = Regex::new(r#"(\d+)\s?\+\s?(\d+)"#).unwrap();
    let re_mul = Regex::new(r#"(\d+)\s?\*\s?(\d+)"#).unwrap();

    // Operation:
    // 10 + 2 + 8
    // 2 * 5 + 10
    println!("Enter operation:");

    let mut text_entry = String::new();
    io::stdin().read_line(&mut text_entry).unwrap();

    loop {
        let caps = re_mul.captures(&text_entry);

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let entry = caps.get(0).unwrap().as_str();
        let x: i16 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y: i16 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = x * y;

        text_entry = text_entry.replace(entry, &result.to_string());
    }

    loop {
        let caps = re_sum.captures(&text_entry);

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let entry = caps.get(0).unwrap().as_str();
        let x: i16 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y: i16 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = x + y;

        text_entry = text_entry.replace(entry, &result.to_string());
    }

    println!("[ RESULT ] = {}", text_entry);
}
