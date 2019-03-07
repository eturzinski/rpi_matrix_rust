use rpi_led_matrix::{LedMatrixOptions, LedMatrix, LedCanvas, LedColor};

fn main() {
    let mut options = LedMatrixOptions::new();
    options.set_rows(16);
    options.set_hardware_mapping("adafruit-hat");
    let matrix = LedMatrix::new(Some(options)).unwrap();
    let mut canvas:LedCanvas = matrix.canvas();
    canvas.fill(&LedColor{red:10,green:0,blue:0})
}
