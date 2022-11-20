use gif_parser::Pixel;

pub fn compose(
  height: usize,
  width: usize,
  eye: gif_parser::FramePixels,
  mouth: gif_parser::FramePixels,
  nose: gif_parser::FramePixels,
) -> gif_parser::FramePixels {
  let mut pixels =
    gif_parser::FramePixels::new(height, width, vec![Pixel::Transparent; height * width]);

  for y in 0..height {
    for x in 0..width {
      if let Some(Pixel::Colored(r, g, b)) = eye.get_pixel(x, y) {
        pixels.write_pixel(x, y, Pixel::Colored(r, g, b));
      }

      if let Some(Pixel::Colored(r, g, b)) = mouth.get_pixel(x, y) {
        pixels.write_pixel(x, y, Pixel::Colored(r, g, b));
      }

      if let Some(Pixel::Colored(r, g, b)) = nose.get_pixel(x, y) {
        pixels.write_pixel(x, y, Pixel::Colored(r, g, b));
      }
    }
  }

  pixels
}

pub struct Composer {
  eye: Vec<gif_parser::Frame>,
  eye_frame: usize,
  mouth: Vec<gif_parser::Frame>,
  mouth_frame: usize,
  nose: Vec<gif_parser::Frame>,
  nose_frame: usize,
}

impl Composer {
  pub fn new(
    eye: Vec<gif_parser::Frame>,
    mouth: Vec<gif_parser::Frame>,
    nose: Vec<gif_parser::Frame>,
  ) -> Self {
    Self {
      eye,
      eye_frame: 0,
      mouth,
      mouth_frame: 0,
      nose,
      nose_frame: 0,
    }
  }

  pub fn next_frame(&mut self) -> gif_parser::Frame {
    let eye = &self.eye[self.eye_frame];
    let mouth = &self.mouth[self.mouth_frame];
    let nose = &self.nose[self.nose_frame];

    let pixels = compose(
      eye.pixels.height(),
      eye.pixels.width(),
      eye.pixels.clone(),
      mouth.pixels.clone(),
      nose.pixels.clone(),
    );

    let frame = gif_parser::Frame {
      pixels,
      delay: eye.delay,
    };

    self.eye_frame = (self.eye_frame + 1) % self.eye.len();
    self.mouth_frame = (self.mouth_frame + 1) % self.mouth.len();
    self.nose_frame = (self.nose_frame + 1) % self.nose.len();

    frame
  }
}
