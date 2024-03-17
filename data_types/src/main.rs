fn add(a: &u16, b: &u16) -> u16 {
    return a + b;
}

fn main() {
    let a: u16 = 5;
    let b: u16 = 4;
    let sum: u16 = add(&a, &b);

    println!("{} + {} = {}", a, b, sum);
}
