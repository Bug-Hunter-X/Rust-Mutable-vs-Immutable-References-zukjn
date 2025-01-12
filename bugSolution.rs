fn main() {
    let mut x = 5;
    {   // create a new scope to limit the lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // modify x through y
    }
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // prints 10
    println!("z = {}", *z); // prints 10
}