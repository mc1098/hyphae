//! Supports finding elements generically by their inner text.
//!
//! The text of an element is the text raw text between the opening and closing tags.
//!
//! ```html
//! <div id="1">
//!     div text node
//!     <button id="2">button text node</button>
//! </div>
//! ```
//! The elements will have the following inner text:
//! 1 - "div text nodebutton text node"
//! 2 - "button text node"
//!
//! # Generics
//! Each trait function supports generics for convenience and to help narrow the scope of the search. If
//! you are querying for a [`HtmlButtonElement`](web_sys::HtmlInputElement) then you won't find a
//! [`HtmlDivElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDivElement.html)
//! and vice versa.
//!
//! In [`hyphae`](crate) the [`HtmlElement`](web_sys::HtmlElement) can be used as a "catch all" generic
//! type[^note].
//!
//! [^note]: _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
//! type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
//! or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._
//!
//! # What is [`JsCast`]?
//!
//! The generic type returned needs to impl [`JsCast`] which is a trait from [`wasm_bindgen`] crate for
//! performing checked and unchecked casting between JS types.
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use hyphae::{Error, QueryElement};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, Node, NodeFilter, TreeWalker};

/// Enables queries by inner text.
///
/// _See each trait function for examples._
pub trait ByText {
    /// Get a generic element by the inner text.
    ///
    /// Using one of the generic types above as `T` will essentially skip the other two types of
    /// elements - if you want to find the very first element that matches the display value then use
    /// [`HtmlElement`](web_sys::HtmlElement).
    ///
    /// # Panics
    /// _Nothing to see here_
    ///
    /// # Examples
    ///
    /// Rendered html:
    /// ```html
    /// <div>
    /// <div id="text-div">Hello, World!</div>
    /// <button id="text-button">Hello, World!</button>
    /// <label id="text-label">Hello, World!</label>
    /// </div>
    /// ```
    ///
    /// ## Get button by text:
    ///
    /// The button is the second element with the correct text node and will be returned due to
    /// `T` being [`HtmlButtonElement`](web_sys::HtmlButtonElement).
    ///
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlButtonElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_button_by_text() {
    /// let rendered: QueryElement = // feature dependent rendering
    /// # QueryElement::new();
    /// let button: HtmlButtonElement = rendered
    /// .get_by_text("Hello, World!")
    /// .unwrap();
    ///
    /// assert_eq!("text-button", button.id());
    /// }
    /// ```
    ///
    /// ## Get label by text:
    ///
    /// The label is the last element with the correct text node and will be returned due to `T` being
    /// [`HtmlLabelElement`](web_sys::HtmlLabelElement).
    ///
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlLabelElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_label_by_text() {
    /// let rendered: QueryElement = // feature dependent rendering
    /// # QueryElement::new();
    /// let label: HtmlLabelElement = rendered
    /// .get_by_text("Hello, World!")
    /// .unwrap();
    ///
    /// assert_eq!("text-label", label.id());
    /// }
    /// ```
    ///
    /// ## Get first element by text:
    ///
    /// The inner div element is the first element with the correct text node and will be returned due
    /// to `T` being [`HtmlElement`](web_sys::HtmlElement)[^note].
    ///
    /// [^note]: _`T` could be
    /// [`HtmlDivElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDivElement.html)
    /// to be even more restrictive, however, it is not required in this case._
    ///
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_first_element_by_text() {
    /// let rendered: QueryElement = // feature dependent rendering
    /// # QueryElement::new();
    /// let element: HtmlElement = rendered
    /// .get_by_text("Hello, World!")
    /// .unwrap();
    ///
    /// assert_eq!("text-div", element.id());
    /// }
    /// ```
    fn get_by_text<T>(&self, search: &str) -> Result<T, Error>
    where
        T: JsCast;

    /// A convenient method which unwraps the result of [`get_by_text`](ByText::get_by_text).
    fn assert_by_text<T>(&self, search: &str) -> T
    where
        T: JsCast;
}

fn first_text_node_in_inner_text_match<T>(node: &Node, query: &str, exact: bool) -> Option<T>
where
    T: JsCast,
{
    let check = if exact {
        str::eq
    } else {
        hyphae_utils::is_close
    };
    let mut node = node.clone();
    while let Some(parent) = node
        .parent_element()
        .map(|e| e.unchecked_into::<HtmlElement>())
    {
        let inner_text = parent.inner_text();
        match inner_text.len().cmp(&query.len()) {
            std::cmp::Ordering::Less if check(&query[..inner_text.len()], &inner_text) => {
                node = parent.unchecked_into();
            }
            std::cmp::Ordering::Equal if check(query, &inner_text) => {
                return parent.dyn_into().ok();
            }
            // we only want to check this when checking for close matches
            std::cmp::Ordering::Greater if !exact && check(&inner_text[..query.len()], query) => {
                return parent.dyn_into().ok()
            }
            _ => break,
        }
    }
    None
}

impl ByText for QueryElement {
    #[inline]
    fn assert_by_text<T>(&self, search: &str) -> T
    where
        T: JsCast,
    {
        let result = self.get_by_text(search);
        if result.is_err() {
            self.remove();
        }
        result.unwrap()
    }

    fn get_by_text<T>(&self, search: &str) -> Result<T, Error>
    where
        T: JsCast,
    {
        let create_filter = |search: &str, exact| {
            let search = search.to_owned();
            move |node| first_text_node_in_inner_text_match::<T>(&node, &search, exact).is_some()
        };

        let walker =
            create_filtered_tree_walker(self, WhatToShow::ShowText, create_filter(search, true));

        if let Some(result) = walker
            .next_node()
            .unwrap()
            .and_then(|node| first_text_node_in_inner_text_match::<T>(&node, search, true))
        {
            Ok(result)
        } else {
            // nothing found - lets go back over each text node and find 'close' matches
            let walker = create_filtered_tree_walker(
                self,
                WhatToShow::ShowText,
                create_filter(search, false),
            );

            let iter =
                std::iter::from_fn(move || walker.next_node().ok().flatten()).filter_map(|node| {
                    first_text_node_in_inner_text_match::<T>(&node, search, false).map(|e| {
                        let element = e.unchecked_into::<HtmlElement>();
                        (element.inner_text(), element)
                    })
                });

            if let Some(closest) = hyphae_utils::closest(search, iter, |(key, _)| key) {
                Err(Box::new(ByTextError::Closest {
                    search_term: search.to_owned(),
                    inner_html: self.inner_html(),
                    closest_element: closest.1,
                }))
            } else {
                Err(Box::new(ByTextError::NotFound {
                    search_term: search.to_owned(),
                    inner_html: self.inner_html(),
                }))
            }
        }
    }
}

/// An error indicating that no inner text was an equal match for a given search term.
enum ByTextError {
    /// No inner text could be found with the given search term.
    NotFound {
        search_term: String,
        inner_html: String,
    },
    /// No inner text with an exact match for the search term could be found, however, a inner text
    /// with a similar content as the search term was found.
    ///
    /// This should help find elements when a user has made a typo in either the test or the
    /// implementation being tested or when trying to find text with a dynamic number that may be
    /// incorrect
    Closest {
        search_term: String,
        inner_html: String,
        closest_element: HtmlElement,
    },
}

impl Debug for ByTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByTextError::NotFound {
                search_term,
                inner_html,
            } => {
                write!(
                    f,
                    "\nNo text node found with text equal or similar to '{}' in the following HTML:{}",
                    search_term,
                    hyphae_utils::format_html(inner_html),
                )
            }
            ByTextError::Closest {
                search_term,
                inner_html,
                closest_element,
            } => {
                let html = hyphae_utils::format_html_with_closest(
                    inner_html,
                    closest_element.unchecked_ref(),
                );
                write!(
                    f,
                    "\nNo exact match found for the text: '{}'.\nA similar match was found in the following HTML:{}",
                    search_term,
                    html,
                )
            }
        }
    }
}

