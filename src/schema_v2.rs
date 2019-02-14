use serde::Deserialize;
use std::collections::BTreeMap;

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
    pub paths: BTreeMap<String, PathItem>,
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
#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

/// The API's licensing information.
#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

/// External documentation information.
#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct ExternalDocs {
    pub description: Option<String>,
    pub url: String,
}

/// Additional metadata tag.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocs>,
}

/// The PathItem describes the operations available
/// on a single path.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    #[serde(rename = "$ref")]
    pub ref_path: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
}

/// A single API operation on a path.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    tags: Option<Vec<String>>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<ExternalDocs>,
    operation_id: Option<String>,
    consumes: Option<Vec<String>>,
    produces: Option<Vec<String>>,
    parameters: Option<Vec<Parameter>>, // TODO: or ref
    responses: BTreeMap<String, Response>, // TODO: or ref
}

/// A single operation parameter.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    name: String,
    #[serde(rename = "in")]
    in_loc: String,
    description: Option<String>,
    required: Option<bool>,
    // schema: TODO
    #[serde(rename = "type")]
    param_type: String,
    format: Option<String>,
    // TODO: Flesh this out
}

/// The expected response of an operation.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    description: String,
    schema: Option<Schema>,
    headers: Option<BTreeMap<String, Header>>,
    examples: Option<BTreeMap<String, String>>, // TODO: allow for other example types
}

/// Definition of input and output data types.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$ref")]
    ref_path: Option<String>,
    #[serde(rename = "type")]
    schema_type: String,
    format: Option<String>,
    title: Option<String>,
    description: Option<String>,
    // default: TODO
    multiple_of: Option<i32>,
    maximum: Option<i32>,
    exclusive_maximum: Option<bool>,
    minimum: Option<i32>,
    exclusive_minimum: Option<bool>,
    max_length: Option<i32>,
    min_length: Option<i32>,
    pattern: Option<String>,
    max_items: Option<i32>,
    min_items: Option<i32>,
    unique_items: Option<bool>,
    max_properties: Option<i32>,
    min_properties: Option<i32>,
    required: Option<bool>,
    // enum: TODO
    items: Option<Box<Schema>>,
    all_of: Option<Vec<Schema>>,
    properties: Option<BTreeMap<String, Schema>>,
    // addtionalProperties: TODO
    discriminator: Option<String>,
    read_only: Option<bool>,
    // xml: TODO
    external_docs: Option<ExternalDocs>,
    // example: TODO
}

/// The expected response of an operation.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    description: Option<String>,
    #[serde(rename = "type")]
    header_type: String,
    format: Option<String>,
    items: Option<Items>,
    collection_format: Option<String>,
    // default: TODO
    maximum: Option<i32>,
    exclusive_maximum: Option<bool>,
    minimum: Option<i32>,
    exclusive_minimum: Option<bool>,
    max_length: Option<i32>,
    min_length: Option<i32>,
    pattern: Option<String>,
    max_items: Option<i32>,
    min_items: Option<i32>,
    unique_items: Option<bool>,
    // enum: TODO
    multiple_of: Option<i32>,
}

/// Limited subset of JSON-Schema's items object.
#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    #[serde(rename = "type")]
    item_type: String,
    format: Option<String>,
    items: Option<Box<Items>>,
    collection_format: Option<String>,
    // default: TODO
    maximum: Option<i32>,
    exclusive_maximum: Option<bool>,
    minimum: Option<i32>,
    exclusive_minimum: Option<bool>,
    max_length: Option<i32>,
    min_length: Option<i32>,
    pattern: Option<String>,
    max_items: Option<i32>,
    min_items: Option<i32>,
    unique_items: Option<bool>,
    // enum: TODO
    multiple_of: Option<i32>,
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
                    version: "2.0".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })
        )
    }

    #[test]
    fn info_deserialize() {
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
                }),
                ..Default::default()
            })
        )
    }

    #[test]
    fn minimal_paths_deserialize() {
        let path = "examples/minimal_paths.yaml";
        let mut path_map = BTreeMap::new();
        path_map.insert(
            "/campaigns".to_string(),
            PathItem {
                get: Some(Operation {
                    tags: Some(vec![
                        "fetch".to_string(),
                        "data".to_string(),
                        "rest".to_string(),
                    ]),
                    summary: Some("A short summary of this operation".to_string()),
                    description: Some("A short description of this operation".to_string()),
                    external_docs: Some(ExternalDocs {
                        description: Some("External docs desc".to_string()),
                        url: "https://swagger.io".to_string(),
                    }),
                    operation_id: Some("getData".to_string()),
                    consumes: Some(vec![
                        "application/json".to_string(),
                        "text/plain; charset=utf-8".to_string(),
                    ]),
                    produces: Some(vec!["application/json".to_string()]),
                    parameters: Some(vec![Parameter {
                        name: "param1".to_string(),
                        in_loc: "query".to_string(),
                        description: Some("Description of param1".to_string()),
                        required: Some(true),
                        param_type: "string".to_string(),
                        format: Some("email".to_string()),
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );

        assert_eq!(
            deserialize(path),
            Ok(Spec {
                swagger: "2.0".to_string(),
                info: Info {
                    title: "Test Title".to_string(),
                    version: "1.0".to_string(),
                    ..Default::default()
                },
                paths: path_map,
                ..Default::default()
            })
        )
    }

    #[test]
    fn minimal_response_deserialize() {
        let path = "examples/minimal_response.yaml";
        let result = deserialize(path)
            .expect("Failed to deserialize Spec")
            .paths
            .remove("/campaigns")
            .expect("No '/campaigns' path")
            .get
            .expect("No 'get' operation on path")
            .responses
            .remove("200")
            .expect("No '200' reponse on operation");

        let mut headers = BTreeMap::new();
        headers.insert("X-Rate-Limit-Limit".to_string(), Header {
            header_type: "integer".to_string(),
            description: Some("The number of allowed requests in a period".to_string()),
            minimum: Some(0),
            maximum: Some(300),
            ..Default::default()
        });

        assert_eq!(
            result,
            Response {
                description: "Get a list of campaigns".to_string(),
                headers: Some(headers),
                ..Default::default()
            }
        )
    }
}
