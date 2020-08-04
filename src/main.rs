use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;

mod cpu;
mod display;
mod ram;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = "data/PONG";
    let mut data = Vec::<u8>::new();
    File::open(file_name)
        .unwrap()
        .read_to_end(&mut data)
        .expect("File not found!");

    let width: usize = 640;
    let height: usize = 320;
    let b_width: usize = 64;
    let b_height: usize = 32;
    let mut buffer: Vec<u32> = vec![0; b_width * b_height];

    let mut cpu = cpu::Cpu::new();
    let mut display = display::Display::new(b_width, b_height);
    cpu.load(data);

    let mut window = match Window::new("CHIP8 Emulator", width, height, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    window.limit_update_rate(Some(std::time::Duration::from_millis(40)));

    let mut timer = 0;
    let mut is_running = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // window.get_keys_pressed(KeyRepeat::No).map(|keys| {
        //     for t in keys {
        //         match t {
        //             Key::Space => is_running = true,
        //             _ => (),
        //         }
        //     }
        // });

        if timer % 1 == 0 && is_running {
            let key = get_key_mapping(window.get_keys_pressed(KeyRepeat::No));
            cpu.next(&mut display, key);
            // is_running = false;
        }

        if timer % 10 == 0 {
            let screen = display.get_screen();
            for i in 0..b_height * b_width {
                if screen[i] == 0x0 {
                    buffer[i] = from_u8_rgb(0, 0, 0);
                } else {
                    buffer[i] = from_u8_rgb(255, 255, 255);
                }
            }

            window
                .update_with_buffer(&buffer, b_width, b_height)
                .unwrap();
        }
        timer += 1;
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn get_key_mapping(keys: Option<Vec<Key>>) -> u8 {
    if let Some(keys) = keys {
        if !keys.is_empty() {
            match keys[0] {
                Key::Key1 => 0x1,
                Key::Key2 => 0x2,
                Key::Key3 => 0x3,
                Key::Key4 => 0xC,
                Key::Q => 0x4,
                Key::W => 0x5,
                Key::E => 0x6,
                Key::R => 0xD,
                Key::A => 0x7,
                Key::S => 0x8,
                Key::D => 0x9,
                Key::F => 0xE,
                Key::Z => 0xA,
                Key::X => 0x0,
                Key::C => 0xB,
                Key::V => 0xF,
                _ => 0x0,
            }
        } else {
            0x0
        }
    } else {
        0x0
    }
}
