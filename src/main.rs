mod chip8;
mod keyboard;
mod monitor;

use chip8::*;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    // Change self.to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("chip8", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut chip8 = Chip8::new(opengl);

    let mut file = std::fs::File::open("roms/TANK")?;
    let mut program = Vec::new();
    let chunk_size = 0x10;
    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)?;
        if n == 0 {
            break;
        }
        program.append(&mut chunk);
        if n < chunk_size {
            break;
        }
    }
    chip8.load_program(program);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            chip8.cycle(&args);
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => chip8.on_key_down(key.code() as u16),
                _ => {}
            }
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => chip8.keyboard.on_key_up(key.code() as u16),
                _ => {}
            }
        }
    }

    Ok(())
}
