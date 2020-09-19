use lonlat::prelude::*;
use std::io::prelude::*;
use std::convert::TryFrom;

// Usage:
//   Step.1 => `cargo run --example geouri_to_dms`.
//   Step.2 => Input a GeoURI, for eg `geouri:42.826667,140.811389`.
//   Then you will get the result like `42°49’36.0”N 140°48’41.0”E`.
fn main()
{
 // Read 1-line from STDIN
 let mut buffer= String::new();
 std::io::stdin().lock().read_line(&mut buffer).unwrap();

 println!(r#"Your input is: "{}""#, &buffer);

 // Parse
 let lonlat = LonLat::try_from(&buffer[..]).unwrap();
 // To String(DMS+NWSE)
 let dms = lonlat.to_string_dms_nwse_space().unwrap();

 println!(r#"GeoURI -> DMS(Degrees-Minutes-Seconds): {}"#, &dms );
}
