use crate::prelude::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum Location
{
 Keyword(String),
 LonLat(LonLat),
}

