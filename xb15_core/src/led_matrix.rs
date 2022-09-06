use napi::bindgen_prelude::*;
use rpi_led_matrix::{LedColor, LedMatrixOptions, LedRuntimeOptions};

#[napi]
struct LedMatrix {
  matrix: rpi_led_matrix::LedMatrix,
}

#[napi]
impl LedMatrix {
  #[napi(constructor)]
  pub fn new(pi_version: i32, cols: i32, rows: i32, rgb_sequence: Option<String>) -> Result<Self> {
    if pi_version < 3 || pi_version > 4 {
      return Err(Error::new(
        Status::InvalidArg,
        "Unsupported Raspberry Pi version".to_string(),
      ));
    }

    if cols < 1 || rows < 1 {
      return Err(Error::new(
        Status::InvalidArg,
        "Invalid matrix dimensions".to_string(),
      ));
    }

    let rgb_sequence = rgb_sequence.unwrap_or_else(|| "RGB".to_string());

    let mut options = LedMatrixOptions::new();
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(2);
    // options.set_hardware_pulsing(true);
    options.set_refresh_rate(false);
    options.set_brightness(100).unwrap();
    options.set_cols(cols as u32);
    options.set_rows(rows as u32);

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(if pi_version == 3 { 1 } else { 2 });

    match rpi_led_matrix::LedMatrix::new(Some(options), Some(rt_options)) {
      Ok(matrix) => Ok(Self { matrix }),
      Err(err) => Err(Error::new(Status::GenericFailure, err.to_string())),
    }
  }

  #[napi]
  pub fn fill(&mut self, r: u8, g: u8, b: u8) {
    self.matrix.canvas().fill(&LedColor {
      red: r,
      green: g,
      blue: b,
    });
  }

  #[napi]
  pub fn render(&mut self) {
    let _ = self.matrix.swap(self.matrix.canvas());
  }
}
