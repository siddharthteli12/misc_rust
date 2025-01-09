trait Test {
    fn test() -> Box<dyn Test>
    where
        Self: Sized;
}

struct Wrapper {
    custom: Box<dyn Test>,
}

struct Custom {
    name: String,
}

impl Custom {
    fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Test for Custom {
    fn test() -> Box<dyn Test> {
        Box::new(Custom::new())
    }
}

fn main() {
    let wrapper = Wrapper {
        custom: Custom::test(),
    };

    println!("Hello, world!");
}
