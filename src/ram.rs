#[derive(Copy, Clone)]
pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    // create memory
    pub fn new() -> Ram {
        let mut ram = Ram { mem: [0; 4096] };

        let font: [u8; 16 * 5] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        for i in 0..16 * 5 {
            ram.mem[i] = font[i];
        }

        return ram;
    }

    // read byte from memory
    pub fn read_byte(self, adr: usize) -> u8 {
        self.mem[adr]
    }

    // read halfword from memory
    pub fn read_halfword(self, adr: usize) -> u16 {
        (self.mem[adr] as u16) << 8 | self.mem[adr + 1] as u16
    }

    // write to memory
    pub fn write(&mut self, adr: u16, data: u8) {
        self.mem[adr as usize] = data;
    }

    pub fn print(self, start: usize, length: usize) {
        let mut i = start;
        println!("");
        for item in &self.mem[start..(start + length)] {
            print!("[index: {}, data: {:b}], ", i, item);
            i += 1;
        }
        println!("");
    }
}
