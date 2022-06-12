#[derive(Debug)]
struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Greetings {}!", self.name);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Tuple
#[derive(Debug)]
struct Point(u16, u16, u16);

// Unit Like
#[derive(Debug)]
struct UnitStruct;

fn main() {
    let admin = User {
        name: String::from("max"),
    };

    println!("> {:?}", admin);

    println!("[ Get Name ] = {}", admin.name);

    admin.greet();

    let rectangle_square = Rectangle::square(5);
    let result = rectangle_square.area();

    println!("[ Rectangle Area ] = {}", result);

    let origin = Point(10, 20, 30);
    println!("> {:?}", origin);
    println!("[ Get Point ] = {}", origin.1);

    let unit_struct = UnitStruct;
    println!("> {:?}s are useful!", unit_struct);
}
