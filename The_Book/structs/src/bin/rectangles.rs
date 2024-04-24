#[derive(Debug)]
struct Rectangle {
    hight: u64,
    width: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.hight * self.width
    }
}

fn main() {
    let rect = Rectangle {
        hight: 30,
        width: 50
    };

    println!("rect: {:#?}", rect);

    println!(
        "the area is {}", 
        rect.area()

    );
}

