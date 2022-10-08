use sdl2;

use sdl2::event::Event;

pub struct Game {
    sdl_context: sdl2::Sdl,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let sdl_context = sdl2::init()?;

        Ok(Game {
            sdl_context,
        })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("Hello, world!");

        let video_subsystem = self.sdl_context.video()?;

        let _window = video_subsystem
            .window("SDL2", 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut event_pump = self.sdl_context.event_pump()?;

        let mut running = true;
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        running = false;
                    }
                    Event::KeyUp { keycode, repeat, .. } => {
                        if repeat {
                            break;
                        }
                        println!("{} Up", keycode.unwrap());
                    }
                    Event::KeyDown { keycode, repeat, .. } => {
                        if repeat {
                            break;
                        }
                        println!("{} Down", keycode.unwrap());
                    }
                    _ => {}
                }
            }
        }

        println!("Goodbye, world.");

        Ok(())
    }
}
