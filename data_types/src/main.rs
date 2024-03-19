fn main() {
    let a: u16 = 5;
    let b: u16 = 4;
    let sum: u16 = add(&a, &b);
    
    println!("{} + {} = {}", a, b, sum);

    let c: i16 = -5;
    let d: i16 = -10;
    let difference: i16 = subtract(&c, &d);

    println!("{} - {} = {}", c, d, difference);

    let e: i32 = -10;
    let f: i32 = 25;
    let product: i32 = multiply(&e, &f);

    println!("{} * {} = {}", e, f, product);

    let g: i32 = 56;
    let h: i32 = 7;
    let quotient1: i32 = divide(&g, &h).unwrap_or_else(|error| {
        println!("Error: {}", error);
        0
    });
    println!("{} / {} = {}", g, h, quotient1);

    let i: i32 = 2;
    let j: i32 = 0;

    match divide(&i, &j) {
        Result::Ok(quotient) => println!("{} / {} = {}", i, j, quotient),
        Result::Err(err) => println!("Cannot divide by zero: {}", err)
    };    
}

fn add(a: &u16, b: &u16) -> u16 {
    return a + b;
}

fn subtract(a: &i16, b: &i16) -> i16 {
    return a - b;
}

fn multiply(a: &i32, b: &i32) -> i32 {
    return a * b;
}

fn divide(a: &i32, b: &i32) -> Result<i32, &'static str> {
    if *b == 0 {
        Result::Err("Division by zero is not allowed")
    } else {
        Result::Ok(a / b)
    }
}