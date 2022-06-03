fn main() {
    println!("Input key number:");
    let mut key: String = String::new();
    std::io::stdin().read_line(&mut key).unwrap();

    let key: u8 = key.trim().parse().unwrap();

    if key != 0 && key <= 3 {
        println!("[ SUCCESS ] = {}", key);
    } else if key == 0 {
        println!("[ EXACT KEY ] = {}", key);
    } else {
        println!("[ INVALID ] = {}", key);
    }
}
