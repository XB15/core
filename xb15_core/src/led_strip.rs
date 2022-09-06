use napi::bindgen_prelude::*;

#[napi(js_name = "LedStrip")]
struct LedStrip {
  controller: rs_ws281x::Controller,
  leds: i32,
}

#[napi]
impl LedStrip {
  #[napi(factory)]
  pub fn with_ws2811gbr(pin: i32, count: i32, brightness: Option<u8>) -> Result<Self> {
    let brightness = brightness.unwrap_or(255);
    println!("pin {}, count {}, brightness {}", pin, count, brightness);

    let channel = rs_ws281x::ChannelBuilder::new()
      .count(count)
      .pin(pin)
      .brightness(brightness)
      .strip_type(rs_ws281x::StripType::Ws2811Gbr)
      .build();

    match rs_ws281x::ControllerBuilder::new()
      .channel(1, channel)
      .build()
    {
      Ok(controller) => Ok(Self {
        controller,
        leds: count,
      }),
      Err(err) => Err(Error::new(Status::GenericFailure, err.to_string())),
    }
  }

  #[napi]
  pub fn set_color(&mut self, index: i32, r: u8, g: u8, b: u8) -> Result<()> {
    if self.leds <= index || index < 0 {
      return Err(Error::new(
        Status::InvalidArg,
        "index out of bounds".to_string(),
      ));
    }

    let leds = self.controller.leds_mut(0);
    leds[index as usize] = [r, g, b, 0];

    Ok(())
  }

  #[napi]
  pub fn fill(&mut self, r: u8, g: u8, b: u8) -> Result<()> {
    let leds = self.controller.leds_mut(0);

    for led in leds {
      *led = [r, g, b, 0];
    }

    let leds = self.controller.leds_mut(1);

    for led in leds {
      *led = [r, g, b, 0];
    }

    Ok(())
  }

  #[napi]
  pub fn turn_off(&mut self) -> Result<()> {
    self.fill(0, 0, 0)?;
    self.render()
  }

  #[napi]
  pub fn set_brightness(&mut self, brightness: u8) -> Result<()> {
    self.controller.set_brightness(0, brightness);
    self.controller.set_brightness(1, brightness);
    self.render()
  }

  #[napi]
  pub fn render(&mut self) -> Result<()> {
    if self.controller.render().is_ok() {
      Ok(())
    } else {
      Err(Error::new(
        Status::GenericFailure,
        "Failed to render".to_string(),
      ))
    }
  }
}
