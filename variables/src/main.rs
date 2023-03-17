// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; # constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
const DEFAULT_VALUE: u32 = 100_000;
fn main() {
    let mut x: u32 = DEFAULT_VALUE;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");



    //Shadowing
  
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
}