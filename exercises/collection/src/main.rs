use std::io;

fn main() {
    // Vector
    let mut users: Vec<String> = Vec::new();
    // Range:
    // 0..3
    // `_`:
    // Ignore values
    for _ in 0..3 {
        println!("Add your users:");

        let mut user_entry = String::new();
        io::stdin().read_line(&mut user_entry).unwrap();

        let user_entry = user_entry.trim().to_uppercase();
        users.push(user_entry);
    }

    println!("[ GET USER ] = {}", users[2]);

    for user in users {
        println!("- {:?}", user);
    }

    // Array
    let scores = ["GOLD", "SILVER"];

    println!("[ GET SCORE ] = {}", scores[0]);

    for score in scores {
        println!("- {:?}", score)
    }
}
