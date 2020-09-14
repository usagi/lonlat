use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Location
{
 Keyword(String),
 LonLat(LonLat),
}

