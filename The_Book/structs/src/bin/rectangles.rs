#[derive(Debug)]
struct Rectangle {
    hight: u64,
    width: u64
}

fn main() {
    let rect = Rectangle {
        hight: 30,
        width: 50
    };

    println!("rect: {:#?}", rect);

    println!(
        "the area is {}", 
        area(&rect)

    );
}

fn area(rectangle: &Rectangle) -> u64 {
    rectangle.hight * rectangle.width
}