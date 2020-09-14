use lonlat::prelude::*;
use std::convert::TryFrom;

use approx::assert_abs_diff_eq;

#[test]
fn lonlat_from()
{
 let expected_lon = Angle::from_degrees(140.811389);
 let expected_lat = Angle::from_degrees(42.826667);
 let expected = LonLat::new(expected_lon, expected_lat);

 assert_eq!(LonLat::try_from("geo:42.826667,140.811389").unwrap(), expected);
 assert_eq!(LonLat::try_from("42.826667,140.811389").unwrap(), expected);

 // Note: UTF-8 has a multiple minutes-like or seconds-like characters; [′’'] [″”"]
 let actual = LonLat::try_from("42°49′36”N 140°48′41”E").unwrap();
 assert_abs_diff_eq!(actual.get_lon().as_degrees(), expected.get_lon().as_degrees(), epsilon = 1.0e-6);
 assert_abs_diff_eq!(actual.get_lat().as_degrees(), expected.get_lat().as_degrees(), epsilon = 1.0e-6);
 let actual = LonLat::try_from("42°49’36”N 140°48’41”E").unwrap();
 assert_abs_diff_eq!(actual.get_lon().as_degrees(), expected.get_lon().as_degrees(), epsilon = 1.0e-6);
 assert_abs_diff_eq!(actual.get_lat().as_degrees(), expected.get_lat().as_degrees(), epsilon = 1.0e-6);
 let actual = LonLat::try_from(r#"42°49'36"N 140°48'41"E"#).unwrap();
 assert_abs_diff_eq!(actual.get_lon().as_degrees(), expected.get_lon().as_degrees(), epsilon = 1.0e-6);
 assert_abs_diff_eq!(actual.get_lat().as_degrees(), expected.get_lat().as_degrees(), epsilon = 1.0e-6);
}

#[test]
fn lonlatalt_from()
{
 let expected_lon = Angle::from_degrees(140.811389);
 let expected_lat = Angle::from_degrees(42.826667);
 let expected_alt = Length::from_meters(123.4);
 let expected = LonLatAlt::new(expected_lon, expected_lat, expected_alt);

 assert_eq!(LonLatAlt::try_from("geo:42.826667,140.811389,123.4").unwrap(), expected);
}
#[test]
fn lonlat_to()
{
 let actual_lon = Angle::from_degrees(140.811389);
 let actual_lat = Angle::from_degrees(42.826667);
 let actual = LonLat::new(actual_lon, actual_lat);

 assert_eq!(actual.to_string_geo_uri().unwrap(), "geo:42.826667,140.811389");
 assert_eq!(actual.to_string_dms_nwse_space().unwrap(), "42°49’36.0”N 140°48’41.0”E");
 assert_eq!(actual.to_string_dms_nwse_comma().unwrap(), "42°49’36.0”N,140°48’41.0”E");
 assert_eq!(actual.to_string_dms_nwse(", ").unwrap(), "42°49’36.0”N, 140°48’41.0”E");
}
#[test]
fn lonlatalt_to()
{
 let actual_lon = Angle::from_degrees(140.811389);
 let actual_lat = Angle::from_degrees(42.826667);
 let actual_alt = Length::from_meters(123.4);
 let actual = LonLatAlt::new(actual_lon, actual_lat, actual_alt);

 assert_eq!(actual.to_string_geo_uri().unwrap(), "geo:42.826667,140.811389,123.4");
}

#[cfg(feature = "ja-JP")]
#[test]
fn lonlat_from_ja_jp()
{
 let expected_lon = Angle::from_degrees(140.811389);
 let expected_lat = Angle::from_degrees(42.826667);
 let expected = LonLat::new(expected_lon, expected_lat);

 let actual = LonLat::try_from("北緯42度49分36秒 東経140度48分41秒").unwrap();
 assert_abs_diff_eq!(actual.get_lon().as_degrees(), expected.get_lon().as_degrees(), epsilon = 1.0e-6);
 assert_abs_diff_eq!(actual.get_lat().as_degrees(), expected.get_lat().as_degrees(), epsilon = 1.0e-6);
}

#[cfg(feature = "ja-JP")]
#[test]
fn lonlat_to_ja_jp()
{
 let actual_lon = Angle::from_degrees(140.811389);
 let actual_lat = Angle::from_degrees(42.826667);
 let actual = LonLat::new(actual_lon, actual_lat);

 assert_eq!(
  actual.to_string_dms_nwse_space_ja_jp().unwrap(),
  "北緯42度49分36.0秒 東経140度48分41.0秒"
 );
 assert_eq!(
  actual.to_string_dms_nwse_comma_ja_jp().unwrap(),
  "北緯42度49分36.0秒,東経140度48分41.0秒"
 );
 assert_eq!(
  actual.to_string_dms_nwse_ja_jp(", ").unwrap(),
  "北緯42度49分36.0秒, 東経140度48分41.0秒"
 );
}
