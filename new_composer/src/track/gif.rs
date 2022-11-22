use std::time::Duration;

use gif_parser::{Frame, FramePixels};

struct FrameWithTime {
  pixels: FramePixels,
  t: Duration,
  duration: Duration,
}

pub struct Gif {
  frames: Vec<FrameWithTime>,
  enabled: bool,
  width: usize,
  height: usize,
}

impl Gif {
  pub fn new(frames: Vec<Frame>) -> Self {
    let mut frames_with_time = Vec::new();
    let mut t = Duration::from_nanos(0);
    let mut width = 0;
    let mut height = 0;

    for Frame { delay, pixels } in frames {
      width = width.max(pixels.width());
      height = height.max(pixels.height());

      frames_with_time.push(FrameWithTime {
        pixels,
        t,
        duration: delay,
      });

      t += delay;
    }

    Self {
      frames: frames_with_time,
      enabled: true,
      width,
      height,
    }
  }

  pub(crate) fn get_pixels_at(&mut self, mut t: Duration) -> FramePixels {
    if t > self.frames.last().unwrap().t + self.frames.last().unwrap().duration {
      t = Duration::from_nanos(
        <u128 as TryInto<u64>>::try_into(t.as_nanos()).unwrap()
          % <u128 as TryInto<u64>>::try_into(
            (self.frames.last().unwrap().t + self.frames.last().unwrap().duration).as_nanos(),
          )
          .unwrap(),
      );
    }

    let frame = self
      .frames
      .iter()
      .find(|frame| frame.t <= t && frame.t + frame.duration > t)
      .unwrap();

    frame.pixels.clone()
  }

  pub fn enable(&mut self) {
    self.enabled = true;
  }

  pub fn disable(&mut self) {
    self.enabled = false;
  }

  pub fn is_enabled(&self) -> bool {
    self.enabled
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}
