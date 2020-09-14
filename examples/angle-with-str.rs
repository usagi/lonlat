use lonlat::prelude::*;

fn print_conversions(title: &str, source: &str) -> Result<(), Box<dyn std::error::Error>>
{
 println!("[{}]", title);

 let angle = Angle::from_dms_str(source)?;

 println!(" source = {}", source);
 println!("  -> angle.as_string_radians => {}", angle.as_string_radians());
 println!("  -> angle.as_string_degrees => {}", angle.as_string_degrees());
 println!("  -> angle.as_string_minutes => {}", angle.as_string_minutes());
 println!("  -> angle.as_string_seconds => {}", angle.as_string_seconds());
 println!("  -> angle.as_string_dms_180 => {}", angle.as_string_dms_180().unwrap());
 println!("  -> angle.as_string_dms_ew  => {}", angle.as_string_dms_ew().unwrap());
 println!("  -> angle.as_string_dms_ns  => {}", angle.as_string_dms_ns().unwrap());

 #[cfg(feature = "ja-JP")]
 {
  println!(r#" #[feature="ja-JP"]"#);
  println!("  -> angle.as_string_degrees_ja_jp => {}", angle.as_string_degrees_ja_jp());
  println!("  -> angle.as_string_minutes_ja_jp => {}", angle.as_string_minutes_ja_jp());
  println!("  -> angle.as_string_seconds_ja_jp => {}", angle.as_string_seconds_ja_jp());
  println!("  -> angle.as_string_dms_180_ja_jp => {}", angle.as_string_dms_180_ja_jp().unwrap());
  println!("  -> angle.as_string_dms_ew_ja_jp  => {}", angle.as_string_dms_ew_ja_jp().unwrap());
  println!("  -> angle.as_string_dms_ns_ja_jp  => {}", angle.as_string_dms_ns_ja_jp().unwrap());
 }

 Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
 print_conversions("[DMS, ISO80000-1 Standard]", "141°21′15.8″")?;
 print_conversions("[DMS, with space]", "141° 21′ 15.8″")?;
 print_conversions("[DMS, EW]", "141° 21′ 15.8″W")?;

 print_conversions("[DMS, NWSE]", "43° 3′ 43.5″N")?;
 print_conversions("[DMS, NWSE]", "43° 3′ 43.5″W")?;
 print_conversions("[DMS, NWSE]", "43° 3′ 43.5″S")?;
 print_conversions("[DMS, NWSE]", "43° 3′ 43.5″E")?;

 #[cfg(feature = "ja-JP")]
 {
  print_conversions("[DMS, EW]", "西経141度21分15.8秒")?;
  print_conversions("[DMS, EW]", "東経141度21分15.8秒")?;
  print_conversions("[DMS, NS]", "北緯43度3分43.5秒″")?;
  print_conversions("[DMS, NS]", "南緯43度3分43.5秒″")?;
 }

 Ok(())
}
