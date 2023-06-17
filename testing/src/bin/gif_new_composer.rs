use clap::Parser;

use gif_parser::*;
use new_composer::Composer;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap(long, default_value = "eye.gif")]
  eye: String,
  #[clap(long, default_value = "mouth.gif")]
  mouth: String,
  #[clap(long, default_value = "nose.gif")]
  nose: String,
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
      Box::new(new_composer::tracks::GifTrack::new(eye_frames)),
      Box::new(new_composer::tracks::GifTrack::new(mouth_frames)),
      Box::new(new_composer::tracks::GifTrack::new(nose_frames)),
    ],
  );

  loop {
    let frame = composer.next_frame();

    let width = frame.pixels.width();
    let height = frame.pixels.height();
    let mut pixels = vec![vec![String::default(); width]; height];

    for (i, pixel) in frame.pixels.pixels().iter().enumerate() {
      let x = i % width;
      let y = i / width;

      // Format as 24 bit color escape sequences
      pixels[y][x] = match pixel {
        Pixel(r, g, b, true) => format!("\x1B[38;2;{};{};{}m█\x1B[0m", r, g, b),
        Pixel(_, _, _, false) => "\x1b[38;2;0;0;0m \x1B[0m".to_string(),
      };
    }

    let pixels = pixels
      .iter()
      .map(|row| row.join(""))
      .collect::<Vec<String>>()
      .join("\n");

    // Clear the screen
    print!("\x1B[2J");

    // Print the pixels
    println!("{}", pixels);

    std::thread::sleep(frame.delay);
  }
}
