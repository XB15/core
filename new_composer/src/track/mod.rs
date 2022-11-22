mod builder;
mod gif;

pub use gif::Gif;
use thiserror::Error;

use gif_parser::{FramePixels, TRANSPARENT};
use std::{collections::HashMap, time::Duration};

use self::builder::{Animation, Builder};

#[derive(Debug, Error)]
pub enum Error {
  #[error("animation not found")]
  AnimationNotFound(String),
}

pub struct Track {
  idle: Gif,
  animations: HashMap<String, Animation>,

  current_animation: Option<String>,

  width: usize,
  height: usize,
}

impl Track {
  pub fn with_idle_animation(animation: Gif) -> Builder {
    Builder {
      idle: animation,
      animations: HashMap::new(),
    }
  }

  fn new(idle: Gif, animations: HashMap<String, Animation>) -> Self {
    let width = idle.width().min(
      animations
        .values()
        .map(|a| a.via.width().min(a.to.width()))
        .min()
        .unwrap_or(idle.width()),
    );

    let height = idle.height().min(
      animations
        .values()
        .map(|a| a.via.height().min(a.to.height()))
        .min()
        .unwrap_or(idle.height()),
    );

    Self {
      idle,
      animations,

      current_animation: None,

      width,
      height,
    }
  }

  pub fn transition_to(&mut self, name: impl ToString) -> Result<(), Error> {
    let name = name.to_string();

    let animation = self
      .animations
      .get_mut(&name)
      .ok_or(Error::AnimationNotFound(name))?;

    animation.via.enable();

    Ok(())
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  /// Returns the pixels for the frame at the given time `t`.
  pub fn get_pixels_at(&mut self, t: Duration) -> FramePixels {
    FramePixels::new(
      self.height,
      self.width,
      vec![TRANSPARENT; self.height * self.width],
    )
  }
}
