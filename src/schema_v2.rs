use serde::Deserialize;

/// The root API struct.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Spec {
    pub info: Info,
}

/// Basic information about the API.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

/// Contact information regarding the API.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub email: String,
}

/// The API's licensing information.
#[derive(Deserialize, Debug, PartialEq)]
pub struct License {
    pub name: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::{Info, Spec};
    use crate::deserialize;

    #[test]
    fn minimal_info_deserialize() {
        let path = "examples/minimal_info.yaml";
        assert_eq!(
            deserialize(path),
            Ok(Spec {
                info: Info {
                    title: "Hello".to_string(),
                    description: None,
                    terms_of_service: None,
                    contact: None,
                    license: None,
                    version: "2.0".to_string(),
                }
            })
        )
    }
}
