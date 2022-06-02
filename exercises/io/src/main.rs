fn main() {
    println!("Input name:");
    let mut name: String = String::new();

    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Input age:");
    let mut age: String = String::new();

    std::io::stdin().read_line(&mut age).unwrap();

    let age: u8 = age.trim().parse().unwrap();

    println!("> Name = {}, Age = {}", name, age);
}
