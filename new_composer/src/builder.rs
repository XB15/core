use std::collections::HashMap;

use crate::{track::Track, Composer};

pub struct Builder {
  pub(crate) tracks: HashMap<String, Track>,
}

impl Builder {
  pub fn add_track(self, name: impl ToString, track: Track) -> Builder {
    let mut tracks = self.tracks;
    tracks.insert(name.to_string(), track);

    Builder { tracks }
  }

  pub fn build(self) -> Composer {
    Composer::new(self.width(), self.height(), self.tracks)
  }

  fn width(&self) -> usize {
    self.tracks.values().map(|t| t.width()).max().unwrap()
  }

  fn height(&self) -> usize {
    self.tracks.values().map(|t| t.height()).max().unwrap()
  }
}
