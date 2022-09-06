use std::borrow::Borrow;

use gif_dispose::RGBA8;
use imgref::ImgVec;

pub struct Pixel(pub u8, pub u8, pub u8);

pub struct FramePixels {
  height: usize,
  width: usize,
  data: Vec<Pixel>,
}

impl FramePixels {
  pub fn new(height: usize, width: usize, data: Vec<Pixel>) -> FramePixels {
    FramePixels {
      height,
      width,
      data,
    }
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn pixels(&self) -> &[Pixel] {
    &self.data
  }
}

impl From<&ImgVec<RGBA8>> for FramePixels {
  fn from(img: &ImgVec<RGBA8>) -> FramePixels {
    let height = img.height();
    let width = img.width();
    let mut data = Vec::new();

    let raw = img.buf();

    for y in 0..height {
      for x in 0..width {
        let offset = (y * width + x) as usize;
        let pixel = raw[offset].borrow();

        let red = pixel.r;
        let green = pixel.g;
        let blue = pixel.b;
        let alpha: &u8 = pixel.a.borrow();

        if *alpha > 0 {
          data.push(Pixel(red, green, blue));
        } else {
          data.push(Pixel(0, 0, 0));
        }
      }
    }

    FramePixels::new(height, width, data)
  }
}

pub struct Frame {
  pub pixels: FramePixels,
  pub delay: std::time::Duration,
}

#[derive(Debug)]
pub enum Error {
  Gif(gif::DecodingError),
  GifDispose(gif_dispose::Error),
}

pub fn parse_gif(path: &str) -> Result<Vec<Frame>, Error> {
  let gif_file = std::fs::File::open(path).unwrap();

  let mut gif_options = gif::DecodeOptions::new();
  gif_options.set_color_output(gif::ColorOutput::Indexed);

  let decoder = gif_options.read_info(gif_file);
  if let Err(e) = decoder {
    return Err(Error::Gif(e));
  }
  let mut decoder = decoder.unwrap();

  let mut screen = gif_dispose::Screen::new_decoder(&decoder);

  let mut frames: Vec<Frame> = Vec::new();

  while let Some(frame) = {
    let frame = decoder.read_next_frame();
    if let Err(e) = frame {
      return Err(Error::Gif(e));
    }
    frame.unwrap()
  } {
    if let Err(err) = screen.blit_frame(frame) {
      return Err(Error::GifDispose(err));
    }

    frames.push(Frame {
      pixels: screen.pixels.borrow().into(),
      delay: std::time::Duration::from_millis((frame.delay * 10) as u64),
    });
  }

  Ok(frames)
}
