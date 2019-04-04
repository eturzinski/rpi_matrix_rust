use rpi_led_matrix::{LedCanvas, LedColor};
use std::collections::HashMap;
use rand::Rng;
use std::time::Duration;

fn _print_letter(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    _print_letter_offset(key, can, map, 0, 0, 0);
}

fn _print_letter_offset(key: char, can: &mut LedCanvas,map: &HashMap<char, [[bool; 6]; 5]>,
                        offset_x: usize, offset_y: usize, running_offset: usize)
{
    let letter = map.get(&key).unwrap();
    let mut rng = rand::thread_rng();
    for i in 0..letter.len() {
        for j in 0..letter[i].len() {
            if letter[i][j] && (i + offset_x) >= running_offset {
                can.set((i + offset_x - running_offset) as i32,
                        (j + offset_y) as i32,
                        &LedColor { red: rng.gen_range(1, 255),
                            green: rng.gen_range(1, 255),
                            blue: rng.gen_range(1, 255) });
            }
        }
    }
}

fn _print_text(text: &str, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    let letter_size = 6;
    can.clear();
    for i in 0..text.len() {
        _print_letter_offset(text[i..].chars().next().unwrap(), can, map, i * letter_size, 0, 0);
    }
}

pub fn print_text_ticker(text: &str, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>, sleep: usize) {
    let text = text.to_uppercase();
    let dur = sleep * 1_000_000;
    let letter_size = 6;
    for off in 0..(letter_size * text.len()) {
        for i in 0..text.len() {
            _print_letter_offset(text[i..].chars().next().unwrap(), can, map, (i * letter_size), 0, off);
        }
        std::thread::sleep(Duration::new(0, dur as u32));
        can.clear();
    }
}