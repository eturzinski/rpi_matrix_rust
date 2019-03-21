extern crate rpi_led_matrix;

use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};
use std::collections::HashMap;

fn main() {
    let mut options = setup_options(100);
    let mut matrix = LedMatrix::new(Some(options)).unwrap();
    let mut canvas: LedCanvas = matrix.canvas();
    // let text = "HELLO WORLD!";

    // let map = setup_letters();
	let mut i:u8 = 0;
	canvas.clear();

	for i2 in 1..10 {
		i=i2;
		canvas.clear();
		canvas.draw_circle(10+i, 10, 3,&LedColor { red: 10, green:  i*10 , blue: 100 + i * 10 });
		std::thread::sleep(std::time::Duration::new(1,0))
	}
	loop{};
	}
    

fn setup_options(brightness:u8)->LedMatrixOptions{
    let mut options = LedMatrixOptions::new();
    options.set_rows(16);
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(1);
    options.set_parallel(true);
    options.set_brightness(brightness);
    options
}

fn setup_letters() -> HashMap<char,[[bool; 6]; 5] > {
    let mut map = HashMap::new();
    map.insert ( 'A',[[true,true,true,true,true,true],[true,false,true,false,false,false],[true,false,true,false,false,false],[true,false,true,false,false,false],[true,true,true,true,true,true]]);
    map.insert ( 'B',[[true,true,true,true,true,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[false,true,true,true,true,false]]);
    map.insert ( 'C',[[true,true,true,true,true,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,false,false,false,false,true]]);
    map.insert ( 'D',[[true,true,true,true,true,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[false,true,true,true,true,false]]);
    map.insert ( 'E',[[true,true,true,true,true,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true]]);
    map.insert ( 'F',[[true,true,true,true,true,true],[true,false,true,false,false,false],[true,false,true,false,false,false],[true,false,true,false,false,false],[true,false,true,false,false,false]]);
    map.insert ( 'G',[[true,true,true,true,true,true],[true,false,false,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,false,true,true,true]]);
    map.insert ( 'H',[[true,true,true,true,true,true],[false,false,true,false,false,false],[false,false,true,false,false,false],[false,false,true,false,false,false],[true,true,true,true,true,true]]);
    map.insert ( 'I',[[true,true,true,true,true,true],[false,false,false,false,false,false],[false,false,false,false,false,false],[false,false,false,false,false,false],[false,false,false,false,false,false]]);
    map.insert ( 'J',[[true,false,false,true,true,false],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,true,true,true,true,true]]);
    map.insert ( 'K',[[true,true,true,true,true,true],[false,false,true,false,false,false],[false,true,false,true,false,false],[true,false,false,false,true,false],[false,false,false,false,false,true]]);
    map.insert ( 'L',[[true,true,true,true,true,true],[false,false,false,false,false,true],[false,false,false,false,false,true],[false,false,false,false,false,true],[false,false,false,false,false,true]]);
    map.insert ( 'M',[[true,true,true,true,true,true],[true,false,false,false,false,false],[false,true,true,false,false,false],[true,false,false,false,false,false],[true,true,true,true,true,true]]);
    map.insert ( 'N',[[true,true,true,true,true,true],[false,true,false,false,false,false],[false,false,true,false,false,false],[false,false,false,true,false,false],[true,true,true,true,true,true]]);
    map.insert ( 'O',[[false,true,true,true,true,false],[true,false,false,false,false,true],[true,false,false,false,false,true],[true,false,false,false,false,true],[false,true,true,true,true,false]]);
    map.insert ( 'P',[[true,true,true,true,true,true],[true,false,false,true,false,false],[true,false,false,true,false,false],[true,false,false,true,false,false],[false,true,true,false,false,false]]);
    map.insert ( 'Q',[[false,true,true,true,false,false],[true,false,false,false,true,false],[true,false,false,true,true,false],[true,false,false,false,true,false],[false,true,true,true,false,true]]);
    map.insert ( 'R',[[true,true,true,true,true,true],[true,false,false,true,false,false],[true,false,false,true,false,false],[true,false,false,true,true,false],[false,true,true,false,false,true]]);
    map.insert ( 'S',[[false,true,false,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,false,false,true,true,false]]);
    map.insert ( 'T',[[true,false,false,false,false,false],[true,false,false,false,false,false],[true,true,true,true,true,true],[true,false,false,false,false,false],[true,false,false,false,false,false]]);
    map.insert ( 'U',[[true,true,true,true,true,false],[false,false,false,false,false,true],[false,false,false,false,false,true],[false,false,false,false,false,true],[true,true,true,true,true,false]]);
    map.insert ( 'V',[[true,true,true,true,false,false],[false,false,false,false,true,false],[false,false,false,false,false,true],[false,false,false,false,true,false],[true,true,true,true,false,false]]);
    map.insert ( 'W',[[true,true,true,true,true,true],[false,false,false,false,true,false],[false,false,false,false,false,true],[false,false,false,false,true,false],[true,true,true,true,true,true]]);
    map.insert ( 'X',[[true,false,false,false,false,true],[false,true,false,false,true,false],[false,false,true,true,false,false],[false,true,false,false,true,false],[true,false,false,false,false,true]]);
    map.insert ( 'Y',[[true,false,false,false,false,false],[false,true,false,false,false,false],[false,false,true,true,true,true],[false,true,false,false,false,false],[true,false,false,false,false,false]]);
    map.insert ( 'Z',[[true,false,false,false,true,true],[true,false,false,true,false,true],[true,false,true,false,false,true],[true,false,true,false,false,true],[true,true,false,false,false,true]]);
    map.insert ( ' ',[[false,false,false,false,false,false],[false,false,false,false,false,false],[false,false,false,false,false,false],[false,false,false,false,false,false],[false,false,false,false,false,false]]);
    map.insert ( '!',[[false,false,false,false,false,false],[false,false,false,false,false,false],[true,true,true,true,false,true],[false,false,false,false,false,false],[false,false,false,false,false,false]]);

    map
}

fn print_letter(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>){
    can.clear();
    let letter = map.get(&key).unwrap();
    for i in 0..letter.len() {
        for j in 0..letter[i].len(){
            if letter[i][j] {
                can.set(i as i32,j as i32,&LedColor { red: 10, green: 0, blue: 0 });
            }
        }
    }
}

fn print_letter_offset(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>, offset_x: usize, offset_y: usize){
    //todo
}
