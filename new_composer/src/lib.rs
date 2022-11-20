pub mod tracks;

use std::time::Duration;

use gif_parser::Pixel;
use tracks::Track;

fn compose(height: usize, width: usize, frames: Vec<gif_parser::Frame>) -> gif_parser::FramePixels {
  let mut pixels =
    gif_parser::FramePixels::new(height, width, vec![Pixel(0, 0, 0, false); height * width]);

  for y in 0..height {
    for x in 0..width {
      let non_transparent_pixels = frames
        .iter()
        .map(|frame| {
          frame
            .pixels
            .get_pixel(x, y)
            .unwrap_or(gif_parser::TRANSPARENT)
        })
        .filter(|pixel| pixel.3)
        .collect::<Vec<_>>();

      pixels.write_pixel(
        x,
        y,
        if non_transparent_pixels.len() > 0 {
          let (r, g, b) = non_transparent_pixels
            .iter()
            .fold((0, 0, 0), |(r, g, b), pixel| {
              (r + pixel.0, g + pixel.1, b + pixel.2)
            });

          gif_parser::Pixel(
            r / non_transparent_pixels.len() as u8,
            g / non_transparent_pixels.len() as u8,
            b / non_transparent_pixels.len() as u8,
            true,
          )
        } else {
          gif_parser::TRANSPARENT
        },
      );
    }
  }

  pixels
}

pub struct Composer {
  height: usize,
  width: usize,
  tracks: Vec<Box<dyn Track>>,
  min_delay: Duration,
}

impl Composer {
  pub fn new(width: usize, height: usize, tracks: Vec<Box<dyn Track>>) -> Self {
    // TODO: Figure out the smallest common divisor of all the tracks' min_delay
    let min_delay = tracks.iter().map(|track| track.min_delay()).min().unwrap();

    Self {
      height,
      width,
      tracks,
      min_delay,
    }
  }

  pub fn next_frame(&mut self) -> gif_parser::Frame {
    let pixels = compose(
      self.height,
      self.width,
      self.tracks.iter_mut().map(|gif| gif.next_frame()).collect(),
    );

    gif_parser::Frame {
      pixels,
      delay: self.min_delay,
    }
  }
}
