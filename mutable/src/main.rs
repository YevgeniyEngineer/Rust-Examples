fn main() {
    let x = 5; // Immutable variable
    println!("The value of x is {}", x);

    let mut y = 7; // Mutable variable
    println!("The value of y is {}", y);
    y = 10;
    println!("The value of y changed to {}", y);
}
