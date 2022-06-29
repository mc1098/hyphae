//! Supports finding elements generically by a selector string.
//!
//! A selector string can contain multiple ways to identify a single or multiple
//! elements in the DOM. The basics are:
//!
//! - Type selectors like a tag name for an element e.g. "input"
//! - Class selectors using the fullstop before a class name e.g. ".classname"
//! - ID selector using the hash before an id e.g. "#idname"
//!
//! There are more selectors and ways of combining selectors which are all based
//! off [CSS selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors).
//!
//! # Generics
//!
//! Each trait function supports generics for convenience and to help narrow the
//! scope of the search. if you are querying for a `HtmlButtonElement` then you
//! won't find a `HtmlDivElement` and vice versa.
//!
//! Unfortunately, `hyphae` doesn't support using generics alone to find
//! elements by a specific tag name - this would reduce the need to type the
//! "input" selector and use the `HtmlInputElement` in conjunction but the
//! tag names are not available on the `web_sys` types without having an
//! instance of that type.
//!
//! In `hyphae` the `HtmlElement` can be used as a "catch all" generic
//! type[^note].
//!
//! [^note]: _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
//! type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
//! or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._
//!
//! # What is `JsCast`?
//!
//! The generic type returned needs to impl `JsCast` which is a trait from
//! `wasm_bindgen` crate for performing checked and unchecked casting between JS
//! types.
use std::fmt::{Debug, Display};

use wasm_bindgen::JsCast;

use hyphae::{ElementIter, Error, QueryElement};
use web_sys::HtmlElement;

