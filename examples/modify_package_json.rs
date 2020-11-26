use std::{
    fs::File,
    io::BufReader,
    io::{BufWriter, Write},
};

use serde_json::Value;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("pkg/package.json")?;
    let reader = BufReader::new(&file);
    let mut json: Value = serde_json::from_reader(reader)?;
    if !json["homepage"].is_string() {
        json["homepage"] = "https://github.com/macoshita/wasm-frontmatter-markdown".into();
    }

    let css = "highlight.css";
    if !json["files"]
        .as_array()
        .unwrap()
        .contains(&Value::from(css))
    {
        json["files"].as_array_mut().unwrap().push(css.into());
    }
    let file = File::create("pkg/package.json")?;
    let mut writer = BufWriter::new(&file);
    write!(writer, "{}", json.to_string())?;
    Ok(())
}
