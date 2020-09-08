const GSI_DEM_PNG_SCALING_FACTOR: f64 = 1.0e-2;

#[test]
fn decode()
{
 let from_png = load_png();
 let from_txt = load_txt();

 let pairs = from_png.iter().zip(from_txt.iter());
 for (actual, expected) in pairs
 {
  assert_eq!(actual.is_nan(), expected.is_nan());
  if !actual.is_nan()
  {
   assert_eq!(actual.round(), expected.round());
  }
 }
}

#[test]
fn encode()
{
 let from_txt = load_txt();
 let encoded = gsj::altitude_tile::nishioka_nagatsu_2015::encode(&from_txt, 256, GSI_DEM_PNG_SCALING_FACTOR);
 let decoded = gsj::altitude_tile::nishioka_nagatsu_2015::decode(&encoded, GSI_DEM_PNG_SCALING_FACTOR);
 for (actual, expected) in decoded.iter().zip(decoded.iter())
 {
  assert_eq!(actual.is_nan(), expected.is_nan());
  if !actual.is_nan()
  {
   assert_eq!(actual.round(), expected.round());
  }
 }
}

fn load_png() -> Vec<f64>
{
 let png = image::open("tests/gsi-dem-z8-x229-y94.png").unwrap();
 gsj::altitude_tile::nishioka_nagatsu_2015::decode(&png, GSI_DEM_PNG_SCALING_FACTOR)
}

fn load_txt() -> Vec<f64>
{
 let from_text = std::fs::read_to_string("tests/gsi-dem-z8-x229-y94.txt").unwrap();
 let from_text = from_text
  .lines()
  .map(|line| line.split(","))
  .flatten()
  .map(|t| {
   let f_maybe = t.parse::<f64>();
   match f_maybe
   {
    Ok(f) => f,
    _ => std::f64::NAN
   }
  })
  .collect::<Vec<_>>();
 from_text
}
