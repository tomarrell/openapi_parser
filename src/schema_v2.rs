use serde::Deserialize;

/// The root API struct.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub swagger: String,
    pub info: Info,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub schemes: Option<Vec<String>>, // TODO: Enumerate these instead of using String
    pub consumes: Option<Vec<String>>, // TODO: Enumerate these instead of using String
    pub produces: Option<Vec<String>>, // TODO: Enumerate these instead of using String
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDocs>,
}

/// Basic information about the API.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
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

/// External documentation information.
#[derive(Deserialize, Debug, PartialEq)]
pub struct ExternalDocs {
    pub description: Option<String>,
    pub url: String,
}

/// Additional metadata tag.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocs>,
}

#[cfg(test)]
mod tests {
    use super::*;
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
                base_path: Some("/api/".to_string()),
                schemes: Some(vec![
                    "http".to_string(),
                    "https".to_string(),
                    "ws".to_string()
                ]),
                consumes: Some(vec![
                    "application/json".to_string(),
                    "text/plain; charset=utf-8".to_string()
                ]),
                produces: Some(vec!["application/json".to_string(),]),
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
                },
                tags: Some(vec![
                    Tag {
                        name: "Tag1".to_string(),
                        description: Some("Tag1 desc".to_string()),
                        external_docs: Some(ExternalDocs {
                            description: Some("Tag1 extern docs desc".to_string()),
                            url: "https://tag1.com".to_string(),
                        }),
                    },
                    Tag {
                        name: "Tag2".to_string(),
                        description: Some("Tag2 desc".to_string()),
                        external_docs: None,
                    }
                ]),
                external_docs: Some(ExternalDocs {
                    description: Some("External docs desc".to_string()),
                    url: "https://swagger.io".to_string(),
                })
            })
        )
    }
}
