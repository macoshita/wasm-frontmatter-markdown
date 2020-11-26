use std::{fs::File, io::BufWriter, io::Write};

use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle},
};

fn main() -> Result<(), std::io::Error> {
    // generate css
    let css_file = File::create("pkg/highlight.css")?;
    let prefixed_style = ClassStyle::SpacedPrefixed { prefix: "hl-" };
    let mut css_writer = BufWriter::new(&css_file);

    // ---------------------------------------------------------------------------------------------
    // generate css files for themes
    let ts = ThemeSet::load_defaults();

    // create dark color scheme css
    let dark_theme = &ts.themes["base16-ocean.dark"];
    let css_dark = css_for_theme_with_class_style(dark_theme, prefixed_style);
    writeln!(
        css_writer,
        "@media (prefers-color-scheme: dark) {{{}}}",
        css_dark
    )?;

    // create light color scheme css
    let light_theme = &ts.themes["base16-ocean.light"];
    let css_light = css_for_theme_with_class_style(light_theme, prefixed_style);
    writeln!(
        css_writer,
        "@media (prefers-color-scheme: light) {{{}}}",
        css_light
    )?;

    Ok(())
}
