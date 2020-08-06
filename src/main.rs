use clap::{App, Arg};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

mod cpu;
mod display;
mod ram;

const INPUT_ARG: &str = "INPUT";
const WINDOW_SIZE_ARG: &str = "WINDOW_SIZE";
const COLOR_ARG: &str = "COLOR";

fn main() {
    let matches = get_argument_matches();
    let file_name = matches.value_of(INPUT_ARG).unwrap();
    let dimension = matches.value_of(WINDOW_SIZE_ARG).unwrap_or("640x320");

    let mut data = Vec::<u8>::new();
    File::open(file_name)
        .unwrap()
        .read_to_end(&mut data)
        .expect("File not found!");

    let (width, height) = get_size_from_string(&dimension);
    let (b_width, b_height) = (64, 32);
    let mut buffer: Vec<u32> = vec![0; b_width * b_height];

    let mut cpu = cpu::Cpu::new().load(data);
    let mut display = display::Display::new(b_width, b_height);

    let mut window = Window::new(
        "CHIP8 Emulator",
        width,
        height,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");
    window.limit_update_rate(Some(std::time::Duration::from_millis(40)));

    let mut timer = 0;
    let mut key = 0x0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if timer % 5 == 0 {
            key = get_key_mapping(window.get_keys_pressed(KeyRepeat::No));
        }

        if timer % 1 == 0 {
            cpu.next(&mut display, key);
        }

        if timer % 10 == 0 {
            let screen = display.get_screen();
            for i in 0..b_height * b_width {
                if screen[i] == 0x0 {
                    buffer[i] = from_u8_rgb(212, 150, 44);
                } else {
                    buffer[i] = from_u8_rgb(255, 201, 54);
                }
            }

            window
                .update_with_buffer(&buffer, b_width, b_height)
                .unwrap();
        }
        timer += 1;
    }
}

fn get_argument_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("CHIP-8 Emulator")
        .version("0.1.0")
        .about("A very simple CHIP-8 Emulator")
        .arg(
            Arg::with_name(INPUT_ARG)
                .required(true)
                .value_name("FILE")
                .help("CHIP-8 ROM")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(WINDOW_SIZE_ARG)
                .short("s")
                .long("size")
                .value_name("WIDTHxHEIGHT")
                .help("Set the size of the window (E.g. 1920x1080)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(COLOR_ARG)
                .short("c")
                .long("color")
                .value_name("#XXXXXX")
                .help("Set the color of the emulator (Not working)")
                .takes_value(true),
        )
        .get_matches()
}

fn get_size_from_string(dim: &str) -> (usize, usize) {
    let size: Vec<&str> = dim.split('x').collect();
    (
        size[0].parse::<usize>().unwrap_or(640),
        size[1].parse::<usize>().unwrap_or(320),
    )
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
