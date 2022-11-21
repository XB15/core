pub mod tracks;

use std::time::Duration;

use gif_parser::{FramePixels, Pixel, BLACK, TRANSPARENT};
use tracks::GifTrack;

fn compose(height: usize, width: usize, pixels: Vec<FramePixels>) -> gif_parser::FramePixels {
  let mut new_pixels = FramePixels::new(height, width, vec![TRANSPARENT; height * width]);

  for y in 0..height {
    for x in 0..width {
      let non_transparent_pixels = pixels
        .iter()
        .map(|frame| frame.get_pixel(x, y).unwrap_or(gif_parser::TRANSPARENT))
        .filter(|pixel| pixel.3)
        .collect::<Vec<_>>();

      new_pixels.write_pixel(
        x,
        y,
        if non_transparent_pixels.len() > 0 {
          let (r, g, b) = non_transparent_pixels
            .iter()
            .fold((0, 0, 0), |(r, g, b), pixel| {
              (r + pixel.0, g + pixel.1, b + pixel.2)
            });

          Pixel(
            r / non_transparent_pixels.len() as u8,
            g / non_transparent_pixels.len() as u8,
            b / non_transparent_pixels.len() as u8,
            true,
          )
        } else {
          TRANSPARENT
        },
      );
    }
  }

  new_pixels
}

pub struct Composer {
  height: usize,
  width: usize,
  tracks: Vec<GifTrack>,
  frame: FramePixels,
}

impl Composer {
  pub fn new(width: usize, height: usize, tracks: Vec<GifTrack>) -> Self {
    Self {
      height,
      width,
      tracks,
      frame: FramePixels::new(height, width, vec![BLACK; height * width]),
    }
  }

  pub fn get_pixels_at(&mut self, t: Duration) -> FramePixels {
    let pixels = compose(
      self.height,
      self.width,
      self
        .tracks
        .iter_mut()
        .filter(|t| t.is_enabled())
        .map(|gif| gif.get_pixels_at(t))
        .collect(),
    );

    self.frame = pixels.clone();

    pixels
  }

  pub fn enable_track(&mut self, track: usize) {
    self.tracks[track].enable();
  }

  pub fn disable_track(&mut self, track: usize) {
    self.tracks[track].disable();
  }
}
