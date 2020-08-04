pub struct Display {
    width: usize,
    height: usize,
    screen: Vec<u8>,
}

impl Display {
    pub fn new(w: usize, h: usize) -> Display {
        Display {
            width: w,
            height: h,
            screen: vec![0; w * h],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(self.width * self.height) {
            self.screen[i] = 0;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, byte: u8) -> bool {
        // draw
        let mut is_flipped = false;
        let y = y as usize % self.height;
        for b in 0..8 {
            let x = (x + b) as usize % self.width;
            let index = y * self.width + x;
            let bit = (byte >> (7 - b)) & 0x1;
            let prev = self.screen[index];
            self.screen[index] ^= bit;

            if prev == 1 && self.screen[index] == 0 {
                is_flipped = true;
            }
        }

        return is_flipped;
    }

    pub fn get_screen(&self) -> &Vec<u8> {
        &self.screen
    }
}
