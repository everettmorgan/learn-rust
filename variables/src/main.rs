fn main() {
    // Mutability
    // let x = 5;
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("in.y = {}", y);
    }

    println!("out.y = {}", y);

    // Valid
    let spaces = "   ";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // Invalid
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
