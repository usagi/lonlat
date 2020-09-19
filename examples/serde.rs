use lonlat::prelude::*;

fn main()
{
 // LonLat
 let lonlat = LonLat::new(Angle::from_degrees(140.811389), Angle::from_degrees(42.826667));

 // Serialize to JSON use serde
 let json_maybe = serde_json::to_string(&lonlat);
 // Show result
 println!("json_maybe => {:?}", &json_maybe);

 // Deserialize from JSON
 let lonlat_maybe = serde_json::from_str::<LonLat>(&json_maybe.unwrap());
 println!("lonlat_maybe => {:?}", lonlat_maybe);
}
