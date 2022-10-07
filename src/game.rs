pub struct Data {
    name: String,
}

pub trait Game {
    fn new() -> Data;

    fn run(&self);
}

impl Game for Data {
    fn new() -> Data {
        Data {
            name: String::from("Foo"),
        }
    }

    fn run(&self) {
        println!("Hello, {}.", self.name);
        println!("Goodbye, {}.", self.name);
    }
}
