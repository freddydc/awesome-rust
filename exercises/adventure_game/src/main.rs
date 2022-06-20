use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Story {
    tag: String,
    message: String,
    health: i32,
    choice: Vec<Choice>,
}

#[derive(Debug)]
struct Choice {
    tag: String,
    message: String,
}

impl Story {
    fn new(value: &[&str]) -> Self {
        let tag = value[1].to_string();
        let message = value[2].to_string();
        let health: i32 = value[3].parse().unwrap_or(0);
        let choice: Vec<Choice> = Vec::new();

        Story {
            tag,
            message,
            choice,
            health,
        }
    }
}

const FILENAME: &str = "story.txt";
const DEFAULT_TAG: &str = "BEGINNING";

fn main() {
    let mut health = 100;
    let mut selected_tag = DEFAULT_TAG;
    let mut tag_record = String::new();

    let mut store_adventure: HashMap<String, Story> = HashMap::new();

    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        match line {
            Ok(data) => {
                if !data.is_empty() {
                    let parse: Vec<&str> = data.split(';').map(|value| value.trim()).collect();

                    let story = Story::new(&parse);
                    let selector = parse[0];

                    if selector == "SITUATION" {
                        tag_record = story.tag.clone();
                        store_adventure.insert(story.tag.clone(), story);
                    } else if selector == "CHOICE" {
                        if let Some(adventure) = store_adventure.get_mut(&tag_record) {
                            let choice = Choice {
                                tag: story.tag.clone(),
                                message: story.message.clone(),
                            };
                            adventure.choice.push(choice);
                        }
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    loop {
        if let Some(adventure) = store_adventure.get(selected_tag) {
            println!("== [ LIFE {} ] ==", health);
            println!("{}", adventure.message);

            for (n, choice) in adventure.choice.iter().enumerate() {
                println!("== [ {} ] == {}", n, choice.message);
            }

            let mut choice_entry = String::new();
            io::stdin().read_line(&mut choice_entry).unwrap();
            let choice_entry: usize = choice_entry.trim().parse().unwrap_or(0);

            if !adventure.choice.is_empty() {
                if let Some(choice) = adventure.choice.get(choice_entry) {
                    selected_tag = &choice.tag;
                } else {
                    println!("== [ INVALID CHOICE ] ==");
                }
            }

            health += adventure.health;

            println!()
        } else {
            break;
        }

        if health <= 0 {
            println!("== [ GAME OVER ] ==");
            break;
        }
    }
}
