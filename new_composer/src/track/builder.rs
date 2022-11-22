use std::collections::HashMap;

use super::{gif::Gif, Track};

pub struct Builder {
  pub(crate) idle: Gif,
  pub(crate) animations: HashMap<String, Animation>,
}

pub struct Animation {
  pub(crate) via: Gif,
  pub(crate) to: Gif,
}

impl Builder {
  pub fn add_animation(self, name: impl ToString, via: Gif, to: Gif) -> Builder {
    let mut animations = self.animations;
    animations.insert(name.to_string(), Animation { via, to });

    Builder {
      idle: self.idle,
      animations,
    }
  }

  pub fn build(self) -> Track {
    Track::new(self.idle, self.animations)
  }
}
