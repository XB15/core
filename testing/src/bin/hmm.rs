use std::{
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

use new_composer::{Composer, Gif, Track};

fn parse_gif() -> Gif {
  Gif::new(Vec::new())
}

fn main() {
  let eyes = Track::with_idle_animation(parse_gif())
    .add_animation("blink".to_string(), parse_gif(), parse_gif())
    .add_animation("heart", parse_gif(), parse_gif())
    .build();

  let nose = Track::with_idle_animation(parse_gif()).build();

  let mouth = Track::with_idle_animation(parse_gif())
    .add_animation("smug", parse_gif(), parse_gif())
    .build();

  let face_features = Track::with_idle_animation(parse_gif())
    .add_animation("blush", parse_gif(), parse_gif())
    .build();

  let composer = Arc::new(Mutex::new(
    Composer::builder()
      .add_track("eyes", eyes)
      .add_track("nose", nose)
      .add_track("mouth", mouth)
      .add_track("face_features", face_features)
      .build(),
  ));

  let c1 = composer.clone();

  // Rendering thread
  thread::spawn(move || loop {
    loop {
      c1.lock().unwrap().get_pixels_at(Duration::from_secs(0));
    }
  });

  composer
    .lock()
    .unwrap()
    .transition_track_to("eyes", "blink");

  loop {
    thread::sleep(Duration::from_secs(1));
  }
}
