use crate::display::Display;
use crate::ram::Ram;
use rand::Rng;
use std::time;

const START_ADR: u16 = 0x0200;
const NNN: u16 = 0x0FFF;
const NN: u16 = 0x00FF;
const N: u16 = 0x000F;
const X: u16 = 0x0F00;
const Y: u16 = 0x00F0;

pub struct Cpu {
    pc: u16,
    i: u16,
    regs: [u8; 16],
    ram: Ram,
    stack: Vec<u16>,
    timer: u8,
}

impl Cpu {
    // create new cpu
    pub fn new() -> Cpu {
        Cpu {
            pc: START_ADR,
            i: 0,
            regs: [0; 16],
            ram: Ram::new(),
            stack: Vec::new(),
            timer: 0,
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        let mut adr = START_ADR;
        for byte in data {
            self.ram.write(adr, byte);
            adr += 1;
        }
    }

    // runs the next instruction
    pub fn next(&mut self, display: &mut Display, key: u8) {
        // read the next instruction from memory with pc
        let instr: u16 = self.ram.read_halfword(self.pc as usize);

        let vx = ((instr & X) >> 8) as usize;
        let vy = ((instr & Y) >> 4) as usize;

        match (instr & 0xF000) >> 12 {
            0x0 => match instr & NN {
                0xE0 => {
                    // clear screen
                    display.clear();
                }
                0xEE => {
                    // return from subroutine
                    if let Some(item) = self.stack.pop() {
                        self.pc = item;
                        return;
                    }
                }
                _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instr),
            },
            0x1 => {
                // jump to adr nnn
                self.pc = instr & NNN;
                return;
            }
            0x2 => {
                // call subroutine from adr nnn
                self.stack.push(self.pc + 2);
                self.pc = instr & NNN;
                return;
            }
            0x3 => {
                // if (Vx == nn) skip next instr
                if self.regs[vx] == (instr & NN) as u8 {
                    self.pc += 2;
                }
            }
            0x4 => {
                // if (Vx != nn) skip next instr
                if self.regs[vx] != (instr & NN) as u8 {
                    self.pc += 2;
                }
            }
            0x5 => {
                // if instr & N == 0x0 && self.regs[vx] == self.regs[vy] {
                if self.regs[vx] == self.regs[vy] {
                    // if (Vx == Vy) skip next instr
                    self.pc += 2;
                }
            }
            0x6 => {
                // Vx = NN
                self.regs[vx] = (instr & NN) as u8;
            }
            0x7 => {
                // Vx += NN
                let sum = self.regs[vx] as u16 + (instr & NN);
                self.regs[vx] = sum as u8;
            }
            0x8 => {
                match instr & N {
                    0x0 => {
                        // Vx = Vy
                        self.regs[vx] = self.regs[vy];
                    }
                    0x1 => {
                        // Vx |= Vy
                        self.regs[vx] |= self.regs[vy];
                    }
                    0x2 => {
                        // Vx &= Vy
                        self.regs[vx] &= self.regs[vy];
                    }
                    0x3 => {
                        // Vx ^= Vy
                        self.regs[vx] ^= self.regs[vy];
                    }
                    0x4 => {
                        // Vx += Vy
                        let sum = self.regs[vx] as u16 + self.regs[vy] as u16;
                        self.regs[vx] = sum as u8;
                        if sum > 0xFF {
                            self.regs[0xF] = 1;
                        }
                    }
                    0x5 => {
                        // Vx -= Vy
                        let diff: i8 = self.regs[vx] as i8 - self.regs[vy] as i8;
                        self.regs[vx] = diff as u8;
                        if diff < 0 {
                            self.regs[0xF] = 1;
                        } else {
                            self.regs[0xF] = 0;
                        }
                    }
                    0x6 => {
                        // VF = Vx & 0x1
                        // Vx >>= 1
                        self.regs[0xF] = self.regs[vx] & 0x1;
                        self.regs[vx] >>= 1;
                    }
                    0x7 => {
                        // Vx = Vy - Vx
                        let diff: i8 = self.regs[vy] as i8 - self.regs[vx] as i8;
                        self.regs[vx] = diff as u8;
                        if diff < 0 {
                            self.regs[0xF] = 1;
                        } else {
                            self.regs[0xF] = 0;
                        }
                    }
                    0xE => {
                        // VF = Vx & 0x8
                        // Vx <<= 1
                        self.regs[0xF] = (self.regs[vx] & 0x80) >> 7;
                        self.regs[vx] <<= 1;
                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instr),
                }
            }
            0x9 => {
                // if (Vx == Vy) skip next instr
                // if instr & N == 0x0 && self.regs[vx] != self.regs[vy] {
                if self.regs[vx] != self.regs[vy] {
                    self.pc += 2;
                }
            }
            0xA => {
                // I = NNN
                self.i = instr & NNN;
            }
            0xB => {
                // PC = V0 + NNN
                self.pc = self.regs[0] as u16 + (instr & NNN);
            }
            0xC => {
                // Vx = rand && NN
                let random_number = rand::thread_rng().gen_range(0, 0xFF);
                self.regs[vx] = (random_number & (instr & NN)) as u8;
            }
            0xD => {
                // draw(Vx, Vy, N)
                self.regs[0xF] = 0;
                let mut should_write = false;
                for h in 0..(instr & N) {
                    let byte = self.ram.read_byte((self.i + h) as usize);
                    if display.draw(self.regs[vx], self.regs[vy] + h as u8, byte) {
                        should_write = true;
                    }
                }
                if should_write {
                    self.regs[0xF] = 1;
                } else {
                    self.regs[0xF] = 0;
                }
            }
            0xE => {
                match instr & NN {
                    0x9E => {
                        // if (key() == Vx)
                        if key == self.regs[vx] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // if (key() != Vx)
                        if key != self.regs[vx] {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instr),
                }
            }
            0xF => {
                match instr & NN {
                    0x07 => {
                        // Vx = get_delay()
                        // should update timer at every next() call instead
                        if self.timer > 0 {
                            self.timer -= 1;
                        }
                        self.regs[vx] = self.timer;
                    }
                    0x0A => {
                        // Vx = get_key()
                        if key != 0x0 {
                            self.regs[vx] = key;
                        } else {
                            return;
                        }
                    }
                    0x15 => {
                        // delay_timer(Vx)
                        self.timer = self.regs[vx];
                    }
                    0x18 => {
                        // sound_timer(Vx)
                    }
                    0x1E => {
                        // I += Vx
                        self.i += self.regs[vx] as u16;
                    }
                    0x29 => {
                        // I = sprite_adr[Vx]
                        self.i = self.regs[vx] as u16 * 5;
                    }
                    0x33 => {
                        // set_BCD(Vx)
                        // adr[I+0] = BCD(3)
                        self.ram.write(self.i, self.regs[vx] / 100);
                        // adr[I+1] = BCD(2)
                        self.ram.write(self.i + 1, (self.regs[vx] % 100) / 10);
                        // adr[I+2] = BCD(1)
                        self.ram.write(self.i + 2, self.regs[vx] / 10);
                    }
                    0x55 => {
                        // reg_dump(Vx, &I)
                        for i in 0..vx + 1 {
                            self.ram.write(self.i + i as u16, self.regs[i]);
                        }
                    }
                    0x65 => {
                        // reg_load(Vx, &I)
                        for i in 0..vx + 1 {
                            self.regs[i] = self.ram.read_byte(self.i as usize + i);
                        }
                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instr),
                }
            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instr),
        }

        // increase pc
        self.pc += 2;
    }
}
