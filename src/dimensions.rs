use crate::prelude::*;

pub use measurements::{
 Angle,
 Length
};

use arithmetic_sign::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

#[cfg(not(feature = "ja-JP"))]
pub const REGEX_DMS_PATTERN: &str = r#"^(?P<head>[^\d+-]*?)(?P<sign>[+-]{0,1})(?:(?:(?P<deg>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1})°){0,1}(?:[ ]*(?P<min>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+)?)[′’']){0,1}(?:[ ]*(?P<sec>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1})[″”"]){0,1}|(?P<deg_only>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1}))(?:[ ]*(?P<nwse>[NWSE]{0,1}))(?P<tail>[^\d]*?)$"#;
#[cfg(feature = "ja-JP")]
pub const REGEX_DMS_PATTERN: &str = r#"^(?P<head>[^\d+-]*?)(?:[ ]*(?P<nwse_ja_jp>(北緯|西経|南緯|東経){0,1}))(?P<sign>[+-]{0,1})(?:(?:(?P<deg>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1})[°度]){0,1}(?:[ ]*(?P<min>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1})[′’'分]){0,1}(?:[ ]*(?P<sec>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1})[″”"秒]){0,1}|(?P<deg_only>[\d]*\.{0,1}[\d]+(?:[eE][-+]{0,1}[\d]+){0,1}))(?:[ ]*(?P<nwse>[NWSE]{0,1}))(?P<tail>[^\d]*)$"#;

pub const REGEX_DMS_PATTERN_SIGN: &str = "sign";
pub const REGEX_DMS_PATTERN_DEG: &str = "deg";
pub const REGEX_DMS_PATTERN_DEG_ONLY: &str = "deg_only";
pub const REGEX_DMS_PATTERN_MIN: &str = "min";
pub const REGEX_DMS_PATTERN_SEC: &str = "sec";
pub const REGEX_DMS_PATTERN_NWSE: &str = "nwse";
#[cfg(feature = "ja-JP")]
pub const REGEX_DMS_PATTERN_NWSE_JA_JP: &str = "nwse_ja_jp";
pub const REGEX_DMS_PATTERN_HEAD: &str = "head";
pub const REGEX_DMS_PATTERN_TAIL: &str = "tail";

pub const UNIT_SYMBOL_DEGREES: char = '°';
pub const UNIT_SYMBOL_MINUTES: char = '’';
pub const UNIT_SYMBOL_SECONDS: char = '”';

pub const ZERO: f64 = 0.0;
pub const ONE: f64 = 1.0;
pub use std::f64::consts::PI;
pub const PI_MUL_2: f64 = std::f64::consts::PI * 2.0;
pub const PI_DIV_2: f64 = std::f64::consts::PI / 2.0;

pub const F360: f64 = 360.0;
pub const F180: f64 = 180.0;
pub const F90: f64 = 90.0;
pub const F60: f64 = 60.0;

/// Note: 1.0 [sec] ≈ 30 [m], 1.0e-4 [sec] ≈ 3.0 [mm]
pub const ANGLE_MINIMUM_SECONDS: f64 = 1.0e-4;
/// - Note:
///     - 180*60*60/1.0e-4 = 23_327_999_999_999.996
///     - Decimal digits of this dynamic-range ≈ 13.37 < 15.95 ≈ Decimal digits of IEEE754/Binary64
///     - 32-bit < log2(23e+12)≈44.41 < 64-bit
pub const ANGLE_PI_SECONDS: f64 = F180 * 60.0 * 60.0;

lazy_static! {
 pub static ref REGEX_DMS: Regex = Regex::new(REGEX_DMS_PATTERN).unwrap();
}

pub trait AngleAsNormalize
{
 /// return Angle := [0..π)
 fn as_normalize_0_pi(&self) -> Self;
 /// return Angle := [-π..π); for eg longitude
 fn as_normalize_negative_pi_positive_pi(&self) -> Self;
 /// return Angle := [-π/2..π/2]; for eg latitude
 fn as_normalize_negative_half_pi_positive_half_pi(&self) -> Self;
}

