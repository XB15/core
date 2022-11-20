use clap::Parser;

use gif_parser::*;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap(short, long, default_value = "test.gif")]
  gif: String,
  #[clap(short, long, default_value = "false")]
  new_composer: bool,
}

fn main() {
  let cli = Args::parse();

  let frames = parse_gif(cli.gif.as_str()).unwrap();

  loop {
    for frame in &frames {
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
}
