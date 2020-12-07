use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;

const COLS: u16 = 64;
const ROWS: u16 = 32;
const SCALE: u16 = 15;

pub struct Monitor {
    pub gl: GlGraphics,
    pub cols: u16,
    pub rows: u16,
    pub display: [u16; COLS as usize * ROWS as usize],
    pub scale: u16,
}

impl Monitor {
    pub fn new(opengl: OpenGL) -> Self {
        Monitor {
            gl: GlGraphics::new(opengl),
            cols: COLS,
            rows: ROWS,
            display: [0; COLS as usize * ROWS as usize],
            scale: SCALE,
        }
    }
    pub fn set_pixel(&mut self, _x: u16, _y: u16) -> bool {
        let mut x = _x;
        let mut y = _y;

        if x > self.cols {
            x -= self.cols;
        }

        if y > self.rows {
            y -= self.rows;
        }
        let mut index = (x + (y * self.cols)) as usize;
        if index >= (self.cols * self.rows) as usize {
            index = (self.cols * self.rows) as usize - 1;
        }

        self.display[index] ^= 1;
        self.display[index] != 1
    }
    pub fn clear(&mut self) {
        for i in 0..self.display.len() {
            self.display[i] = 0;
        }
    }

    pub fn paint(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.scale as f64);
        //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let display = self.display;
        let cols = self.cols;
        let scale = self.scale;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for i in 0..display.len() {
                let x = (i as u16 % cols) * scale;
                let y = (i as u16 / cols) * scale;
                if display[i] == 1 {
                    let transform = c.transform.trans(x as f64, y as f64);

                    rectangle(WHITE, square, transform, gl);
                }
            }
        });
    }
}
