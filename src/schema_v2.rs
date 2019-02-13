use serde::Deserialize;

/// The root API struct.
#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct Spec {
    pub swagger: String,
    pub info: Info,
    pub host: Option<String>,
}

/// Basic information about the API.
#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

/// Contact information regarding the API.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

/// The API's licensing information.
#[derive(Deserialize, Debug, PartialEq)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Info, Spec, Contact, License};
    use crate::deserialize;

    #[test]
    fn minimal_info_deserialize() {
        let path = "examples/minimal_info.yaml";
        assert_eq!(
            deserialize(path),
            Ok(Spec {
                swagger: "2.0".to_string(),
                info: Info {
                    title: "Hello".to_string(),
                    description: None,
                    terms_of_service: None,
                    contact: None,
                    license: None,
                    version: "2.0".to_string(),
                },
                ..Default::default()
            })
        )
    }

    #[test]
    fn info() {
        let path = "examples/info.yaml";
        assert_eq!(
            deserialize(path),
            Ok(Spec {
                swagger: "2.0".to_string(),
                host: Some("localhost:8080".to_string()),
                info: Info {
                    title: "Test Title".to_string(),
                    description: Some("A test API".to_string()),
                    terms_of_service: Some("Example terms".to_string()),
                    contact: Some(Contact {
                        name: Some("John Doe".to_string()),
                        url: Some("https://johndoe.com".to_string()),
                        email: Some("john@doe.com".to_string()),
                    }),
                    license: Some(License {
                        name: "MIT".to_string(),
                        url: Some("https://mit.com".to_string()),
                    }),
                    version: "1.0".to_string(),
                }
            })
        )
    }
}