/// Enables queries by selector.
/// _See each trait function for examples_
pub trait BySelector {
    /// Get the first generic element found using the selector string.
    ///
    /// Using a specific generic type as `T` will essentially skip the
    /// other types of elements - if you want to find the very first element
    /// that matches the selector string then use `HtmlElement`.
    ///
    /// # Panics
    /// _Nothing to see here_
    ///
    /// # Examples
    /// Rendered html:
    /// ```html
    /// <div id="div-1">
    ///     <div id="div-2" class="myclass"></div>
    ///     <input id="input-1" />
    ///     <input id="input-2" class="myclass" />
    ///     <input id="input-3" />
    /// </div>
    /// ```
    ///
    /// ## Get first input by type selector "input":
    /// The first input ("input-1") is the first `HtmlInputElement` in the DOM
    /// so the query will short circuit once this element is found.
    ///
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlInputElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_input_by_type() {
    ///     let rendered: QueryElement = // feature dependent rendering
    ///     # QueryElement::new();
    ///     let input: HtmlInputElement = rendered
    ///         .get_first_by_selector("input")
    ///         .unwrap();
    ///
    ///     assert_eq!("input-1", input.id());
    /// }
    /// ```
    ///
    /// ## Get second input by class selector ".myclass":
    /// The second input ("input-2") is the only `HtmlInputElement` with the
    /// class value of "myclass" and the `HtmlDivElement`s are skipped because
    /// they don't match the `T` returned.
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlInputElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_input_by_class() {
    ///     let rendered: QueryElement = // feature dependent rendering
    ///     # QueryElement::new();
    ///     let mut input: HtmlInputElement = rendered
    ///         .get_first_by_selector(".myclass")
    ///         .unwrap();
    ///
    ///     assert_eq!("input-2", input.id());
    /// }
    /// ```
    ///
    /// ## Get third input by id selector "#input-3":
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlInputElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_input_by_id() {
    ///     let rendered: QueryElement = // feature dependent rendering
    ///     # QueryElement::new();
    ///     let mut input: HtmlInputElement = rendered
    ///         .get_first_by_selector("#input-3")
    ///         .unwrap();
    ///
    ///     assert_eq!("input-3", input.id());
    /// }
    /// ```
    fn get_first_by_selector<T>(&self, selector: &str) -> Result<T, Error>
    where
        T: JsCast;

    /// A convenient method which unwraps the result of `get_first_by_selector`.
    fn assert_first_by_selector<T>(&self, selector: &str) -> T
    where
        T: JsCast,
    {
        self.get_first_by_selector(selector).unwrap()
    }

    /// Get all the generic elements found using the selector string.
    ///
    /// Using a specific generic type as `T` will essentially skip the
    /// other types of elements - if you want to find all the elements
    /// that matches the selector string then use `HtmlElement`.
    ///
    /// # Panics
    /// _Nothing to see here_
    ///
    /// # Examples
    /// Rendered html:
    /// ```html
    /// <div id="div-1">
    ///     <div id="div-2" class="myclass"></div>
    ///     <input id="input-1" />
    ///     <input id="input-2" class="myclass" />
    ///     <input id="input-3" />
    /// </div>
    /// ```
    ///
    /// ## Get all inputs by type selector "input":
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlInputElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_inputs_by_type() {
    ///     let rendered: QueryElement = // feature dependent rendering
    ///     # QueryElement::new();
    ///     let mut iter = rendered
    ///         .get_all_by_selector::<HtmlInputElement>("input")
    ///         .unwrap();
    ///
    ///     assert_eq!("input-1", iter.next().unwrap().id());
    ///     assert_eq!("input-2", iter.next().unwrap().id());
    ///     assert_eq!("input-3", iter.next().unwrap().id());
    ///     assert!(iter.next().is_none());
    /// }
    /// ```
    ///
    /// ## Get all elements by class selector ".myclass":
    /// The second div ("div-2") and input ("input-2") are the only elements with the
    /// class value of "myclass".
    /// ```no_run
    /// # fn main() {}
    /// use wasm_bindgen_test::*;
    /// wasm_bindgen_test_configure!(run_in_browser);
    /// use hyphae::prelude::*;
    /// use web_sys::HtmlInputElement;
    ///
    /// #[wasm_bindgen_test]
    /// fn get_input_by_class() {
    ///     let rendered: QueryElement = // feature dependent rendering
    ///     # QueryElement::new();
    ///     let mut iter = rendered
    ///         .get_all_by_selector::<HtmlInputElement>(".myclass")
    ///         .unwrap();
    ///
    ///     assert_eq!("div-2", iter.next().unwrap().id());
    ///     assert_eq!("input-2", iter.next().unwrap().id());
    ///     assert!(iter.next().is_none());
    /// }
    /// ```
    ///
    /// If you are using an ID selector then you really are only looking for one
    /// element so consider using `get_first_by_selector`.
    fn get_all_by_selector<T>(&self, selector: &str) -> Result<ElementIter<T>, Error>
    where
        T: JsCast;

    /// A convenient method which unwraps the result of `get_all_by_selector`.
    fn assert_all_by_selector<T>(&self, selector: &str) -> ElementIter<T>
    where
        T: JsCast;
}

impl BySelector for QueryElement {
    fn get_first_by_selector<T>(&self, selector: &str) -> Result<T, Error>
    where
        T: JsCast,
    {
        // we need to use selector all as we want to not just the first
        // result of the selector but the first one that matches for the
        // generic T.
        if let Ok(element) = self
            .get_all_by_selector(selector)
            .map(|mut iter| iter.next().unwrap())
        {
            Ok(element)
        } else {
            let closest = self.get_first_by_selector::<HtmlElement>(selector)?;
            Err(Box::new(BySelectorError::Closest {
                selector: selector.to_owned(),
                inner_html: self.inner_html(),
                closest_element: closest,
            }))
        }
    }

    fn get_all_by_selector<T>(&self, selector: &str) -> Result<ElementIter<T>, Error>
    where
        T: JsCast,
    {
        let elements = self
            .query_selector_all(selector)
            .map(ElementIter::from)
            .map_err(|_| BySelectorError::SyntaxError(selector.to_owned()))?;
        if let (_, Some(0)) = elements.size_hint() {
            Err(BySelectorError::NoElementFound(selector.to_owned()).into())
        } else {
            Ok(elements)
        }
    }

    fn assert_all_by_selector<T>(&self, selector: &str) -> ElementIter<T>
    where
        T: JsCast,
    {
        let result = self.get_all_by_selector(selector);
        if result.is_err() {
            self.remove();
        }
        result.unwrap()
    }
}

enum BySelectorError {
    Closest {
        selector: String,
        inner_html: String,
        closest_element: HtmlElement,
    },
    NoElementFound(String),
    SyntaxError(String),
}

impl Debug for BySelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closest {
                selector,
                inner_html,
                closest_element,
            } => {
                write!(f,
                    "\nNo exact match found for '{selector}'.\nA similar match was found in the following HTML:{}",
                    hyphae_utils::format_html_with_closest(inner_html, closest_element)
                )
            }
            Self::NoElementFound(selector) => {
                write!(
                    f,
                    "\nNo element found that matches the given selector of '{selector}'."
                )
            }
            Self::SyntaxError(selector) => {
                write!(f, "\nSelector string of '{selector}' syntax is not valid!")
            }
        }
    }
}

impl Display for BySelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for BySelectorError {}

#[cfg(test)]
mod tests {
    use super::*;

