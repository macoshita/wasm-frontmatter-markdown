use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::dumps::from_binary;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: Lazy<SyntaxSet> =
    Lazy::new(|| from_binary(include_bytes!("./newlines.packdump")));
static EMOJI_REPLACER: Lazy<gh_emoji::Replacer> = Lazy::new(|| gh_emoji::Replacer::new());

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
    let prefixed_style = ClassStyle::SpacedPrefixed { prefix: "hl-" };
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
                let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                    syntax.unwrap_or(SYNTAX_SET.find_syntax_plain_text()),
                    &SYNTAX_SET,
                    prefixed_style,
                );
                for line in code.lines() {
                    html_generator.parse_html_for_line(&line);
                }
                let html = html_generator.finalize();
                parser.push(Event::Html(
                    format!("<pre class=\"hl-code\">{}</pre>", html).into(),
                ));
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
