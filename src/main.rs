#![feature(proc_macro_hygiene, decl_macro)]

extern crate rand;
#[macro_use]
extern crate rocket;
extern crate rpi_led_matrix;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use rocket::{Request, State};
use rocket::http::RawStr;
use rocket::request::Form;
use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};

use font::Font;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Sender, channel};
use std::thread;

mod font;

struct PrintData {
    text:String,
    speed:usize
}


#[options("/<txt>/<sleep>")]
fn index(txt: String, sleep: usize,sender: State<Mutex<Sender<PrintData>>>) -> &'static str {

    sender.lock().unwrap().send(PrintData{text:txt,speed:sleep});
    "hi"
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Du dumme Sau, so geht das nicht!")
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

fn main() {

    let(tx,rx)=channel::<PrintData>();

    thread::spawn(move ||{
        let mut options = setup_options(100);
        let mut matrix = LedMatrix::new(Some(options)).unwrap();
        let mut canvas: LedCanvas = matrix.canvas();
        let map = Font::from_file("font.json").letters;
        for received in rx {
            print_text_ticker(received.text.as_str(),&mut canvas,&map,received.speed);
        }
    });

    rocket::ignite()
        .manage(Mutex::new(tx.clone()))
        .register(catchers![not_found])
        .mount("/", routes![index])
        .launch();
}


fn _print_letter(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>) {
    _print_letter_offset(key, can, map, 0, 0, 0);
}

fn _print_letter_offset(key: char, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>, offset_x: usize, offset_y: usize, running_offset: usize) {
    let letter = map.get(&key).unwrap();
    let mut rng = rand::thread_rng();
    for i in 0..letter.len() {
        for j in 0..letter[i].len() {
            if letter[i][j] && (i + offset_x) >= running_offset {
                can.set((i + offset_x - running_offset) as i32, (j + offset_y) as i32, &LedColor { red: rng.gen_range(1, 255), green: rng.gen_range(1, 255), blue: rng.gen_range(1, 255) });
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

fn print_text_ticker(text: &str, can: &mut LedCanvas, map: &HashMap<char, [[bool; 6]; 5]>, sleep: usize) {
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
