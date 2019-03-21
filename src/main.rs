extern crate rpi_led_matrix;

use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut options = setup_options(100);
    let mut matrix = LedMatrix::new(Some(options)).unwrap();
    let mut canvas: LedCanvas = matrix.canvas();
    let text = "HELLO WORLD!";

    let map = setup_letters();

//    loop {
//        for l in text.chars() {
//            print_letter(l, &mut canvas, &map);
//            std::thread::sleep(std::time::Duration::new(1,0));
//            canvas.clear();
//            std::thread::sleep(std::time::Duration::new(0,100_000_000));
//        }
//    }

    print_text_ticker(text, &mut canvas, &map);
}

fn setup_options(brightness: u8) -> LedMatrixOptions {
    let mut options = LedMatrixOptions::new();
    options.set_rows(16);
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(1);
    options.set_parallel(true);
    options.set_brightness(brightness);
    options
}

fn setup_letters() -> HashMap<char, [[bool; 6]; 5]> {
    let mut map = HashMap::new();
    map.insert('A', [[true, true, true, true, true, true], [true, false, true, false, false, false], [true, false, true, false, false, false], [true, false, true, false, false, false], [true, true, true, true, true, true]]);
    map.insert('B', [[true, true, true, true, true, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [false, true, true, true, true, false]]);
    map.insert('C', [[true, true, true, true, true, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, false, false, false, false, true]]);
    map.insert('D', [[true, true, true, true, true, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [false, true, true, true, true, false]]);
    map.insert('E', [[true, true, true, true, true, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true]]);
    map.insert('F', [[true, true, true, true, true, true], [true, false, true, false, false, false], [true, false, true, false, false, false], [true, false, true, false, false, false], [true, false, true, false, false, false]]);
    map.insert('G', [[true, true, true, true, true, true], [true, false, false, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, false, true, true, true]]);
    map.insert('H', [[true, true, true, true, true, true], [false, false, true, false, false, false], [false, false, true, false, false, false], [false, false, true, false, false, false], [true, true, true, true, true, true]]);
    map.insert('I', [[true, true, true, true, true, true], [false, false, false, false, false, false], [false, false, false, false, false, false], [false, false, false, false, false, false], [false, false, false, false, false, false]]);
    map.insert('J', [[true, false, false, true, true, false], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, true, true, true, true, true]]);
    map.insert('K', [[true, true, true, true, true, true], [false, false, true, false, false, false], [false, true, false, true, false, false], [true, false, false, false, true, false], [false, false, false, false, false, true]]);
    map.insert('L', [[true, true, true, true, true, true], [false, false, false, false, false, true], [false, false, false, false, false, true], [false, false, false, false, false, true], [false, false, false, false, false, true]]);
    map.insert('M', [[true, true, true, true, true, true], [true, false, false, false, false, false], [false, true, true, false, false, false], [true, false, false, false, false, false], [true, true, true, true, true, true]]);
    map.insert('N', [[true, true, true, true, true, true], [false, true, false, false, false, false], [false, false, true, false, false, false], [false, false, false, true, false, false], [true, true, true, true, true, true]]);
    map.insert('O', [[false, true, true, true, true, false], [true, false, false, false, false, true], [true, false, false, false, false, true], [true, false, false, false, false, true], [false, true, true, true, true, false]]);
    map.insert('P', [[true, true, true, true, true, true], [true, false, false, true, false, false], [true, false, false, true, false, false], [true, false, false, true, false, false], [false, true, true, false, false, false]]);
    map.insert('Q', [[false, true, true, true, false, false], [true, false, false, false, true, false], [true, false, false, true, true, false], [true, false, false, false, true, false], [false, true, true, true, false, true]]);
    map.insert('R', [[true, true, true, true, true, true], [true, false, false, true, false, false], [true, false, false, true, false, false], [true, false, false, true, true, false], [false, true, true, false, false, true]]);
    map.insert('S', [[false, true, false, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, false, false, true, true, false]]);
    map.insert('T', [[true, false, false, false, false, false], [true, false, false, false, false, false], [true, true, true, true, true, true], [true, false, false, false, false, false], [true, false, false, false, false, false]]);
    map.insert('U', [[true, true, true, true, true, false], [false, false, false, false, false, true], [false, false, false, false, false, true], [false, false, false, false, false, true], [true, true, true, true, true, false]]);
    map.insert('V', [[true, true, true, true, false, false], [false, false, false, false, true, false], [false, false, false, false, false, true], [false, false, false, false, true, false], [true, true, true, true, false, false]]);
    map.insert('W', [[true, true, true, true, true, true], [false, false, false, false, true, false], [false, false, false, false, false, true], [false, false, false, false, true, false], [true, true, true, true, true, true]]);
    map.insert('X', [[true, false, false, false, false, true], [false, true, false, false, true, false], [false, false, true, true, false, false], [false, true, false, false, true, false], [true, false, false, false, false, true]]);
    map.insert('Y', [[true, false, false, false, false, false], [false, true, false, false, false, false], [false, false, true, true, true, true], [false, true, false, false, false, false], [true, false, false, false, false, false]]);
    map.insert('Z', [[true, false, false, false, true, true], [true, false, false, true, false, true], [true, false, true, false, false, true], [true, false, true, false, false, true], [true, true, false, false, false, true]]);
    map.insert(' ', [[false, false, false, false, false, false], [false, false, false, false, false, false], [false, false, false, false, false, false], [false, false, false, false, false, false], [false, false, false, false, false, false]]);
    map.insert('!', [[false, false, false, false, false, false], [false, false, false, false, false, false], [true, true, true, true, false, true], [false, false, false, false, false, false], [false, false, false, false, false, false]]);

    map
}

fn print_letter(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    print_letter_offset(key, can, map, 0, 0, 0);
}

fn print_letter_offset(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>, offset_x: usize, offset_y: usize, running_offset: usize) {
    let letter = map.get(&key).unwrap();
    let mut rng = rand::thread_rng();
    for i in 0..letter.len() {
        for j in 0..letter[i].len() {
            if letter[i][j] && (i + offset_x) >= running_offset {
                can.set((i + offset_x-running_offset) as i32, (j + offset_y) as i32, &LedColor { red: rng.gen_range(1,255), green: rng.gen_range(1,255), blue: rng.gen_range(1,255) });
            }
        }
    }
}

fn print_text(mut text: &str, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    let letter_size = 6;
    can.clear();
    for i in 0..text.len() {
        print_letter_offset(text[i..].chars().next().unwrap(), can, map, i * letter_size, 0, 0);
    }
}

fn print_text_ticker(mut text: &str, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    let letter_size = 6;
    for off in 0..(letter_size * text.len()) {
        for i in 0..text.len() {
            print_letter_offset(text[i..].chars().next().unwrap(), can, map, (i * letter_size), 0, off);
        }
        std::thread::sleep(std::time::Duration::new(0, 100_000_000));
        can.clear();
    }
}