impl Display for ByTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for ByTextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[non_exhaustive]
enum WhatToShow {
    ShowText,
}

impl From<WhatToShow> for u32 {
    fn from(show: WhatToShow) -> Self {
        match show {
            WhatToShow::ShowText => 4,
        }
    }
}

struct FilteredTreeWalker {
    walker: TreeWalker,
    _filter_cb: Closure<dyn Fn(Node) -> bool>,
}

impl Deref for FilteredTreeWalker {
    type Target = TreeWalker;

    fn deref(&self) -> &Self::Target {
        &self.walker
    }
}

fn create_filtered_tree_walker<F>(
    root: &Node,
    what_to_show: WhatToShow,
    filter: F,
) -> FilteredTreeWalker
where
    F: Fn(Node) -> bool + 'static,
{
    let mut node_filter = NodeFilter::new();
    let cb = Closure::wrap(Box::new(filter) as Box<dyn Fn(Node) -> bool>);
    node_filter.accept_node(cb.as_ref().unchecked_ref());
    let document = web_sys::Document::new().expect("No global 'document' object!");
    let walker = document
        .create_tree_walker_with_what_to_show_and_filter(
            root,
            what_to_show.into(),
            Some(&node_filter),
        )
        .expect("Unable to create a TreeWalker object!");

    FilteredTreeWalker {
        walker,
        _filter_cb: cb,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use hyphae_utils::make_element_with_html_string;

    use web_sys::{Element, HtmlButtonElement, HtmlLabelElement};

    #[wasm_bindgen_test]
    fn traverse_the_element_tree_to_find_text() {
        let rendered: QueryElement = make_element_with_html_string(
            "
            <div>
                <strong>1</strong>
                <span> item left</span>
            </div>
        ",
        )
        .into();

        rendered.assert_by_text::<Element>("1 item left");
    }

    #[wasm_bindgen_test]
    fn search_multi_text_node_element() {
        let rendered: QueryElement =
            make_element_with_html_string("<div id=\"mydiv\">One: </div>").into();
        let document = web_sys::window()
            .expect("No global window object")
            .document()
            .expect("No global document object");

        let div = document.get_element_by_id("mydiv").unwrap();
        let text = document.create_text_node("Two");
        div.append_child(&text).expect("Unable to append text node");

        rendered.assert_by_text::<Element>("One: Two");
    }

    #[wasm_bindgen_test]
    fn text_search() {
        let test: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                <div>Hello, World!</div>
            </div>
        """#,
        )
        .into();

        let result = test.get_by_text::<Element>("Hello, World!");
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn search_for_text_narrow_with_generics() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                <div id="div">Hello!</div>
                <label id="label">Hello!</label>
                <button id="button">Hello!</button>
            </div>
        """#,
        )
        .into();

        let button: HtmlButtonElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("button", button.id());

        let div: Element = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("div", div.id());

        let label: HtmlLabelElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("label", label.id());
    }

    #[wasm_bindgen_test]
    fn by_text_uses_inner_text_not_text_content() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                Hello, 
                <strong>World!</strong>
            </div>
        """#,
        )
        .into();
        // can't find `Hello, World!` as they are two distinct text nodes :(
        let not_found = rendered.get_by_text::<Element>("Hello, World!");
        assert!(not_found.is_ok());
    }

    #[wasm_bindgen_test]
    fn find_close_match() {
        let rendered: QueryElement =
            make_element_with_html_string("<button>Click me!</button>").into();

        let result = rendered.get_by_text::<HtmlButtonElement>("Click me");

        match result {
            Ok(_) => panic!("Should not have found the button as the text is not an exact match!"),
            Err(error) => {
                let expected = format!(
                    "\nNo exact match found for the text: '{}'.\nA similar match was found in the following HTML:{}",
                    "Click me",
                    r#"
<button>Click me!</button>
^^^^^^^^^^^^^^^^^^^^^^^^^^ Did you mean to find this element?
"#
                );

                assert_eq!(expected, format!("{:?}", error));
            }
        }

        drop(rendered);

        let rendered: QueryElement = make_element_with_html_string("<div>Click me!</div>").into();

        let result = rendered.get_by_text::<HtmlButtonElement>("Click me");

        match result {
            Ok(_) => panic!("Should not have found the div as the text is not a match and the generic type is too restrictive"),
            Err(err) => {
                let expected = format!(
                    "\nNo text node found with text equal or similar to '{}' in the following HTML:{}",
                    "Click me",
                    r#"
<div>Click me!</div>
"#
                );
                assert_eq!(expected, format!("{:?}", err));
            }
        }
    }
}
