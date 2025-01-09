use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<u32>().unwrap();
    println!("{:?}", factorial(input));
}

fn factorial(num: u32) -> u32 {
    if num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}
