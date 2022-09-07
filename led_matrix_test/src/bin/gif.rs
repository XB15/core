use std::thread::sleep;

use clap::Parser;
use rpi_led_matrix::{LedColor, LedMatrixOptions, LedRuntimeOptions};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, default_value = "test.gif")]
  gif: String,
  #[clap(long, takes_value = false)]
  pwm: bool,
  #[clap(long, default_value = "3")]
  pi_version: u8,
  #[clap(short, long, default_value = "64")]
  cols: u32,
  #[clap(short, long, default_value = "32")]
  rows: u32,
  #[clap(short = 'l', long, default_value = "1")]
  chain_length: u32,
  #[clap(short = 's', long, default_value = "RGB")]
  rgb_sequence: String,
}

fn main() -> ! {
  // have to do this type assert here because proc macros are broken in rust analyzer atm
  // thanks rust :/
  let args: Args = Args::parse();

  let mut options = LedMatrixOptions::new();
  options.set_hardware_pulsing(args.pwm);
  options.set_hardware_mapping(if args.pwm {
    "adafruit-hat-pwm"
  } else {
    "adafruit-hat"
  });

  options.set_cols(args.cols);
  options.set_rows(args.rows);

  options.set_chain_length(args.chain_length);

  options.set_led_rgb_sequence(args.rgb_sequence.as_str());
  options.set_brightness(100).unwrap();

  options.set_pwm_bits(11).unwrap();
  options.set_pwm_lsb_nanoseconds(200);

  options.set_refresh_rate(true);

  let mut rt_options = LedRuntimeOptions::new();
  rt_options.set_gpio_slowdown(if args.pi_version == 4 { 2 } else { 1 });

  let matrix = rpi_led_matrix::LedMatrix::new(Some(options), Some(rt_options)).unwrap();

  let mut canvas = matrix.offscreen_canvas();

  let frames = gif_parser::parse_gif(args.gif.as_str()).unwrap();

  loop {
    for frame in frames.iter() {
      let width = frame.pixels.width();

      for (i, pixel) in frame.pixels.pixels().iter().enumerate() {
        let color = LedColor {
          red: pixel.0,
          green: pixel.1,
          blue: pixel.2,
        };

        let y = (i / width) as i32;

        for display in 0..args.chain_length {
          let x = if display % 2 == 0 {
            ((i % width) + (width * display as usize)) as i32
          } else {
            ((width - (i % width)) + (width * display as usize)) as i32
          };

          canvas.set(x, y, &color);
        }
      }

      canvas = matrix.swap(canvas);

      sleep(frame.delay);
    }
  }
}
