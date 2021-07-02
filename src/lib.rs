#![warn(missing_docs)]

/*!
Sap

Provides helper functions and traits to help test Yew components.

Requirements:
- [`wasm-bindgen-test`](wasm_bindgen_test) in dev-dependencies

All Sap functions are assuming they will be in wasm-bindgen-tests:

```no_run
use wasm_bindgen_test::*;
wasm_bindgen_test_configuration!(run_in_browser);

#[wasm_bindgen_test]
fn test() {
    // .. test code here
}
```

Running wasm-bindgen-tests

Multiple browsers can be used here or just one:
```
$ wasm-pack test --headless --firefox --chrome
```
*/

use std::{marker::PhantomData, ops::Deref};

use wasm_bindgen::JsCast;
use web_sys::{Element, NodeList};

mod asserts;
#[doc(inline)]
pub mod events;
#[doc(inline)]
pub mod queries;
mod utils;

/// Wrapper around a root element which has been rendered.
pub struct TestRender {
    root_element: Element,
}

/// Iterator for [`Element`]s
pub struct ElementIter<'a, T: JsCast> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
    _marker: PhantomData<&'a T>,
}

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
    use yew::virtual_dom::shallow_render;
    use sap::prelude::*;
    // Counter component implementation..
    let rendered = TestRender::new(test_render(html!(<Counter />)));
    // .. use `rendered` to get elements and perform tests
    ```
    */
    pub fn new(root_element: Element) -> Self {
        Self { root_element }
    }

    /**
    Get an Element by the id, if no element with that id exists then returns [`None`].

    This is a low level query and users can't see an element id - it is recommended to use
    a by_* query instead.

    # Examples
    ```no_run
    use yew::virtual_dom::shallow_render;
    use sap::prelude::*;

    let rendered = shallow_render(html! {
        <div id="mydiv" />
    })
    .into();

    let div: HtmlElement = rendered.get_by_id("mydiv").expect("only element has id of mydiv!");
    assert_eq!("mydiv", div.id());
    ```
    */
    pub fn get_by_id<T>(&self, id: &str) -> Option<T>
    where
        T: JsCast,
    {
        self.root_element
            .query_selector(&format!("#{}", id))
            .ok()
            .flatten()
            .and_then(|e| e.dyn_into().ok())
    }

    /**
    Get all Elements by the class value.

    This is a low level query and users can't see what class an element has - it is recommended to
    use a by_* query instead.

    # Examples
    ```no_run
    use yew::virtual_dom::shallow_render;
    use sap::prelude::*;

    let rendered = shallow_render(html! {
        <div class="divclass" />
    })
    .into();

    let iter = rendered.get_by_class("divclass");
    let div: HtmlElement = iter.next().expect("should be one element in this iterator!");
    assert!(iter.next().is_none());
    ```
    */
    pub fn query_by_class<T>(&'_ self, class: &str) -> impl Iterator<Item = T>
    where
        T: JsCast,
    {
        RawNodeListIter::new(
            self.root_element
                .query_selector_all(&format!(".{}", class))
                .ok(),
        )
    }
}

impl From<Element> for TestRender {
    fn from(root_element: Element) -> Self {
        Self { root_element }
    }
}

impl Deref for TestRender {
    type Target = Element;

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
    pub use crate::{assert_text_content, queries::*, TestRender};
    pub use web_sys::{Element, HtmlElement, Node};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use web_sys::HtmlButtonElement;
    use yew::prelude::*;
    use yew::virtual_dom::test_render;

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    #[wasm_bindgen_test]
    fn get_div_by_id() {
        let rendered: TestRender = test_render(html! {
            <div id="mydiv"/>
        })
        .into();

        let _: HtmlElement = rendered
            .get_by_id("mydiv")
            .expect("div with id=`mydiv` is the only element!");

        let nothing = rendered.get_by_id::<Element>("this id does not exist!");
        assert!(nothing.is_none())
    }

    #[wasm_bindgen_test]
    fn get_btn_by_class() {
        let rendered: TestRender = test_render(html! {
            <>
                <button class="super-btn" />
                <div class="super-btn" />
            </>
        })
        .into();

        let mut super_btns = rendered.query_by_class::<HtmlElement>("super-btn");
        let _button: HtmlButtonElement = super_btns.next().and_then(|e| e.dyn_into().ok()).unwrap();
        let _div: HtmlElement = super_btns.next().unwrap();
        assert!(super_btns.next().is_none());
    }
}
