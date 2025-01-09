use std::io::stdin;
fn main() {
    let mut pyramid_height = String::new();
    let _ = stdin().read_line(&mut pyramid_height).unwrap();
    let pyramid_height = pyramid_height.trim().parse::<u32>().unwrap();

    let total_width = (pyramid_height * 2) - 1;

    for i in 1..pyramid_height {
        let i = (i * 2) - 1;
        let white_space = (total_width - i) as usize;
        print!("{}", " ".repeat(white_space / 2));
        print!("{}", "*".repeat(i as usize));
        println!("{}", " ".repeat(white_space / 2));
    }
}
