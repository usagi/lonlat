use arithmetic_sign::error::ArithmeticSignError;
use measurements::Angle;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CivilEngineeringLocationError
{
 #[error("Arithmetic-sign error.")]
 ArithmeticSignError(#[from] ArithmeticSignError),

 #[error("Could not process Nan.; Not a Number")]
 Nan,

 #[error("The value is infinite(NaN|+inf|-inf).")]
 Infinite,

 #[error("The source latitude is not available in the target.; source angle = {0}")]
 LatitudeDomainError(Angle),

 #[error("Could not parse the string to GeoURI.; source = {0:?}")]
 GeoUriParseError(String),

 #[error("Could not parse in Angle::from_iso_80000_1_dms.; source = {0:?}")]
 CouldNotParseFromIso800001Dms(String),

 #[error("")]
 ParseFloatError(#[from] std::num::ParseFloatError),

 #[error("Parse error occured around a sign(+/-).")]
 ParseErrorAroundSign,

 #[error("Could not detect an angle pattern.")]
 CouldNotDetectAnglePattern,

 #[error("Parse error occured around a NWSE.")]
 ParseErrorAroundNwse,

 #[cfg(feature = "ja-JP")]
 #[error("Parse error occured around a NWSE/ja-JP such as 北緯, 西経, 南緯 or 東経.")]
 ParseErrorAroundNwseJaJp,

 #[error("LonLat::from was failed. Unknown source pattern.")]
 LonLatFromStrUnknownPattern
}
