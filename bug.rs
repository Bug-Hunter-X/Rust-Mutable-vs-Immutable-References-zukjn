fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10; // modify x through y
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // prints 10
    println!("z = {}", *z); // prints 10
    // This line will cause a compiler error
    // *z = 20;
}