use std::io;

fn prompt(coordinate: char) -> f64 {
    let parsed: f64;
    let mut input = String::new();

    println!("Enter {coordinate} coordinate");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    parsed = input
        .trim()
        .parse()
        .expect("Not a number!");

    parsed
}

fn main() {
    let point: (f64, f64) = (
        prompt('x'),
        prompt('y'),
    );

    let x = point.0;
    let y = point.1;

    println!("Point({x}, {y})");
}