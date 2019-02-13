//!
//! OpenAPI Parser is a utility to parse the OpenAPI v2.0 spec.
//!

use std::fs::File;

mod schema_v2;
use schema_v2::Spec;

/// Deserialize an OpenAPI spec given the file path.
/// 
/// # Example
/// ```
/// fn main() {
///     let spec: Spec = deserialize("examples/minimal_info.yaml");
/// }
/// ```
pub fn deserialize<P>(path: P) -> Result<Spec, String>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(path).expect("Failed to open path");
    Ok(serde_yaml::from_reader::<File, Spec>(file).expect("Failed to deserialize file"))
}
