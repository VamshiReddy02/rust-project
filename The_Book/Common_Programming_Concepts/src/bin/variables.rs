fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    let shadow = "example of shadowing";
    print!("{}",shadow);
    let shadow = "will be printed without error";
    print!("{}", shadow)
}