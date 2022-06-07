// Returns:
// `i32` value
fn calculate_hat_price(price: i32) -> i32 {
    if price > 50 {
        price
    } else {
        price * 2
    }
}

// Returns:
// `()` Unit Type
fn get_price(value: i32) {
    println!("[ HAT PRICE ] = {}", value);
}

fn main() {
    let price_1 = calculate_hat_price(10);
    get_price(price_1);

    let price_2 = calculate_hat_price(400);
    get_price(price_2);
}
