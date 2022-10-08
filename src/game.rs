use sdl2;

use sdl2::event::Event;

pub struct Game {
    name: &'static str,
    sdl_context: sdl2::Sdl,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let sdl_context = sdl2::init()?;

        Ok(Game {
            name: "Bellator",
            sdl_context,
        })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("Hello, world!");

        let video_subsystem = self.sdl_context.video()?;

        let timer = self.sdl_context.timer()?;

        let _window = video_subsystem
            .window(self.name, 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut event_pump = self.sdl_context.event_pump()?;

        let update_period = 16;
        let minimum_render_period = 16;
        let maximum_render_period = 1000;

        let offset_tick = timer.ticks();

        let mut current_tick = 0;
        let mut last_update_tick = 0;
        let mut last_render_tick = 0;

        self.update(last_update_tick);
        self.render();

        'main : loop {
            let next_render = last_render_tick + minimum_render_period;
            let next_render_limit = last_render_tick + maximum_render_period;

            loop {
                let timeout = if next_render <= current_tick { 0 } else { next_render - current_tick };
                let wait_event = event_pump.wait_event_timeout(timeout);
                current_tick = timer.ticks() - offset_tick;

                while (last_update_tick + update_period) <= current_tick {
                    last_update_tick += update_period;
                    self.update(last_update_tick);
                }

                let event = match wait_event {
                    None => break,
                    Some(Event::Quit { .. }) => break 'main,
                    Some(event) => event,
                };

                self.handle_event(last_update_tick, event);

                current_tick = timer.ticks() - offset_tick;
                if current_tick >= next_render_limit {
                    break;
                }
            }

            if last_render_tick != last_update_tick {
                self.render();
                last_render_tick = last_update_tick;
            }

            current_tick = timer.ticks() - offset_tick;

        }

        println!("Goodbye, world.");

        Ok(())
    }

    fn handle_event(&self, last_update_tick: u32, _event: Event) {
        println!("handle_event({last_update_tick}, <event>);");
    }

    fn update(&self, millis: u32) {
        println!("update({millis});");
    }

    fn render(&self) {
        println!("render();");
    }
}
