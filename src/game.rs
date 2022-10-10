use std::time::Duration;
use std::time::Instant;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::scene::Scene;

pub struct Game {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    current_scene: Scene,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let name = "Bellator";

        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(name, 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let current_scene = Scene::new()?;

        Ok(Game {
            sdl_context,
            canvas,
            current_scene,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("Hello, world!");

        let mut event_pump = self.sdl_context.event_pump()?;

        let update_interval = Duration::from_millis(16);
        let minimum_render_interval = Duration::from_millis(16);
        let maximum_render_interval = Duration::from_secs(1);

        let current_time_offset = Instant::now();
        let mut current_time = Duration::ZERO;
        let mut last_update = Duration::ZERO;
        let mut last_render = Duration::ZERO;

        self.update(last_update);
        self.render();

        'main : loop {
            let next_render = last_render + minimum_render_interval;
            let next_render_limit = last_render + maximum_render_interval;

            'event : loop {
                let timeout = if next_render <= current_time { Duration::ZERO } else { next_render - current_time };
                let event_option = event_pump.wait_event_timeout(u32::try_from(timeout.as_millis()).unwrap());
                current_time = Instant::now() - current_time_offset;

                while (last_update + update_interval) <= current_time {
                    last_update += update_interval;
                    self.update(last_update);
                }

                let event = match event_option {
                    None => break 'event,
                    Some(Event::Quit { .. }) => break 'main,
                    Some(event) => event,
                };

                self.handle_event(last_update, event);

                current_time = Instant::now() - current_time_offset;
                if current_time >= next_render_limit {
                    break;
                }
            }

            if last_render != last_update {
                self.render();
                last_render = last_update;
            }

            current_time = Instant::now() - current_time_offset;

        }

        println!("Goodbye, world.");

        Ok(())
    }

    fn handle_event(&self, instant: Duration, event: Event) {
        match event {
            Event::KeyDown {
                repeat: false,
                keycode: Some(keycode),
                ..
            } => self.current_scene.handle_key(instant, keycode, true),
            Event::KeyUp {
                repeat: false,
                keycode: Some(keycode),
                ..
            } => self.current_scene.handle_key(instant, keycode, false),
            _ => return,
        };
    }

    fn update(&self, instant: Duration) {
        self.current_scene.update(instant);
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.current_scene.render(&mut self.canvas);
        self.canvas.present();
    }
}