pub trait AngleAsStringIso800001: AngleAsDms
{
 fn as_string_radians(&self) -> String;

 fn as_string_degrees(&self) -> String;
 fn as_string_minutes(&self) -> String;
 fn as_string_seconds(&self) -> String;
 fn as_string_dms_360(&self) -> Result<String, CivilEngineeringLocationError>;
 fn as_string_dms_180(&self) -> Result<String, CivilEngineeringLocationError>;
 fn as_string_dms_90(&self) -> Result<String, CivilEngineeringLocationError>;
}

#[cfg(feature = "ja-JP")]
pub trait AngleAsStringIso800001JaJp: AngleAsDms
{
 fn as_string_degrees_ja_jp(&self) -> String;
 fn as_string_minutes_ja_jp(&self) -> String;
 fn as_string_seconds_ja_jp(&self) -> String;
 fn as_string_dms_360_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>;
 fn as_string_dms_180_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>;
 fn as_string_dms_90_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>;
}

pub trait AngleAsStringDmsNwse: AngleAsDms
{
 fn as_string_dms_ns(&self) -> Result<String, CivilEngineeringLocationError>;
 fn as_string_dms_ew(&self) -> Result<String, CivilEngineeringLocationError>;

 #[cfg(feature = "ja-JP")]
 fn as_string_dms_ns_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>;
 #[cfg(feature = "ja-JP")]
 fn as_string_dms_ew_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>;
}

pub trait AngleAsDms
{
 /// return ( degrees := [0..360], minutes, seconds )
 fn as_dms_360(&self) -> Result<(u16, u8, f64), CivilEngineeringLocationError>;

 /// return ( sign, degrees := [0..180], minutes, seconds )
 fn as_dms_180(&self) -> Result<(Sign, u8, u8, f64), CivilEngineeringLocationError>;

 /// return ( sign, degrees := [0..90], minutes, seconds )
 fn as_dms_90(&self) -> Result<(Sign, u8, u8, f64), CivilEngineeringLocationError>;

 fn as_minutes(&self) -> f64;
 fn as_seconds(&self) -> f64;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AngleDirectionNotation
{
 Longitude,
 Latitude,
 None
}

pub trait FromDmsStr<T: Sized>: FromDms
{
 fn from_dms_str(source: &str) -> Result<T, CivilEngineeringLocationError>
 {
  Self::from_dms_str_with_direction(source).map(|(a, _)| a)
 }

 fn from_dms_str_with_direction(source: &str) -> Result<(T, AngleDirectionNotation), CivilEngineeringLocationError>;
}

impl AngleAsNormalize for Angle
{
 fn as_normalize_0_pi(&self) -> Self
 {
  let rad = self.as_radians();
  let rad = if rad < ZERO { (rad % PI_MUL_2) + PI_MUL_2 } else { rad % PI_MUL_2 };
  Self::from_radians(rad)
 }

 fn as_normalize_negative_pi_positive_pi(&self) -> Self
 {
  let rad = self.as_radians();

  let rad = if rad >= PI
  {
   // -> [π..2π)
   let rad = rad % PI_MUL_2;
   // -> [-π..0)
   rad - PI_MUL_2
  }
  else if rad < PI
  {
   let negative_rotation_count = (rad / PI_MUL_2).floor();
   // -> [0..2π)
   let rad = rad + PI_MUL_2 * (negative_rotation_count + 1.0);
   if rad >= PI
   {
    rad - PI_MUL_2
   }
   else
   {
    rad
   }
  }
  else
  {
   rad
  };
  Self::from_radians(rad)
 }

 fn as_normalize_negative_half_pi_positive_half_pi(&self) -> Self
 {
  let rad = self.as_radians();
  let rad = if rad < ZERO { (rad % PI_MUL_2) + PI_MUL_2 } else { rad % PI_MUL_2 };
  let rad = if rad >= PI { rad - PI_MUL_2 } else { rad };
  let rad = if rad > PI_DIV_2
  {
   PI_DIV_2 - (rad - PI_DIV_2)
  }
  else if rad < -PI_DIV_2
  {
   -PI_DIV_2 - (rad - -PI_DIV_2)
  }
  else
  {
   rad
  };
  Self::from_radians(rad)
 }
}

impl AngleAsStringIso800001 for Angle
{
 fn as_string_radians(&self) -> String
 {
  format!("{} [rad]", self.as_radians())
 }

