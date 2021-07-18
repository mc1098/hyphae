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
use std::{marker::PhantomData, ops::Deref};

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};

mod asserts;
#[doc(inline)]
pub mod events;
#[doc(inline)]
pub mod queries;
mod util;

/// Wrapper around a root element which has been rendered.
pub struct TestRender {
    root_element: HtmlElement,
}

/// Iterator for [`Element`]s
pub struct ElementIter<'a, T: JsCast> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
    _marker: PhantomData<&'a T>,
}

#[allow(dead_code)]
impl<T: JsCast> ElementIter<'_, T> {
    pub(crate) fn new(node_list: Option<NodeList>) -> Self {
        if let Some(node_list) = node_list {
            node_list.into()
        } else {
            Self {
                iter: Box::new(std::iter::empty()),
                _marker: PhantomData,
            }
        }
    }
}

impl<T: JsCast> From<NodeList> for ElementIter<'_, T> {
    fn from(node_list: NodeList) -> Self {
        let mut nodes = vec![];
        for i in 0..node_list.length() {
            if let Some(element) = node_list.get(i).and_then(|node| node.dyn_into().ok()) {
                nodes.push(element);
            }
        }

        Self {
            iter: Box::new(nodes.into_iter()),
            _marker: PhantomData,
        }
    }
}

impl<T: JsCast> Iterator for ElementIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Iterator for [`NodeList`]
pub(crate) struct RawNodeListIter<T> {
    index: u32,
    node_list: Option<NodeList>,
    _marker: PhantomData<T>,
}

impl<T> RawNodeListIter<T>
where
    T: JsCast,
{
    fn new(node_list: Option<NodeList>) -> Self {
        Self {
            index: 0,
            node_list,
            _marker: PhantomData,
        }
    }
}

impl<T> Iterator for RawNodeListIter<T>
where
    T: JsCast,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_list = self.node_list.as_ref()?;

        if self.index < node_list.length() {
            let node = node_list.get(self.index)?;
            self.index += 1;
            node.dyn_into().ok()
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            self.node_list.as_ref().map(|list| list.length() as usize),
        )
    }
}

impl TestRender {
    /**
    Wrap rendered root element ready to be queried.

    # Examples
    ```no_run
    use sap::prelude::*;
    # use web_sys::Element;
    # fn render(element: Element) {
    let rendered = TestRender::new(element);
    // .. use `rendered` to get elements and perform tests
    # }
    ```
    */
    pub fn new(root_element: Element) -> Self {
        Self {
            root_element: root_element.unchecked_into(),
        }
    }
}

impl From<HtmlElement> for TestRender {
    fn from(root_element: HtmlElement) -> Self {
        Self { root_element }
    }
}

impl Deref for TestRender {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.root_element
    }
}

/// Sap Prelude
///
/// Convenient module to import the most used imports for yew_test.
///
/// ```no_run
/// use sap::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        assert_text_content,
        queries::{
            by_aria::*, by_display_value::*, by_label_text::*, by_placeholder_text::*, by_text::*,
        },
        TestRender,
    };
    pub use sap_aria::*;
    pub use web_sys::{Element, HtmlElement, Node};
}
