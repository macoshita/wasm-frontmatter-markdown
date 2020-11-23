mod utils;

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct ParseOptions {
    frontmatter: Option<bool>,
    content: Option<bool>,
}

#[wasm_bindgen]
pub fn parse(text: &str, options: &JsValue) -> Result<JsValue, JsValue> {
    let options: ParseOptions = options.into_serde().unwrap_or(ParseOptions {
        frontmatter: Some(false),
        content: Some(false),
    });

    let (frontmatter, content) = utils::separate_frontmatter(text)?;

    let frontmatter_json = if options.frontmatter.unwrap_or(false) {
        Some(utils::yaml_to_json(&frontmatter)?)
    } else {
        None
    };

    let content_html = if options.content.unwrap_or(false) {
        Some(utils::convert_html(&content))
    } else {
        None
    };

    let json = json!({
        "frontmatter": frontmatter_json,
        "content": content_html,
    });

    Ok(JsValue::from_serde(&json).unwrap())
}
