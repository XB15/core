use std::thread::sleep;
use std::time::Duration;

use rpi_led_matrix::{LedColor, LedMatrixOptions, LedRuntimeOptions};

fn main() -> ! {
  let mut options = LedMatrixOptions::new();
  options.set_hardware_mapping("adafruit-hat-pwm");
  options.set_chain_length(2);
  options.set_cols(64);
  options.set_rows(32);
  options.set_refresh_rate(true);
  options.set_brightness(100).unwrap();

  options.set_hardware_pulsing(true);
  options.set_pwm_bits(11).unwrap();
  options.set_pwm_lsb_nanoseconds(200);

  let mut rt_options = LedRuntimeOptions::new();
  // TODO: expose in constructor
  rt_options.set_gpio_slowdown(2);

  let matrix = rpi_led_matrix::LedMatrix::new(Some(options), Some(rt_options)).unwrap();

  let mut canvas = matrix.offscreen_canvas();

  let mut continuuum = 0u16;
  let mut color = LedColor {
    red: 0,
    green: 0,
    blue: 0,
  };

  loop {
    sleep(Duration::from_millis(5));

    continuuum += 1;
    continuuum %= 3 * 255;

    if continuuum <= 255 {
      let c = continuuum;
      color.blue = (255 - c) as u8;
      color.red = c as u8;
    } else if continuuum > 255 && continuuum <= 511 {
      let c = continuuum - 256;
      color.red = (255 - c) as u8;
      color.green = c as u8;
    } else {
      let c = continuuum - 512;
      color.green = (255 - c) as u8;
      color.blue = c as u8;
    }

    canvas.fill(&color);

    canvas = matrix.swap(canvas);
  }
}
