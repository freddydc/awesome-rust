fn main() {
    let x = 20;
    let y = 5;

    let result = x + y;

    loop {
        println!("Type your solution [ {} + {} = ? ] ", x, y);

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        let user_input: i16 = user_input.trim().parse().unwrap();

        if user_input == result {
            println!("[ CORRECT ] {} + {} = {}", x, y, user_input);
            break;
        } else {
            println!("[ INCORRECT ] {} + {} = {}", x, y, user_input);
        }
    }
}
