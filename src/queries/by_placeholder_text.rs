/*!
Supports finding: [`HtmlInputElement`] or [`HtmlTextAreaElement`] generically by `placeholder text`.

Using this trait to assert that a `placeholder text` is correct is valid, however, placeholders are
**not** accessible - consider whether you can find your element by the following (in the order given):
1. [`get_by_aria_role`](crate::queries::by_aria::ByAria::get_by_aria_role)
2. [`get_by_display_value`](super::by_display_value::ByDisplayValue::get_by_display_value)


# Placeholder Text

The `placeholder text` for each element:

- [`HtmlInputElement`]\:
```html
<input type="email" placeholder="example@domain.com" />
                                 ^^^^^^^^^^^^^^^^^^ the placeholder value
```

- [`HtmlTextAreaElement`]\:
```html
<textarea placeholder="Enter you bio here!" />
                       ^^^^^^^^^^^^^^^^^^^ the placeholder value
```

# Generics
Each trait function supports generics for convenience and to help narrow the scope of the search. If
you are querying for a [`HtmlInputElement`](web_sys::HtmlInputElement) then you won't find a
[`HtmlTextAreaElement`] and vice versa.

In [`Sap`](crate) the [`HtmlElement`](web_sys::HtmlElement) can be used as a "catch all" generic
type[^note].

[^note] _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._

# What is [`JsCast`]?

The generic type returned needs to impl [`JsCast`] which is a trait from [`wasm_bindgen`] crate for
performing checked and unchecked casting between JS types.
*/
use std::fmt::{Debug, Display};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, Node};

use crate::{Error, QueryElement, RawNodeListIter};

/**
Enables querying by `placeholder text`.

_See each trait function for examples._
 */
pub trait ByPlaceholderText {
    /**
    Get a generic element by the placeholder text.

    The possible elements that can be returned are:
    - [`HtmlInputElement`]
    - [`HtmlTextAreaElement`]

    Using one of the generic types above as `T` will essentially skip the other two types of
    elements - if you want to find the very first element that matches the display value then use
    [`HtmlElement`](web_sys::HtmlElement).

    # Panics
    _Nothing to see here._

    # Examples

    Rendered html:
    ```html
    <div>
        <input id="username-input" placeholder="Username" />
        <textarea id="username-textarea" placeholder="Username" />
    </div>
    ```
    ## Get input by placeholder text

    The first element with a placeholder value matching is the input and would have been found
    even if `T` was [`HtmlElement`](web_sys::HtmlElement).

    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_input_by_placeholder_text() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("Username")
            .unwrap();

        assert_eq!("username-input", input.id());
    }
    ```

    ## Get textarea by placeholder text

    The first input element is skipped even though it has the correct placeholder text due to
    `T` being [`HtmlTextAreaElement`].

    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlTextAreaElement;

    #[wasm_bindgen_test]
    fn get_text_area_by_placeholder_text() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let text_area: HtmlTextAreaElement = rendered
            .get_by_placeholder_text("Username")
            .unwrap();

        assert_eq!("username-textarea", text_area.id());
    }
    ```

    ## Get first element by placeholder text

    When using [`HtmlElement`](web_sys::HtmlElement) type as the generic the function will return
    the first element which has the correct placeholder text[^note].

    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn get_first_element_by_placeholder_text() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let element: HtmlElement = rendered
            .get_by_placeholder_text("Username")
            .unwrap();

        assert_eq!("username-element", element.id());
    }
    ```
    [^note]_Use [`HtmlElement`](web_sys::HtmlElement) with care and only when you truly want to
    find the first element with a display value regardless of it’s type._

    */
    fn get_by_placeholder_text<T>(&self, search: &str) -> Result<T, Error>
    where
        T: JsCast;

    /// A convenient method which unwraps the result of
    /// [`get_by_placeholder_text`](ByPlaceholderText::get_by_placeholder_text).
    fn assert_by_placeholder_text<T>(&self, search: &str) -> T
    where
        T: JsCast;
}

impl ByPlaceholderText for QueryElement {
    fn assert_by_placeholder_text<T>(&self, search: &str) -> T
    where
        T: JsCast,
    {
        let result = self.get_by_placeholder_text(search);
        if result.is_err() {
            self.remove();
        }
        result.unwrap()
    }