 fn as_string_degrees(&self) -> String
 {
  format!("{}°", self.as_degrees())
 }

 fn as_string_minutes(&self) -> String
 {
  format!("{}’", self.as_degrees() * F60)
 }

 fn as_string_seconds(&self) -> String
 {
  format!("{}”", self.as_degrees() * F60 * F60)
 }

 fn as_string_dms_360(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (deg, min, sec) = self.as_dms_360()?;
  Ok(format!("{}{}°{}’{:.1}”", Sign::Positive, deg, min, sec))
 }

 fn as_string_dms_180(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_180()?;
  Ok(format!("{}{}°{}’{:.1}”", sign, deg, min, sec))
 }

 fn as_string_dms_90(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_90()?;
  Ok(format!("{}{}°{}’{:.1}”", sign, deg, min, sec))
 }
}

#[cfg(feature = "ja-JP")]
impl AngleAsStringIso800001JaJp for Angle
{
 fn as_string_degrees_ja_jp(&self) -> String
 {
  format!("{}度", self.as_degrees())
 }

 fn as_string_minutes_ja_jp(&self) -> String
 {
  format!("{}分", self.as_degrees() * F60)
 }

 fn as_string_seconds_ja_jp(&self) -> String
 {
  format!("{}秒", self.as_degrees() * F60 * F60)
 }

 fn as_string_dms_360_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (deg, min, sec) = self.as_dms_360()?;
  Ok(format!("{}{}度{}分{:.1}秒", Sign::Positive, deg, min, sec))
 }

 fn as_string_dms_180_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_180()?;
  Ok(format!("{}{}度{}分{:.1}秒", sign, deg, min, sec))
 }

 fn as_string_dms_90_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_90()?;
  Ok(format!("{}{}度{}分{:.1}秒", sign, deg, min, sec))
 }
}

impl AngleAsDms for Angle
{
 fn as_dms_360(&self) -> Result<(u16, u8, f64), CivilEngineeringLocationError>
 {
  if self.as_degrees().is_infinite()
  {
   Err(CivilEngineeringLocationError::Infinite)?;
  }

  let degrees = self.as_normalize_0_pi().as_degrees();

  let part_of_degrees = degrees.floor() as u16;
  let part_of_minutes = ((degrees % ONE) * F60).floor() as u8;
  let part_of_seconds = (((degrees % ONE) * F60) % ONE) * F60;

  Ok((part_of_degrees, part_of_minutes, part_of_seconds))
 }

 fn as_dms_180(&self) -> Result<(Sign, u8, u8, f64), CivilEngineeringLocationError>
 {
  if self.as_degrees().is_infinite()
  {
   Err(CivilEngineeringLocationError::Infinite)?;
  }

  // tsf; Total-Seconds in Float
  let tsf = self.as_normalize_negative_pi_positive_pi().as_seconds();
  let part_of_sign = Sign::try_from(tsf)?;
  // tsm: Total-Seconds Magnified
  let tsm = (tsf / ANGLE_MINIMUM_SECONDS).round() as i64;
  // tsi; Total-Seconds in Integer
  let tsi = tsm / ((ONE / ANGLE_MINIMUM_SECONDS) as i64);
  let part_of_degrees = tsi / 60 / 60;
  let _p = part_of_degrees;
  // ds; part-of-Degrees in Seconds
  let ds = part_of_degrees * 60 * 60;
  let part_of_minutes = (tsi - ds) / 60;
  // ms; part-of-Minutes in Seconds
  let ms = part_of_minutes * 60;
  // dm; part-of-Degrees + part-of-Minutes in Seconds
  let dms = ds + ms;
  // dmf; part-of-Degrees + part-of-Minutes in Seconds Magnified
  let dmsm = dms * ((ONE / ANGLE_MINIMUM_SECONDS) as i64);
  let part_of_seconds = (tsm - dmsm).abs() as f64 * ANGLE_MINIMUM_SECONDS;
  let part_of_minutes = part_of_minutes.abs() as u8;
  let part_of_degrees = part_of_degrees.abs() as u8;

  Ok((part_of_sign, part_of_degrees, part_of_minutes, part_of_seconds))
 }

