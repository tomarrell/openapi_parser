use std::collections::BTreeMap;

struct Spec {
    swagger: String,
    info: Info,
    host: String,
    basePath: String,
    schemes: Vec<String>,
    consumes: Vec<String>,
    produces: Vec<String>,
    paths: BTreeMap<String, PathItem>,
    definitions: Definitions,
    parameters: Parameters,
    responses: BTreeMap<String, ResponseOrRef>,
    security_definitions: SecurityDefinitions,
    tags: Tags,
    external_docs: ExternalDocs,
}

struct Info {
    title: String,
    description: String,
    terms_of_service: String,
    contact: Contact,
    license: License,
    version: String,
}

struct Contact {
    name: String,
    url: String,
    email: String,
}

struct License {
    name: String,
    url: String,
}

struct PathItem {
    reference: String,
    get: Operation,
    put: Operation,
    post: Operation,
    delete: Operation,
    options: Operation,
    head: Operation,
    patch: Operation,
    parameters: Parameters,
}

struct Operation {
    tags: Vec<String>,
    summary: String,
    description: String,
    external_docs: ExternalDocs,
    operation_id: String,
    consumes: Vec<String>,
    produces: Vec<String>,
    parameters: Parameters,
    responses: BTreeMap<String, ResponseOrRef>,
    schemes: Vec<String>,
    deprecated: bool,
    security: Vec<SecurityRequirement>,
}

struct Response {
    description: String,
    schema: SchemaOrRef,
    headers: Headers,
    examples: Examples,
}

struct Headers {
}

struct Examples {
}

struct Definitions {
}

struct Parameter {
    name: String,
    param_loc: String,
    description: String,
    required: bool,
    schema: Schema,
    param_type: String,
    format: String,
    allow_empty_value: bool,
    items: Items,
    collection_format: String,
    // default: TODO
    maximum: i32,
    exclusive_maximum: bool,
    minimum: bool,
    exclusive_minimum: bool,
    max_length: i32,
    min_length: i32,
    pattern: String,
    max_items: i32,
    min_items: i32,
    unique_items: bool,
    // enum: TODO
    multiple_of: i32,
}

struct Items {
}

struct Schema {
    reference: String,
    format: String,
    title: String,
    description: String,
    // default: TODO
    multiple_of: i32,
    maximum: i32,
    exclusive_maximum: bool,
    minimum: i32,
    exclusive_minimum: bool,
    max_length: i32,
    min_length: i32,
    pattern: String,
    max_items: i32,
    min_items: i32,
    unique_items: i32,
    max_properties: i32,
    min_properties: i32,
    required: bool,
    // enum: TODO
    val_type: String,
    all_of: Vec<SchemaOrRef>
}

struct Reference {
}

struct SecurityDefinitions {
}

struct SecurityRequirement {
}

struct Tags {
}

struct ExternalDocs {
    description: String,
    url: String,
}

enum Parameters {
    Parameter(Parameter),
    Reference(Reference),
}

enum ResponseOrRef {
    reference(String),
    Response(Response),
}

enum SchemaOrRef {
    reference(String),
    Schema(Schema),
}
