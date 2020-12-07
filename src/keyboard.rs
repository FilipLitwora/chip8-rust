use std::collections::HashMap;

pub struct Keyboard {
    keymap: HashMap<u16, u8>,
    keys_pressed: Vec<bool>,
}
impl Keyboard {
    pub fn new() -> Self {
        let map: HashMap<u16, u8> = [
            (49, 0x1),  // 1 1
            (50, 0x2),  // 2 2
            (51, 0x3),  // 3 3
            (52, 0xc),  // 4 C
            (113, 0x4), // Q 4
            (119, 0x5), // W 5
            (101, 0x6), // E 6
            (114, 0xD), // R D
            (97, 0x7),  // A 7
            (115, 0x8), // S 8
            (100, 0x9), // D 9
            (102, 0xE), // F E
            (422, 0xA), // Z A
            (99, 0x0),  // X 0
            (118, 0xB), // C B
            (118, 0xF), // V F
        ]
        .iter()
        .cloned()
        .collect();

        Keyboard {
            keymap: map,
            keys_pressed: vec![false; 16],
        }
    }
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keys_pressed[key_code as usize]
    }

    pub fn on_key_down(&mut self, which: u16) {
        if self.keymap.contains_key(&which) {
            let key = self.keymap[&which];
            self.keys_pressed[key as usize] = true;
        }

        // Make sure onNextKeyPress is initialized and the pressed key is actually mapped to a Chip-8 key
        //if (self.onNextKeyPress !== null && key) {
        //self.onNextKeyPress(parseInt(key));
        //self.onNextKeyPress = null;
        //}
    }

    pub fn on_key_up(&mut self, which: u16) {
        if self.keymap.contains_key(&which) {
            let key = self.keymap[&which];
            self.keys_pressed[key as usize] = false;
        }
    }
}
