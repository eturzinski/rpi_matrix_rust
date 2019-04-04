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
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use rocket::{Request, State};
use rocket::http::RawStr;
use rocket::request::Form;
use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};

use font::Font;

mod font;
mod print;

struct PrintData {
    text: String,
    speed: usize,
}

fn main() {
    let (tx, rx) = channel::<PrintData>();

    thread::spawn(move || {
        let mut options = setup_options(100);
        let mut matrix = LedMatrix::new(Some(options)).unwrap();
        let mut canvas: LedCanvas = matrix.canvas();
        let map = Font::from_file("font.json").letters;
        for received in rx {
            print::print_text_ticker(received.text.as_str(), &mut canvas, &map, received.speed);
        }
    });

    rocket::ignite()
        .manage(Mutex::new(tx.clone()))
        .register(catchers![not_found])
        .mount("/", routes![index])
        .launch();
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

#[options("/<txt>/<sleep>")]
fn index(txt: String, sleep: usize, sender: State<Mutex<Sender<PrintData>>>) -> &'static str {
    sender.lock().unwrap().send(PrintData { text: txt, speed: sleep });
    "hi"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Du dumme Sau, so geht das nicht!")
}
