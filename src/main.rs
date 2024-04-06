fn tt(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}

fn main() {
    println!("Hello, world!");

    let a = 10;
    let b = 20;
    let c = tt(a, b);
    println!("c = {}", c);
}
