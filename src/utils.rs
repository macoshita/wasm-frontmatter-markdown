use lazy_static::lazy_static;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::{dumps::from_binary, highlighting::ThemeSet};

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = from_binary(include_bytes!("./newlines.packdump"));
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    pub static ref EMOJI_REPLACER: gh_emoji::Replacer = gh_emoji::Replacer::new();
}

pub fn separate_frontmatter(text: &str) -> Result<(String, String), String> {
    if !text.starts_with("---\n") {
        return Err("frontmatter not found".into());
    }

    let slice_after_marker = &text[4..];
    let end = slice_after_marker.find("---\n");
    if end.is_none() {
        return Err("frontmatter not found".into());
    }

    let fm_end = end.unwrap();
    let frontmatter = &slice_after_marker[..fm_end];
    let content = &slice_after_marker[fm_end + 4..];
    Ok((frontmatter.into(), content.into()))
}

pub fn yaml_to_json(text: &str) -> Result<serde_json::Value, String> {
    serde_yaml::from_str::<serde_json::Value>(text).map_err(|err| format!("{}", err))
}

pub fn convert_html(markdown: &str) -> String {
    let theme = &THEME_SET.themes["base16-ocean.dark"];

    let mut in_code_block = false;
    let mut syntax = None;
    let mut code = String::new();

    let mut parser = Vec::new();

    for event in Parser::new_ext(markdown, Options::empty()) {
        match event {
            Event::Start(Tag::CodeBlock(ref info)) => {
                in_code_block = true;

                if let CodeBlockKind::Fenced(info) = info {
                    let lang = info.split(' ').next().unwrap();
                    if !lang.is_empty() {
                        syntax = SYNTAX_SET.find_syntax_by_token(lang);
                    }
                }
            }

            Event::Text(t) => {
                if in_code_block {
                    code.push_str(&t);
                } else {
                    let s = EMOJI_REPLACER.replace_all(&t);
                    parser.push(Event::Text(s.to_string().into()));
                }
            }

            Event::End(Tag::CodeBlock(_)) if in_code_block => {
                in_code_block = false;
                let html = highlighted_html_for_string(
                    &code,
                    &SYNTAX_SET,
                    syntax.unwrap_or(SYNTAX_SET.find_syntax_plain_text()),
                    theme,
                );
                parser.push(Event::Html(html.into()));
                code = String::new();
                syntax = None;
            }

            _ => {
                parser.push(event);
            }
        }
    }

    let mut html_output: String = String::with_capacity(markdown.len() * 3 / 2);
    html::push_html(&mut html_output, parser.into_iter());
    html_output
}
