#![warn(missing_docs)]
/*!
# Sap

Sap is a testing library that provides abstractions on top of [`wasm_bindgen`] for testing DOM nodes.

The main feature of this crate is using `queries` to find elements in the DOM and perform actions
that simulate user behaviour to assert that your application behaves correctly.

The recommended query set to use is in the [`by_aria`](crate::queries::by_aria) module - this queries
by ARIA and using this will also help you consider the accessibility of your application.


Requirements:
- [`wasm-bindgen-test`](https://crates.io/crates/wasm-bindgen) in dev-dependencies

All Sap functions are assuming they will be in wasm-bindgen-tests:

```no_run
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test() {
    // .. test code here
}
```

Running wasm-bindgen-tests

Multiple browsers can be used here or just one:
```bash
$ wasm-pack test --headless --firefox --chrome
```
*/

extern crate self as sap;

mod asserts;
pub mod event;
mod iter;
pub mod queries;

pub use iter::*;
pub use queries::QueryElement;

/// Alias for boxed error
pub type Error = Box<dyn std::error::Error>;

#[cfg(test)]
pub(crate) fn make_element_with_html_string(inner_html: &str) -> web_sys::HtmlElement {
    use wasm_bindgen::JsCast;

    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    // remove \n & \t and 4 x spaces which are just formatting to avoid text nodes being added
    let inner_html = inner_html
        .chars()
        .fold((String::new(), 0), |(mut s, ws), c| match c {
            ' ' if ws == 3 => {
                s.truncate(s.len() - 3);
                (s, 0)
            }
            ' ' => {
                s.push(c);
                (s, ws + 1)
            }
            '\n' | '\t' => (s, 0),
            _ => {
                s.push(c);
                (s, 0)
            }
        })
        .0;
    div.set_inner_html(&inner_html);

    document.body().unwrap().append_child(&div).unwrap();
    div.unchecked_into()
}

/// Sap Prelude
///
/// Convenient module to import the most used imports for sap.
///
/// ```no_run
/// use sap::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        assert_inner_text, assert_text_content,
        iter::*,
        queries::{
            by_aria::*, by_display_value::*, by_label_text::*, by_placeholder_text::*, by_text::*,
            QueryElement,
        },
        Error,
    };
    pub use sap_aria::{property::*, role::*, state::*};
}
