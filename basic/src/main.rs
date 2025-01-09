
fn main() {
    let height = 10;
    let width = 7;
    let mid_width = 6;


    for i in 0..height {
        if i == 0 {
            let values = "#".repeat(width);
            println!("{}", values);
        } else if i == height / 2 {
            let values = "#".repeat(mid_width);
            println!("{}", values);
        } else {
            println!("#");
        }
    }
}