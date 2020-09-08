use image::{
 DynamicImage,
 GenericImageView,
 Pixel
};

use super::error::NishiokaNagatsuError;

pub fn check(source: &DynamicImage) -> Result<(), NishiokaNagatsuError>
{
 const PIXELS_IN_ARRIS: u32 = 256;

 {
  let w = source.width();
  let h = source.height();

  if w != PIXELS_IN_ARRIS || h != PIXELS_IN_ARRIS
  {
   Err(NishiokaNagatsuError::SizeIsNot256x256Pixels {
    w,
    h
   })?
  }
 }

 {
  let c = source.color();
  match c
  {
   image::ColorType::Rgb8 | image::ColorType::Rgba8 => Ok(()),
   _ => Err(NishiokaNagatsuError::ColorTypeIsNotRgb8OrRgba8(c))
  }
 }
}

/// Decode an input image to altitudes Vec with Nishioka-Nagatsu-2015 altitude tile method.
///
/// ## Params
///
/// - source: The souce image data.
/// - resolution_factor: The resolution factor; If you use GSI's dem-png tile then it is 1.0e-2 .
///     - See also: <https://maps.gsi.go.jp/development/demtile.html>
///
/// ## Reference
///
/// - <https://www.jstage.jst.go.jp/article/geoinformatics/26/4/26_155/_article/-char/ja>
///
/// ## Note
///
/// - The specification is defined to width and height eq 256, and channels eq RGB8 orRGBA8, but this function would not check it because Nishioka-Nagatsu-2015 method is not required the specification in essentially.
///     - You can use `check_nishioka_nagatsu_2015` function if you needed.
/// - Output Vec is lineared, its not included a width or a height information. That is 256 maybe.
///     - You can get a `(width:u32, height:u32)` using `.dimension` function of the source if you needed.
///
pub fn decode(source: &DynamicImage, resolution_factor: f64) -> Vec<f64>
{
 let (w, h) = source.dimensions();
 let pixels = w as usize * h as usize;

 let mut out = vec![std::f64::NAN; pixels];

 for (pixel_index, (_, _, p)) in source.pixels().into_iter().enumerate()
 {
  let (r, g, b, _) = p.channels4();

  if r == 128 && g == 0 && b == 0
  {
   // No changes from NaN(initial value)
   continue;
  }

  let tmp = r as i32 * 256 * 256 + g as i32 * 256 + b as i32;
  let altitude_in_meter = resolution_factor * if tmp < (1 << 23) { tmp as f64 } else { (tmp - (1 << 24)) as f64 };
  out[pixel_index] = altitude_in_meter;
 }

 out
}

/// Encode an altitudes Vec to an image with Nishioka-Nagatsu-2015 altitude tile method.
///
/// ## Params
///
/// - source: The souce image data.
/// - width: The source width in pixels. It might be 256 in the specification.
/// - resolution_factor: The resolution factor; If you use GSI's dem-png tile then it is 1.0e-2 .
///     - See also: <https://maps.gsi.go.jp/development/demtile.html>
///
/// ## Reference
///
/// - <https://www.jstage.jst.go.jp/article/geoinformatics/26/4/26_155/_article/-char/ja>
///
pub fn encode(source: &Vec<f64>, width: u32, resolution_factor: f64) -> DynamicImage
{
 let height = source.len() as u32 / width;

 let mut i = image::ImageBuffer::new(width, height);
 let pairs = source.iter().zip(i.enumerate_pixels_mut());

 for (from, (_, _, to)) in pairs
 {
  *to = if from.is_nan()
  {
   image::Rgb([128, 0, 0])
  }
  else
  {
   let scaled = (from / resolution_factor).round() as i32;
   let r = (((scaled >> 16) & 0xFF) | if scaled.is_negative() { 0x80 } else { 0x00 }) as u8;
   let g = ((scaled >> 8) & 0xFF) as u8;
   let b = (scaled & 0xFF) as u8;
   image::Rgb([r, g, b])
  }
 }

 DynamicImage::ImageRgb8(i)
}
