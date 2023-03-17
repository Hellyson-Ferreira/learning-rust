fn scalar_types() {
    println!("Hello, world!");

    let __guess: u32 = "42".parse().expect("Não é um número!");

    let __x = 2.0; // f64

    let __y: f32 = 3.0; // f32

    //////////////////////////
    
    // addition
    let __sum = 5 + 10;

    // subtraction
    let __difference = 95.5 - 4.3;

    // multiplication
    let __product = 4 * 30;

    // division
    let __quotient = 56.7 / 32.2;
    let __truncated = -5 / 3; // Results in -1

    // remainder
    let __remainder = 43 % 5;



    let __t = true;

    let __f: bool = false; // with explicit type annotation


    let __c = 'z';
    let __z = 'ℤ';
    let __heart_eyed_cat = '😻';

}