    use hyphae_utils::make_element_with_html_string;
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn assert_first_input_finds_only_input() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div>
                <section>
                    <div>
                        <input value="my input" />
                    </div>
                </section>
            </div>
        "#,
        )
        .into();

        let input: HtmlInputElement = rendered.assert_first_by_selector("input");

        assert_eq!("my input", input.value());
    }

    #[wasm_bindgen_test]
    fn assert_first_class_finds_first_button_with_class_skipping_other_elements() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div id="div-1" class="classname">
                <section class="classname">
                    <input class="classname" />
                    <button id="button-1">Click Me</button>
                    <button id="button-2" class="classname">Click Me</button>
                    <div class="classname"></div>
                </section>
            </div>
            "#,
        )
        .into();

        let button: HtmlButtonElement = rendered.assert_first_by_selector(".classname");
        // skip the div, section, input elements because of the generic type
        // choosen, also skip the first button because it doesn't have the
        // correct class
        assert_eq!("button-2", button.id());

        let element: HtmlElement = rendered.assert_first_by_selector(".classname");

        // HtmlElement is a catch all so we will find the very first element
        // that matches the selector, the first element is the div.
        assert_eq!("div-1", element.id());
    }

    #[wasm_bindgen_test]
    fn get_all_input_elements() {
        let rendered: QueryElement = make_element_with_html_string(
            // just add id and class of "input" to show we don't catch those
            // while looking for a element by name
            r#"
            <div id="input" class="input">
                <input id="input-1" />
                <input id="input-2" />
                <input id="input-3" />
                <input id="input-4" />
            </div>
            "#,
        )
        .into();

        let mut iter = rendered.assert_all_by_selector::<HtmlInputElement>("input");

        assert_eq!("input-1", iter.next().unwrap().id());
        assert_eq!("input-2", iter.next().unwrap().id());
        assert_eq!("input-3", iter.next().unwrap().id());
        assert_eq!("input-4", iter.next().unwrap().id());
        assert!(iter.next().is_none());
    }

    #[wasm_bindgen_test]
    fn no_element_found_error_when_selector_does_not_match() {
        let rendered: QueryElement = make_element_with_html_string("<button></button>").into();

        let result = rendered.get_all_by_selector::<HtmlElement>("div");

        match result {
            Ok(_) => panic!("selector 'div' should not match for a button element"),
            Err(error) => {
                let expected = "\nNo element found that matches the given selector of 'div'.";
                assert_eq!(expected, format!("{error:?}"));
            }
        }
    }

    #[wasm_bindgen_test]
    fn t() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
                <input type="text" class="myclass" />
            "#,
        )
        .into();

        let result = rendered.get_first_by_selector::<HtmlButtonElement>(".myclass");

        match result {
            Ok(_) => panic!("input element shouldn't have matched the button element generic!"),
            Err(error) => {
                let expected = format!("\nNo exact match found for '.myclass'.\nA similar match was found in the following HTML:{}",
                    r#"
<input type="text" class="myclass">
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Did you mean to find this element?
"#
                );
                assert_eq!(expected, format!("{error:?}"));
            }
        }
    }

    #[wasm_bindgen_test]
    fn syntax_error_when_selector_is_not_valid() {
        let rendered: QueryElement = make_element_with_html_string("<button></button>").into();

        let result = rendered.get_all_by_selector::<HtmlElement>("@@@");

        match result {
            Ok(_) => panic!("invalid selector should not match anything!"),
            Err(error) => {
                let expected = "\nSelector string of '@@@' syntax is not valid!";
                assert_eq!(expected, format!("{error:?}"));
            }
        }
    }
}