 fn as_dms_90(&self) -> Result<(Sign, u8, u8, f64), CivilEngineeringLocationError>
 {
  if self.as_degrees().is_infinite()
  {
   Err(CivilEngineeringLocationError::Infinite)?;
  }

  let degrees = self.as_normalize_negative_half_pi_positive_half_pi().as_degrees();

  let part_of_sign = Sign::try_from(degrees)?;
  let part_of_degrees = degrees.abs().floor() as u8;
  let part_of_minutes = ((degrees.abs() % ONE) * F60).floor() as u8;
  let part_of_seconds = (((degrees.abs() % ONE) * F60) % ONE) * F60;

  Ok((part_of_sign, part_of_degrees, part_of_minutes, part_of_seconds))
 }

 fn as_minutes(&self) -> f64
 {
  self.as_degrees() * F60
 }

 fn as_seconds(&self) -> f64
 {
  self.as_minutes() * F60
 }
}

impl AngleAsStringDmsNwse for Angle
{
 fn as_string_dms_ns(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_90()?;
  let ns = sign.to_string_ns();
  Ok(format!("{}°{}’{:.1}”{}", deg, min, sec, ns))
 }

 fn as_string_dms_ew(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_180()?;
  let ew = sign.to_string_ew();
  Ok(format!("{}°{}’{:.1}”{}", deg, min, sec, ew))
 }

 #[cfg(feature = "ja-JP")]
 fn as_string_dms_ns_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_90()?;
  let ns_ja_jp = sign.to_string_ns_ja_jp();
  Ok(format!("{}{}度{}分{:.1}秒", ns_ja_jp, deg, min, sec))
 }

 #[cfg(feature = "ja-JP")]
 fn as_string_dms_ew_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let (sign, deg, min, sec) = self.as_dms_180()?;
  let ew_ja_jp = sign.to_string_ew_ja_jp();
  Ok(format!("{}{}度{}分{:.1}秒", ew_ja_jp, deg, min, sec))
 }
}

pub trait FromDms
{
 fn from_dms(degrees: f64, minutes: f64, seconds: f64) -> Result<Self, CivilEngineeringLocationError>
 where
  Self: Sized;
 fn from_minutes(minutes: f64) -> Self;
 fn from_seconds(seconds: f64) -> Self;
}

impl FromDms for Angle
{
 fn from_dms(degrees: f64, minutes: f64, seconds: f64) -> Result<Self, CivilEngineeringLocationError>
 {
  let sd = Sign::try_from(degrees)?;
  let sm = Sign::try_from(minutes)?;
  let ss = Sign::try_from(seconds)?;

  let sign = sd * sm * ss;
  let degrees_of_seconds_part = seconds.abs() / F60 / F60;
  let degrees_of_minutes_part = minutes.abs() / F60;
  let total = sign.as_f64() * (degrees.abs() + degrees_of_minutes_part + degrees_of_seconds_part);
  Ok(Self::from_degrees(total))
 }

 fn from_minutes(minutes: f64) -> Self
 {
  Self::from_degrees(minutes / F60)
 }

 fn from_seconds(seconds: f64) -> Self
 {
  Self::from_degrees(seconds / F60 / F60)
 }
}

