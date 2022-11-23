mod builder;
mod track;

pub use track::{Gif, Track};

use builder::Builder;
use gif_parser::{FramePixels, Pixel, TRANSPARENT};
use std::{collections::HashMap, time::Duration};

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
  tracks: HashMap<String, Track>,
}

impl Composer {
  pub fn builder() -> Builder {
    Builder {
      tracks: HashMap::new(),
    }
  }

  fn new(width: usize, height: usize, tracks: HashMap<String, Track>) -> Self {
    Self {
      height,
      width,
      tracks,
    }
  }

  pub fn get_pixels_at(&self, t: Duration) -> FramePixels {
    let pixels = compose(
      self.height,
      self.width,
      self
        .tracks
        .iter()
        .map(|(_, track)| track.get_pixels_at(t))
        .collect(),
    );

    pixels
  }

  pub fn transition_track_to(&mut self, track: impl ToString, animation: impl ToString) {
    // TODO: Error handling
    self
      .tracks
      .get_mut(&track.to_string())
      .unwrap()
      .transition_to(animation)
      .unwrap();
  }
}
