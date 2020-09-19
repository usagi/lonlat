use crate::prelude::*;
use measurements::{
 Angle,
 Length
};

pub trait LonLatGettable
{
 fn get_lon(&self) -> Angle;
 fn get_lat(&self) -> Angle;
}

pub trait LonLatSettable: Default
{
 fn set_lon(&mut self, lon: Angle);
 fn set_lat(&mut self, lat: Angle);
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LonLat
{
 pub lon: Angle,
 pub lat: Angle
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LonLatAlt
{
 pub lon: Angle,
 pub lat: Angle,
 pub alt: Length
}

impl std::convert::TryFrom<&str> for LonLat
{
 /// ## Supported common notations
 /// - DMS-like(1): 43°3′43.5″N,141°21′15.8″E
 /// - DMS-like(2): 43°3′43.5″N 141°21′15.8″E
 /// - DEC-like(1): 43.062083,141.354389
 /// - DEC-like(2): 43.062083 141.354389
 /// - GeoURI-like:	geo:43.062083,141.354389
 type Error = CivilEngineeringLocationError;

 fn try_from(source: &str) -> Result<Self, CivilEngineeringLocationError>
 {
  let source = source.trim();
  let source = if source.starts_with("geo:") { &source[4..] } else { &source[..] };

  // "," separated
  let separated = source.splitn(2, ",").collect::<Vec<_>>();
  let separated = if separated.len() < 2
  {
   // " " separated
   source.splitn(2, " ").collect::<Vec<_>>()
  }
  else
  {
   separated
  };

  let mut lat: Option<Angle> = None;
  let mut lon: Option<Angle> = None;
  for (index, source_part) in separated.into_iter().enumerate()
  {
   let (angle, direction) = Angle::from_dms_str_with_direction(source_part)?;
   match (index, direction)
   {
    (_, AngleDirectionNotation::Latitude) | (0, _) => lat = Some(angle),
    (_, AngleDirectionNotation::Longitude) | (1, _) => lon = Some(angle),
    _ => Err(CivilEngineeringLocationError::LonLatFromStrUnknownPattern)?
   }
  }

  match (lat, lon)
  {
   (Some(lat), Some(lon)) =>
   {
    Ok(Self {
     lat,
     lon
    })
   },
   _ => Err(CivilEngineeringLocationError::LonLatFromStrUnknownPattern)
  }
 }
}

impl std::convert::TryFrom<&str> for LonLatAlt
{
 /// ## Supported notation
 /// - GeoURI-like:	geo:43.062083,141.354389,123.45
 /// - LonLat-convertibles(1):	{`LonLat::try_from`-ables},123.45
 /// - LonLat-convertibles(2):	{`LonLat::try_from`-ables} 123.45
 type Error = CivilEngineeringLocationError;

 fn try_from(source: &str) -> Result<Self, CivilEngineeringLocationError>
 {
  let source = source.trim();
  let source = if source.starts_with("geo:") { &source[4..] } else { &source[..] };

  // "," separated
  let separated = source.splitn(3, ",").collect::<Vec<_>>();
  let separated = if separated.len() < 3
  {
   // " " separated
   source.splitn(3, " ").collect::<Vec<_>>()
  }
  else
  {
   separated
  };

  let alt = Length::from_meters(separated[2].parse::<f64>()?);
  let lonlat = LonLat::try_from(&format!("{},{}", separated[0], separated[1])[..])?;

  Ok(LonLatAlt::from((lonlat, alt)))
 }
}

impl LonLat
{
 pub fn new(lon: Angle, lat: Angle) -> Self
 {
  Self {
   lon,
   lat
  }
 }
}

impl LonLatAlt
{
 pub fn new(lon: Angle, lat: Angle, alt: Length) -> Self
 {
  Self {
   lon,
   lat,
   alt
  }
 }
}

pub trait AsLonLat
{
 fn as_lonlat(&self) -> LonLat;
}

impl<T: LonLatGettable> AsLonLat for T
{
 fn as_lonlat(&self) -> LonLat
 {
  LonLat {
   lon: self.get_lon(),
   lat: self.get_lat()
  }
 }
}

impl Default for LonLat
{
 fn default() -> Self
 {
  Self {
   lon: Angle::from_radians(0.0),
   lat: Angle::from_radians(0.0)
  }
 }
}

impl Default for LonLatAlt
{
 fn default() -> Self
 {
  Self {
   lon: Angle::from_radians(0.0),
   lat: Angle::from_radians(0.0),
   alt: Length::from_meters(0.0)
  }
 }
}

impl From<LonLatAlt> for LonLat
{
 fn from(source: LonLatAlt) -> Self
 {
  Self {
   lon: source.lon,
   lat: source.lat
  }
 }
}

impl From<(LonLat, Length)> for LonLatAlt
{
 fn from(source: (LonLat, Length)) -> Self
 {
  Self {
   lon: source.0.lon,
   lat: source.0.lat,
   alt: source.1
  }
 }
}

impl LonLatGettable for LonLat
{
 fn get_lat(&self) -> Angle
 {
  self.lat
 }

 fn get_lon(&self) -> Angle
 {
  self.lon
 }
}

impl LonLatSettable for LonLat
{
 fn set_lat(&mut self, lat: Angle)
 {
  self.lat = lat;
 }

 fn set_lon(&mut self, lon: Angle)
 {
  self.lon = lon;
 }
}

impl LonLatGettable for LonLatAlt
{
 fn get_lat(&self) -> Angle
 {
  self.lat
 }

 fn get_lon(&self) -> Angle
 {
  self.lon
 }
}

impl LonLatSettable for LonLatAlt
{
 fn set_lat(&mut self, lat: Angle)
 {
  self.lat = lat;
 }

 fn set_lon(&mut self, lon: Angle)
 {
  self.lon = lon;
 }
}

pub trait ToStringGeoUri
{
 fn to_string_geo_uri(&self) -> Result<String, CivilEngineeringLocationError>;
}

pub trait ToStringDms
{
 fn to_string_dms(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>;
 fn to_string_dms_comma(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms(",")
 }
 fn to_string_dms_space(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms(" ")
 }
}

#[cfg(feature = "ja-JP")]
pub trait ToStringDmsJaJp
{
 fn to_string_dms_ja_jp(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>;
 fn to_string_dms_comma_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_ja_jp(",")
 }
 fn to_string_dms_space_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_ja_jp(" ")
 }
}

pub trait ToStringDmsNwse
{
 fn to_string_dms_nwse(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>;
 fn to_string_dms_nwse_comma(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_nwse(",")
 }
 fn to_string_dms_nwse_space(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_nwse(" ")
 }
}

#[cfg(feature = "ja-JP")]
pub trait ToStringDmsNwseJaJp
{
 fn to_string_dms_nwse_ja_jp(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>;
 fn to_string_dms_nwse_comma_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_nwse_ja_jp(",")
 }
 fn to_string_dms_nwse_space_ja_jp(&self) -> Result<String, CivilEngineeringLocationError>
 {
  self.to_string_dms_nwse_ja_jp(" ")
 }
}

impl ToStringGeoUri for LonLat
{
 fn to_string_geo_uri(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.get_lon().as_degrees();
  let lat = self.get_lat().as_degrees();
  match lon.is_infinite() || lat.is_infinite()
  {
   true => Err(CivilEngineeringLocationError::Infinite),
   false => Ok(format!("geo:{},{}", lat, lon))
  }
 }
}

impl ToStringGeoUri for LonLatAlt
{
 fn to_string_geo_uri(&self) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.lon.as_degrees();
  let lat = self.lat.as_degrees();
  let alt = self.alt.as_meters();
  match lon.is_infinite() || lat.is_infinite()
  {
   true => Err(CivilEngineeringLocationError::Infinite),
   false => Ok(format!("geo:{},{},{}", lat, lon, alt))
  }
 }
}

impl<T: LonLatGettable> ToStringDms for T
{
 fn to_string_dms(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.get_lon().as_string_dms_180()?;
  let lat = self.get_lat().as_string_dms_90()?;
  Ok(format!("{}{}{}", lat, separator, lon))
 }
}

#[cfg(feature = "ja-JP")]
impl<T: LonLatGettable> ToStringDmsJaJp for T
{
 fn to_string_dms_ja_jp(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.get_lon().as_string_dms_180_ja_jp()?;
  let lat = self.get_lat().as_string_dms_90_ja_jp()?;
  Ok(format!("{}{}{}", lat, separator, lon))
 }
}

impl<T: LonLatGettable> ToStringDmsNwse for T
{
 fn to_string_dms_nwse(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.get_lon().as_string_dms_ew()?;
  let lat = self.get_lat().as_string_dms_ns()?;
  Ok(format!("{}{}{}", lat, separator, lon))
 }
}

#[cfg(feature = "ja-JP")]
impl<T: LonLatGettable> ToStringDmsNwseJaJp for T
{
 fn to_string_dms_nwse_ja_jp(&self, separator: &str) -> Result<String, CivilEngineeringLocationError>
 {
  let lon = self.get_lon().as_string_dms_ew_ja_jp()?;
  let lat = self.get_lat().as_string_dms_ns_ja_jp()?;
  Ok(format!("{}{}{}", lat, separator, lon))
 }
}
