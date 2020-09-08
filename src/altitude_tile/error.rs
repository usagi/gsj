use thiserror::Error;

#[derive(Debug, Error)]
pub enum NishiokaNagatsuError
{
 #[error("The width or height is not 256 [pixels]; Width = {w}, Height = {h}")]
 SizeIsNot256x256Pixels
 {
  w: u32, h: u32
 },

 #[error("The color type is not RGB8 or RGBA8. Color = {0:?}")]
 ColorTypeIsNotRgb8OrRgba8(image::ColorType)
}
