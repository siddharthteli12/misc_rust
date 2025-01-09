use std::thread;


fn main() {

    let handler = thread::spawn(|| {
        thread::spawn(|| {
            loop {
               println!("Hello");
            }
       });
       thread::spawn(|| {
        loop {
           println!("World");
        }
   });
    });
    handler.join().unwrap();
}
