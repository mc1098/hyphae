#![warn(missing_docs)]
/*!
# hyphae

hyphae is a testing library that provides abstractions on top of [`wasm_bindgen`] for testing DOM nodes.

The main feature of this crate is using `queries` to find elements in the DOM and perform actions
that simulate user behaviour to assert that your application behaves correctly.

The recommended query set to use is in the [`by_aria`](crate::queries::by_aria) module - this queries
by ARIA and using this will also help you consider the accessibility of your application.


Requirements:
- [`wasm-bindgen-test`](https://crates.io/crates/wasm-bindgen) in dev-dependencies

All hyphae functions are assuming they will be in wasm-bindgen-tests:

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

extern crate self as hyphae;

mod asserts;
pub mod event;
mod iter;
pub mod queries;

/// Utility functions.
pub mod utils {
    pub use hyphae_utils::{effect_dom, wait_ms};
}

pub use iter::*;
pub use queries::QueryElement;

/// Alias for boxed error
pub type Error = Box<dyn std::error::Error>;

/// hyphae Prelude
///
/// Convenient module to import the most used imports for hyphae.
///
/// ```no_run
/// use hyphae::prelude::*;
/// ```
pub mod prelude {
    pub use hyphae::{
        assert_inner_text, assert_text_content,
        iter::*,
        queries::{
            by_aria::*, by_display_value::*, by_label_text::*, by_placeholder_text::*, by_text::*,
            QueryElement,
        },
        Error,
    };
    pub use hyphae_aria::{property::*, role::*, state::*};
}
