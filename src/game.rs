use sdl2;

pub struct Game {
    sdl_context: sdl2::Sdl,
    name: String,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let sdl_context = sdl2::init()?;

        Ok(Game {
            sdl_context,
            name: String::from("Foo"),
        })
    }

    pub fn run(&self) {
        println!("Hello, {}.", self.name);
        println!("Goodbye, {}.", self.name);
    }
}
