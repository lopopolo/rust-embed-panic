#[macro_use]
extern crate rust_embed;

use std::borrow::Cow;

/// Static assets
#[derive(RustEmbed)]
#[folder = "static/"]
struct Source;

fn main() {
    let content = Source::get("main.js")
        .map(Cow::into_owned)
        .map(String::from_utf8)
        .expect("main.js");
    println!("[main.js] {}", content);
}
