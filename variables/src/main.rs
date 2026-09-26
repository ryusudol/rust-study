// Constant convention
// - variable names consist of capital letters and underscores only
// - they are ALWAYS immutable -> can't make it mutable
// - the type of the value must be annotated
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // variables are immutable by default
    let x = 5;
    // x = 6;  <-- this will cause a compile-time error.
    println!("The value of x: {x}\n");

    // we can make variables mutable
    let mut x = 5;
    println!("The value of x: {x}");
    x = 6;
    println!("The value of x: {x}\n");

    // shadowing
    let x = 5;
    let x = x + 1;
    println!("The value of x: {x}");
    {
        let x = x * 2;
        println!("The value of x: {x}");
    }
    println!("The value of x: {x}\n");

    println!("The value of the constant: {THREE_HOURS_IN_SECONDS}");
}
