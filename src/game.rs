pub struct Game {
    name: String,
}

impl Game {
    pub fn new() -> Game {
        Game {
            name: String::from("Foo"),
        }
    }

    pub fn run(&self) {
        println!("Hello, {}.", self.name);
        println!("Goodbye, {}.", self.name);
    }
}
