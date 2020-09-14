use arithmetic_sign::prelude::*;
use lonlat::prelude::*;

use approx::{
 assert_abs_diff_eq,
 assert_abs_diff_ne
};

#[test]
fn dms180()
{
 let angle = Angle::from_degrees(141.354389);
 let (sig, deg, min, sec) = angle.as_dms_180().unwrap();
 assert_eq!(sig, Sign::Positive);
 assert_eq!(deg, 141);
 assert_eq!(min, 21);
 // maybe: sec = 15.800_399_999_991_441

 assert_abs_diff_eq!(sec, 15.8, epsilon = 1.0e-1);
 assert_abs_diff_ne!(sec, 15.8, epsilon = 1.0e-4);
}

macro_rules! parse_detail {
 ($source:expr, $expected:expr) => {{
  let angle = Angle::from_dms_str($source).unwrap();
  let (sign, deg, min, sec) = angle.as_dms_180().unwrap();
  assert_eq!(sign, $expected.0);
  assert_eq!(deg, $expected.1);
  assert_eq!(min, $expected.2);
  assert_abs_diff_eq!(sec, $expected.3, epsilon = 1.0e-3);
 }};
}

#[test]
fn from_dms_str()
{
 parse_detail!("0°0′0″", (Sign::Positive, 0, 0, 0.0));

 parse_detail!("1°2′3.4″", (Sign::Positive, 1, 2, 3.4));
 parse_detail!("-1°2′3.4″", (Sign::Negative, 1, 2, 3.4));

 parse_detail!("141°21′15.8″", (Sign::Positive, 141, 21, 15.8));
 parse_detail!("-141°21′15.8″", (Sign::Negative, 141, 21, 15.8));

 parse_detail!("179°59′59.9″", (Sign::Positive, 179, 59, 59.9));
 parse_detail!("-179°59′59.9″", (Sign::Negative, 179, 59, 59.9));

 // DMS := [-180.0 deg .. +180.0 deg), thus +180 [deg] -> -180 [deg]
 parse_detail!("180°0′0″", (Sign::Negative, 180, 0, 0.0));
 parse_detail!("-180°0′0″", (Sign::Negative, 180, 0, 0.0));
}

#[test]
fn from_dms_str_with_direction()
{
 assert_eq!(
  Angle::from_dms_str_with_direction("42.826667").unwrap(),
  (Angle::from_degrees(42.826667), AngleDirectionNotation::None)
 );

 assert_eq!(
  Angle::from_dms_str_with_direction("42.826667N").unwrap(),
  (Angle::from_degrees(42.826667), AngleDirectionNotation::Latitude)
 );

 assert_eq!(
  Angle::from_dms_str_with_direction("42.826667E").unwrap(),
  (Angle::from_degrees(42.826667), AngleDirectionNotation::Longitude)
 );
}

#[test]
fn from_dms_str_space()
{
 parse_detail!("141° 21′ 15.8″", (Sign::Positive, 141, 21, 15.8));
 parse_detail!("-141° 21′ 15.8″", (Sign::Negative, 141, 21, 15.8));
 parse_detail!("43° 3′ 43.5″", (Sign::Positive, 43, 3, 43.5));
 parse_detail!("-43° 3′ 43.5″", (Sign::Negative, 43, 3, 43.5));
}
#[test]
fn from_dms_str_nwse()
{
 parse_detail!("141°21′15.8″E", (Sign::Positive, 141, 21, 15.8));
 parse_detail!("141°21′15.8″W", (Sign::Negative, 141, 21, 15.8));
 parse_detail!("43°3′43.5″N", (Sign::Positive, 43, 3, 43.5));
 parse_detail!("43°3′43.5″S", (Sign::Negative, 43, 3, 43.5));
}

#[cfg(feature = "ja-JP")]
#[test]
fn from_dms_ja_jp()
{
 parse_detail!("141度21分15.8秒″", (Sign::Positive, 141, 21, 15.8));
 parse_detail!("-141度21分15.8秒″", (Sign::Negative, 141, 21, 15.8));
 parse_detail!("43度3分43.5秒", (Sign::Positive, 43, 3, 43.5));
 parse_detail!("-43度3分43.5秒", (Sign::Negative, 43, 3, 43.5));
}

#[cfg(feature = "ja-JP")]
#[test]
fn from_dms_nwse_ja_jp()
{
 parse_detail!("東経141度21分15.8秒″", (Sign::Positive, 141, 21, 15.8));
 parse_detail!("西経141度21分15.8秒″", (Sign::Negative, 141, 21, 15.8));
 parse_detail!("北緯43度3分43.5秒", (Sign::Positive, 43, 3, 43.5));
 parse_detail!("南緯43度3分43.5秒", (Sign::Negative, 43, 3, 43.5));
}
