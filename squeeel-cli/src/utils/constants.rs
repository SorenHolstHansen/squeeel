pub const JSON_VALUE_TYPE: &str = r#"type JsonValue = string | number | boolean | null | {
    [Key in string]?: JsonValue;
} | JsonValue[];"#;
