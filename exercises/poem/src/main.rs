use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("poem.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        match line {
            Ok(text) => {
                let data: Vec<&str> = text.split(",").collect();
                println!("{:?}", data)
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
