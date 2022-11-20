use std::time::Duration;

use gif_parser::{Frame, FramePixels};

struct FrameWithTime {
  pixels: FramePixels,
  t: Duration,
  duration: Duration,
}

pub struct GifTrack {
  frames: Vec<FrameWithTime>,
}

impl GifTrack {
  pub fn new(frames: Vec<Frame>) -> Self {
    let mut frames_with_time = Vec::new();
    let mut t = Duration::from_nanos(0);

    for Frame { delay, pixels } in frames {
      frames_with_time.push(FrameWithTime {
        pixels,
        t,
        duration: delay,
      });

      t += delay;
    }

    Self {
      frames: frames_with_time,
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
}
