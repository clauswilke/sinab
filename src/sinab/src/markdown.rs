extern crate pulldown_cmark;

use super::utils::c_helper::*;
use pulldown_cmark::{html, Options, Parser};

/// External C interface to `md_to_html()`.
#[no_mangle]
pub extern "C" fn sinab_md_to_html(text: *const c_char) -> *mut c_char {
    let html_output = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };

    str_to_cstring(html_output.as_str())
}

/// Convert markdown to html. Uses the `pulldown_cmark` crate.
pub fn md_to_html(input: &str) -> String {
    // set up markdown parser
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(input, options);

    // write parsed input as html
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}