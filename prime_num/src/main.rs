fn main() {
    println!("{:?}", prime_number(4));
}

fn prime_number(num: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(num);
    if num > 0 {
        result.push(2);
        if num > 1 {
            result.push(3);
        }
    }
    for i in 1..(num - 1) {
        result.push((i * 6) + 1);
    }
    result
}
