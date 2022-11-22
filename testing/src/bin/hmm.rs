use new_composer::{Composer, Track};

fn main() {
  let eyes = Track::with_idle_animation(Gif::new(Vec::new()))
    .add_animation(
      "blink".to_string(),
      Gif::new(Vec::new()),
      Gif::new(Vec::new()),
    )
    .add_animation("heart", Gif::new(Vec::new()), Gif::new(Vec::new()))
    .build();

  let nose = Track::with_idle_animation(Gif::new(Vec::new())).build();

  let mouth = Track::with_idle_animation(Gif::new(Vec::new()))
    .add_animation("smug", Gif::new(Vec::new()), Gif::new(Vec::new()))
    .build();

  let face_features = Track::with_idle_animation(Gif::new(Vec::new()))
    .add_animation("blush", Gif::new(Vec::new()), Gif::new(Vec::new()))
    .build();

  let composer = Composer::builder()
    .add_track("eyes", eyes)
    .add_track("nose", nose)
    .add_track("mouth", mouth)
    .add_track("face_features", face_features)
    .build();
}
