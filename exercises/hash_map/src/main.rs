use std::collections::HashMap;

fn main() {
    let mut basket: HashMap<String, u16> = HashMap::new();

    basket.insert("Banana".to_string(), 15);
    basket.insert("Apple".to_string(), 8);
    basket.insert("Mango".to_string(), 10);

    println!("{:?}", basket);

    let banana = "Banana";
    let mango = "Mango";

    println!("[ GET ] {} = {}", banana, basket[banana]);

    println!("[ GET ] {} = {:?}", mango, basket.get(mango));

    for (key, value) in basket {
        println!("{} = {}", key, value);
    }
}
