use std::time::Duration;

use gif_parser::Frame;

pub trait Track {
  fn next_frame(&mut self) -> Frame;
  fn min_delay(&self) -> Duration;
}

pub struct GifTrack {
  frames: Vec<Frame>,
  frame_index: usize,
}

impl GifTrack {
  pub fn new(frames: Vec<Frame>) -> Self {
    Self {
      frames,
      frame_index: 0,
    }
  }
}

impl Track for GifTrack {
  fn next_frame(&mut self) -> Frame {
    let frame = self.frames[self.frame_index].clone();
    self.frame_index = (self.frame_index + 1) % self.frames.len();

    frame
  }

  fn min_delay(&self) -> Duration {
    self.frames.iter().map(|frame| frame.delay).min().unwrap()
  }
}
