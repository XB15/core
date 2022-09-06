use clap::Parser;

use gif_parser::*;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap(short, long, default_value = "test.gif")]
  gif: String,
}

fn main() {
  // have to do this type assert here because proc macros are broken in rust analyzer atm
  // thanks rust :/
  let cli: Args = Args::parse();

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
        pixels[y][x] = format!("\x1B[38;2;{};{};{}m█\x1B[0m", pixel.0, pixel.1, pixel.2);
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
