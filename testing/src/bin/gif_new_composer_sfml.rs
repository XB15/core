use clap::Parser;

use gif_parser::*;
use new_composer::Composer;
use sfml::graphics::{RenderTarget, RenderWindow, Sprite, Texture};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap(long, default_value = "eye.gif")]
  eye: String,
  #[clap(long, default_value = "mouth.gif")]
  mouth: String,
  #[clap(long, default_value = "nose.gif")]
  nose: String,
  #[clap(long, short = 'm', default_value = "8")]
  magnification: u8,
}

fn main() {
  let cli: Args = Args::parse();

  let eye_frames = parse_gif(cli.eye.as_str()).unwrap();
  let mouth_frames = parse_gif(cli.mouth.as_str()).unwrap();
  let nose_frames = parse_gif(cli.nose.as_str()).unwrap();

  let mut composer = Composer::new(
    64,
    32,
    vec![
      new_composer::tracks::GifTrack::new(eye_frames),
      new_composer::tracks::GifTrack::new(mouth_frames),
      new_composer::tracks::GifTrack::new(nose_frames),
    ],
  );

  let mut window = RenderWindow::new(
    (64 * cli.magnification as u32, 32 * cli.magnification as u32),
    "New Composer Demo",
    sfml::window::Style::CLOSE,
    &Default::default(),
  );
  window.set_vertical_sync_enabled(true);
  window.set_active(true);

  let mut tex = Texture::new().unwrap();
  tex.set_srgb(true);
  if !tex.create(64 * cli.magnification as u32, 32 * cli.magnification as u32) {
    panic!("Failed to create texture");
  }

  let start = std::time::Instant::now();
  loop {
    while let Some(event) = window.poll_event() {
      match event {
        sfml::window::Event::Closed => return,
        _ => {}
      }
    }

    let pixels = composer.get_pixels_at(std::time::Instant::now() - start);

    let width = pixels.width();
    let height = pixels.height();
    let scaled_width = width * cli.magnification as usize;
    let scaled_height = height * cli.magnification as usize;
    let mut framebuffer = vec![0u8; (scaled_width * scaled_height * 4) as usize];

    for (i, pixel) in pixels.pixels().iter().enumerate() {
      let x = i % width;
      let y = i / width;

      let color = match pixel {
        Pixel(r, g, b, true) => [*r, *g, *b, 255],
        Pixel(_, _, _, false) => [0, 0, 0, 0],
      };

      for x_offset in 0..cli.magnification {
        for y_offset in 0..cli.magnification {
          let scaled_x = (x * cli.magnification as usize) + x_offset as usize;
          let scaled_y = (y * cli.magnification as usize) + y_offset as usize;
          let scaled_i = (scaled_y * scaled_width + scaled_x) * 4;

          framebuffer[scaled_i] = color[0];
          framebuffer[scaled_i + 1] = color[1];
          framebuffer[scaled_i + 2] = color[2];
          framebuffer[scaled_i + 3] = color[3];
        }
      }
    }

    unsafe {
      tex.update_from_pixels(
        &framebuffer,
        scaled_width as u32,
        scaled_height as u32,
        0,
        0,
      );
    }

    let sprite = Sprite::with_texture(&tex);
    window.clear(sfml::graphics::Color::BLACK);
    window.draw(&sprite);
    window.display();
  }
}
