//! Queries for finding [`Element`](web_sys::Element)s.
//!
//! This module helps to query the DOM of a rendered root element. The goal is to use high/medium level
//! APIs so that the DOM can be queried in a manner similar to how a user might navigate the UI.

use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub mod by_aria;
pub mod by_display_value;
pub mod by_label_text;
pub mod by_placeholder_text;
pub mod by_selector;
pub mod by_text;

/// Wrapper around a root element which has been rendered.
pub struct QueryElement(HtmlElement);

impl QueryElement {
    /// Wrap rendered root element ready to be queried.
    ///
    /// # Examples
    /// ```no_run
    /// use hyphae::prelude::*;
    /// let rendered = QueryElement::new();
    /// // .. use `rendered` to get elements and perform tests
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for QueryElement {
    fn default() -> Self {
        let doc = web_sys::window()
            .and_then(|w| w.document())
            .expect("Cannot get global document");
        let div = doc.create_element("div").expect("Unable to create element");
        div.set_id("hyphae-test-app");
        doc.body()
            .expect("Cannot get body element")
            .append_child(&div)
            .expect("Unable to append test div to body");

        Self(div.unchecked_into())
    }
}

impl From<HtmlElement> for QueryElement {
    fn from(root_element: HtmlElement) -> Self {
        Self(root_element)
    }
}

impl Deref for QueryElement {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<HtmlElement> for QueryElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.0
    }
}

// Removing the element is useful to avoid conflicts when a test module has multiple
// #[wasm_bindgen_test]s, however, it does mean that everything is removed from the DOM when a
// user is performing wasm-pack test without --headless.
impl Drop for QueryElement {
    fn drop(&mut self) {
        self.0.remove();
    }
}
