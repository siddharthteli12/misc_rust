use static_init::dynamic;

// I lied this is not generic. We have to use concrete type with static types.
// Hence, using dynamic dispatch for this.
struct Tatti;

unsafe impl Send for Tatti {}
unsafe impl Sync for Tatti {}

impl Tatti {
    fn something(value: Self) -> Test<Box<dyn Send + Sync>> {
        let trait_value: Test<Box<dyn Send + Sync>> = Test {
            value: Box::new(value),
        };
        trait_value
    }
}

struct Test<T> {
    value: T,
}

#[dynamic]
static TEST: Test<Box<dyn Send + Sync>> = Tatti::something(Tatti);

fn main() {
    println!("Hello, world!");
}