impl FromDmsStr<Angle> for Angle
{
 fn from_dms_str_with_direction(source: &str) -> Result<(Angle, AngleDirectionNotation), CivilEngineeringLocationError>
 {
  let capture = REGEX_DMS
   .captures(source)
   .ok_or(CivilEngineeringLocationError::CouldNotParseFromIso800001Dms(source.into()))?;

  // eprintln!("{:?}", &capture);

  let (sign, direction) = match capture
   .name(REGEX_DMS_PATTERN_SIGN)
   .ok_or(CivilEngineeringLocationError::ParseErrorAroundSign)?
   .as_str()
  {
   "-" => (-ONE, AngleDirectionNotation::None),
   "+" => (ONE, AngleDirectionNotation::None),
   "" =>
   {
    match capture
     .name(REGEX_DMS_PATTERN_NWSE)
     .ok_or(CivilEngineeringLocationError::ParseErrorAroundNwse)?
     .as_str()
    {
     "N" => (ONE, AngleDirectionNotation::Latitude),
     "W" => (-ONE, AngleDirectionNotation::Longitude),
     "S" => (-ONE, AngleDirectionNotation::Latitude),
     "E" => (ONE, AngleDirectionNotation::Longitude),

     #[cfg(not(feature = "ja-JP"))]
     "" => (ONE, AngleDirectionNotation::None),

     #[cfg(feature = "ja-JP")]
     "" =>
     {
      match capture
       .name(REGEX_DMS_PATTERN_NWSE_JA_JP)
       .ok_or(CivilEngineeringLocationError::ParseErrorAroundNwseJaJp)?
       .as_str()
      {
       "北緯" => (ONE, AngleDirectionNotation::Latitude),
       "西経" => (-ONE, AngleDirectionNotation::Longitude),
       "南緯" => (-ONE, AngleDirectionNotation::Latitude),
       "東経" => (ONE, AngleDirectionNotation::Longitude),
       "" => (ONE, AngleDirectionNotation::None),

       _ => Err(CivilEngineeringLocationError::ParseErrorAroundNwse)?
      }
     },

     _ => Err(CivilEngineeringLocationError::ParseErrorAroundNwse)?
    }
   },
   _ => Err(CivilEngineeringLocationError::ParseErrorAroundNwse)?
  };

  if capture.name(REGEX_DMS_PATTERN_DEG).is_none()
   && capture.name(REGEX_DMS_PATTERN_MIN).is_none()
   && capture.name(REGEX_DMS_PATTERN_SEC).is_none()
  {
   match capture.name(REGEX_DMS_PATTERN_DEG_ONLY)
   {
    Some(m) =>
    {
     let deg_only = m.as_str().parse::<f64>()?;
     Ok((Self::from_degrees(deg_only), direction))
    },
    None => Err(CivilEngineeringLocationError::CouldNotDetectAnglePattern)
   }
  }
  else
  {
   let deg = match capture.name(REGEX_DMS_PATTERN_DEG)
   {
    Some(m) => m.as_str().parse::<f64>()?,
    None => 0.0
   };

   let min = match capture.name(REGEX_DMS_PATTERN_MIN)
   {
    Some(m) => m.as_str().parse::<f64>()?,
    None => 0.0
   };

   let sec = match capture.name(REGEX_DMS_PATTERN_SEC)
   {
    Some(m) => m.as_str().parse::<f64>()?,
    None => 0.0
   };

   Ok((Self::from_dms(sign * deg, min, sec)?, direction))
  }
 }
}

pub trait ToStringNS
{
 fn to_string_ns(&self) -> &str;

 #[cfg(feature = "ja-JP")]
 fn to_string_ns_ja_jp(&self) -> &str;
}

impl ToStringNS for Sign
{
 fn to_string_ns(&self) -> &str
 {
  self.to_string_specific("S", "N")
 }

 #[cfg(feature = "ja-JP")]
 fn to_string_ns_ja_jp(&self) -> &str
 {
  self.to_string_specific("南緯", "北緯")
 }
}

pub trait ToStringEW
{
 fn to_string_ew(&self) -> &str;

 #[cfg(feature = "ja-JP")]
 fn to_string_ew_ja_jp(&self) -> &str;
}

impl ToStringEW for Sign
{
 fn to_string_ew(&self) -> &str
 {
  self.to_string_specific("W", "E")
 }

 #[cfg(feature = "ja-JP")]
 fn to_string_ew_ja_jp(&self) -> &str
 {
  self.to_string_specific("西経", "東経")
 }
}
