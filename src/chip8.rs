use super::keyboard::Keyboard;
use super::monitor::Monitor;

use opengl_graphics::OpenGL;
use piston::input::RenderArgs;
use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; NUM_REGISTERS],
    pub index: usize,
    pub pc: usize,
    pub stack: Vec<usize>,
    pub sp: usize,
    pub delay_timer: u8,
    pub monitor: Monitor,
    pub paused: bool,
    pub speed: u8,
    pub keyboard: Keyboard,
    is_waiting_for_keyboard: bool,
    wait_for_keyboard_value: usize,
}
impl Chip8 {
    pub fn new(opengl: OpenGL) -> Self {
        let monitor = Monitor::new(opengl);
        let mut chip8 = Self {
            memory: [0; MEMORY_SIZE],
            v: [0; NUM_REGISTERS],
            index: 0,
            pc: 0x200,
            stack: vec![],
            sp: 0,
            delay_timer: 0,
            monitor: monitor,
            paused: false,
            speed: 10,
            keyboard: Keyboard::new(),
            is_waiting_for_keyboard: false,
            wait_for_keyboard_value: 0,
        };
        chip8.load_sprites();
        chip8
    }
    pub fn on_key_down(&mut self, code: u16) {
        self.keyboard.on_key_down(code);
        if self.is_waiting_for_keyboard {
            self.v[self.wait_for_keyboard_value] = code as u8;
            self.paused = false;
            self.is_waiting_for_keyboard = false;
        }
    }

    pub fn load_sprites(&mut self) {
        let sprites = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..sprites.len() {
            self.memory[i] = sprites[i];
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for i in 0..program.len() {
            self.memory[0x200 + i] = program[i];
        }
    }

    pub fn update_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        //if self.sound_timer > 0 {
        //self.sound_timer -= 1;
        //}
    }

    pub fn cycle(&mut self, args: &RenderArgs) {
        for _i in 0..self.speed {
            if !self.paused {
                let opcode = ((self.memory[self.pc as usize] as u16) << 8) as u16
                    | self.memory[(self.pc + 1) as usize] as u16;
                self.interpret_instruction(opcode as usize);
            }
        }

        if !self.paused {
            self.update_timer();
        }
        self.monitor.paint(args);
    }

    pub fn interpret_instruction(&mut self, instruction: usize) {
        //println!("doing {:02x}", instruction);
        self.pc += 2;
        let x = ((instruction & 0x0f00) >> 8) as usize;
        let y = ((instruction & 0x00f0) >> 4) as usize;
        let kk = (instruction & 0xFF) as u8;
        let opcode: usize = instruction & 0xf000;

        match opcode {
            0x0000 => match instruction {
                0x00E0 => {
                    self.monitor.clear();
                }
                0x0EE => {
                    self.pc = self.stack.pop().unwrap() as usize;
                    if self.sp > 0 {
                        self.sp -= 1;
                    }
                }
                _ => {
                    println!("BAD OPCODE!");
                }
            },
            0x1000 => {
                self.pc = instruction & 0xFFF;
            }
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = instruction & 0xFFF;
            }
            0x3000 => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.v[x] = kk;
            }

            0x7000 => {
                self.v[x] = (self.v[x] as u16 + kk as u16) as u8;
            }
            0x8000 => match instruction & 0xf {
                0x0 => {
                    self.v[x] = self.v[y];
                }
                0x1 => {
                    self.v[x] |= self.v[y];
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                }
                0x4 => {
                    let sum: u16 = (self.v[x] as u16) + (self.v[y] as u16);
                    self.v[x] = sum as u8;
                    self.v[0x0f] = if sum > 0xFF { 1 } else { 0 };
                }
                0x5 => {
                    self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                }
                0x6 => {
                    self.v[0xf] = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                }
                0x7 => {
                    self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                }
                0xe => {
                    self.v[0xf] = (self.v[x] & 0x80) >> 7;
                    self.v[x] <<= 1;
                }
                _ => {
                    println!("BAD OPCODE!");
                }
            },
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.index = instruction & 0xfff;
            }
            0xB000 => {
                self.pc = (instruction & 0xFFF) + self.v[0] as usize;
            }
            0xC000 => {
                let r = rand::thread_rng().gen::<u8>();
                self.v[x] = r & kk;
            }
            0xD000 => {
                let width: u16 = 8;
                let height: u16 = (instruction & 0xF) as u16;

                self.v[0xF] = 0;

                for row in 0..height {
                    let mut sprite = self.memory[self.index + row as usize];

                    for col in 0..width {
                        if (sprite & 0x80) > 0 {
                            if self
                                .monitor
                                .set_pixel((self.v[x] as u16) + col, (self.v[y] as u16) + row)
                            {
                                self.v[0xF] = 1;
                            }
                        }
                        sprite <<= 1;
                    }
                }
            }
            0xE000 => match instruction & 0xff {
                0x9e => {
                    if self.keyboard.is_key_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }
                0xa1 => {
                    if !self.keyboard.is_key_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }
                _ => println!("BAD OPCODE"),
            },
            0xF000 => match instruction & 0xff {
                0x07 => {
                    self.v[x] = self.delay_timer;
                }
                0x0A => {
                    self.paused = true;
                    self.is_waiting_for_keyboard = true;
                    self.wait_for_keyboard_value = x;
                }
                0x15 => {
                    self.delay_timer = self.v[x];
                }
                0x18 => {
                    // sound
                }
                0x1E => {
                    self.index += self.v[x] as usize;
                    self.v[0x0f] = if self.index > 0x0F00 { 1 } else { 0 };
                }
                0x29 => {
                    self.index = (self.v[x] * 5) as usize;
                }
                0x33 => {
                    self.memory[self.index] = self.v[x] / 100;
                    self.memory[self.index + 1] = (self.v[x] % 100) / 10;
                    self.memory[self.index + 2] = self.v[x] % 10;
                }
                0x55 => {
                    for i in 0..x + 1 {
                        self.memory[self.index + i] = self.v[i];
                    }
                }
                0x65 => {
                    for i in 0..x + 1 {
                        self.v[i] = self.memory[self.index + i];
                    }
                }
                _ => {
                    println!("BAD OPCODE!");
                }
            },
            _ => {
                println!("BAD OPCODE!");
            }
        };
    }
}
