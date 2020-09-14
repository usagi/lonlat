# lonlat

LonLat and LonLatAlt geo-location types and utils.

## Features

- [x] `LonLat` and `LonLatAlt` types; `lon: Angle`, `lat: Angle` (, `alt: Length` )
    - [x] `.try_from` -> `LonLat` -> `.to_string_XXX`
        - [x] A human readable degrees-minutes-seconds notation patterns. eg, `42°49′36”N 140°48′41”E`
        - [x] The GeoURI pattern. eg, `geo:42.826667,140.811389`
        - [x] Additional language "ja-JP"(Japanese; 日本語) supports. eg, `北緯42度49分36秒 東経140度48分41秒`
- [x] `civil_engineering_location::dimensions::Angle` = `measurement::Angle` + extension `trait`s
  - [x] `.as_string_radians` -> `"2.4670994982555996 [rad]"`
  - [x] `.as_string_degrees` -> `"141.35438888888888°"`
  - [x] `.as_string_minutes` -> `"8481.263333333332’"`
  - [x] `.as_string_seconds` -> `"508875.79999999993”"`
  - [x] `.as_string_dms_180` -> `+141°21’15.8”`; "ja-JP" feature -> `141度21分15.8秒`
  - [x] `.as_string_dms_ns` -> `"141°21’15.8”N"`; "ja-JP" feature -> `北緯141度21分15.8秒`
  - [x] `.as_string_dms_90` -> `+41°21’15.8”`; "ja-JP" feature -> `41度21分15.8秒`
  - [x] `.as_string_dms_ew` -> `"41°21’15.8”E"`; "ja-JP" feature -> `東経41度21分15.8秒`
  - [x] And more patterns.

Note: To enable `"ja-JP"` features if you need additional Japanese features.

## Example and Tests

- To see: [tests/](tests/) and [examples/](examples/).

## License

- [MIT](LICENSE.md)

## Author

- [USAGI.NETWORK / Usagi Ito](https://github.com/usagi)
