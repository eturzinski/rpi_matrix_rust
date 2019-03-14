extern crate rpi_led_matrix;

use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};
use std::collections::HashMap;

fn main() {
    let mut options = setup_options(100);
    let mut matrix = LedMatrix::new(Some(options)).unwrap();
    let mut canvas: LedCanvas = matrix.canvas();

    let map = setup_letters();
    printLetter('A',&mut canvas,&map);

    std::thread::sleep(std::time::Duration::new(5,0));

    printLetter('B',&mut canvas,&map);
    loop {}
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
    map
}

fn printLetter(key: char, can: &mut LedCanvas,map: &HashMap<char, [[bool; 6]; 5]>){
    can.clear();
    let letter = map.get(&key).unwrap();
    for i in 0..letter.len() {
        for j in 0..letter[i].len(){
            can.set(i as i32,j as i32,&LedColor { red: 10, green: 0, blue: 0 });
        }
    }
}
