extern crate rpi_led_matrix;

use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions};

fn main() {
    let mut options = LedMatrixOptions::new();
    options.set_rows(16);
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(1);
    options.set_parallel(true);
    options.set_brightness(50);
    let matrix = LedMatrix::new(Some(options)).unwrap();
    let mut canvas: LedCanvas = matrix.canvas();
    canvas.fill(&LedColor { red: 10, green: 0, blue: 0 });
    loop {}
}