    fn get_by_placeholder_text<T>(&self, search: &str) -> Result<T, Error>
    where
        T: JsCast,
    {
        let holders = self.query_selector_all(":placeholder-shown").ok();

        let holders = RawNodeListIter::<T>::new(holders).filter_map(|holder| match holder
            .dyn_into::<HtmlInputElement>(
        ) {
            Ok(e) => Some((e.placeholder(), e.unchecked_into::<T>())),
            Err(t) => t
                .dyn_into::<HtmlTextAreaElement>()
                .map(|e| (e.placeholder(), e.unchecked_into::<T>()))
                .ok(),
        });
        if let Some((ph, e)) = sap_utils::closest(search, holders, |(k, _)| k) {
            if search == ph {
                Ok(e)
            } else {
                Err(Box::new(ByPlaceholderTextError::Closest {
                    search_term: search.to_owned(),
                    inner_html: self.inner_html(),
                    closest_node: e.unchecked_into(),
                }))
            }
        } else {
            Err(Box::new(ByPlaceholderTextError::NotFound {
                search_term: search.to_owned(),
                inner_html: self.inner_html(),
            }))
        }
    }
}

/**
An error indicating that no element with a placeholder text was an equal match for a given search term.
*/
enum ByPlaceholderTextError {
    /// No element could be found with the given search term.
    NotFound {
        search_term: String,
        inner_html: String,
    },
    /**
    No element placeholder text was an exact match for the search term could be found, however, an
    element with a similar placeholder text as the search term was found.

    This should help find elements when a user has made a typo in either the test or the
    implementation being tested or when trying to find text with a dynamic number that may be
    incorrect
    */
    Closest {
        search_term: String,
        inner_html: String,
        closest_node: Node,
    },
}

impl Debug for ByPlaceholderTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByPlaceholderTextError::NotFound {
                search_term,
                inner_html,
            } => {
                write!(
                    f,
                    "\nNo element found with placeholder text equal or similar to '{}' in the following HTML:{}",
                    search_term,
                    sap_utils::format_html(inner_html)
                )
            }
            ByPlaceholderTextError::Closest {
                search_term,
                inner_html,
                closest_node,
            } => {
                write!(
                    f,
                    "\nNo exact match found for the placeholder text: '{}'.\nA similar match was found in the following HTML:{}",
                    search_term,
                    sap_utils::format_html_with_closest(inner_html, closest_node.unchecked_ref())
                )
            }
        }
    }
}

impl Display for ByPlaceholderTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for ByPlaceholderTextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{Element, HtmlElement};

    use super::*;
    use crate::{make_element_with_html_string, QueryElement};

    #[wasm_bindgen_test]
    fn get_input_by_placeholder_text() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                <input id="34" placeholder="Username" />
            </div>
        """#,
        )
        .into();

        let result: HtmlElement = rendered.get_by_placeholder_text("Username").unwrap();
        assert_eq!("34", result.id());
    }

    #[wasm_bindgen_test]
    fn get_textarea_by_placeholder_text() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div>
                <textarea id="23" placeholder="Enter bio here"></textarea>
            </div>
        "#,
        )
        .into();

        let result: HtmlElement = rendered.get_by_placeholder_text("Enter bio here").unwrap();
        assert_eq!("23", result.id());

        assert!(rendered
            .get_by_placeholder_text::<Element>("Enter life story")
            .is_err());
    }

    #[wasm_bindgen_test]
    fn get_errors() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <input placeholder="Username" type="text" />
        "#,
        )
        .into();

        let result = rendered.get_by_placeholder_text::<HtmlInputElement>("usrname");

        match result {
            Ok(_) => {
                panic!("Should not have found the input as the placeholder is not an exact match!")
            }
            Err(error) => {
                let expected = format!(
                    "\nNo exact match found for the placeholder text: '{}'.\nA similar match was found in the following HTML:{}",
                    "usrname",
                    r#"
<input placeholder="Username" type="text">
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Did you mean to find this element?
"#
                );

                assert_eq!(expected, format!("{:?}", error));
            }
        }

        drop(rendered);

        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div>
                Click me!
            </div>
        "#,
        )
        .into();

        let result = rendered.get_by_placeholder_text::<HtmlTextAreaElement>("Enter bio");

        match result {
            Ok(_) => panic!("Should not have found the div as the text is not a match and the generic type is too restrictive"),
            Err(err) => {
                let expected = format!(
                    "\nNo element found with placeholder text equal or similar to '{}' in the following HTML:{}",
                    "Enter bio",
                    r#"
<div>Click me!</div>
"#
                );
                assert_eq!(expected, format!("{:?}", err));
            }
        }
    }
}
