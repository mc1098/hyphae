/*!
Supports finding: [`HtmlInputElement`](web_sys::HtmlInputElement) or
[`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)
generically by `label text`.

# Label Text
[`HtmlInputElement`](web_sys::HtmlInputElement) and
[`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)
can have a [`HtmlLabelElement`] associated to it by setting the `for` attribute of the label with
the value of the labelled element's `id` attribute:

```html
<label for="username">Username:</label>
                      ^^^^^^^^^ the "label text"
<input id="username" type="text" />
```
The `for` attribute of the label element must match the `id` attribute of the input or output element
in order to be found.

# Generics
Each trait function supports generics for convenience and to help narrow the scope of the search. If
you are querying for a [`HtmlInputElement`](web_sys::HtmlInputElement) then you won't find a
[`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)
and vice versa.

In [`hyphae`](crate) the [`HtmlElement`](web_sys::HtmlElement) can be used as a "catch all" generic
type[^note].

[^note]: _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._

# What is [`JsCast`]?

The generic type returned needs to impl [`JsCast`] which is a trait from [`wasm_bindgen`] crate for
performing checked and unchecked casting between JS types.

*/
use std::fmt::Display;

use wasm_bindgen::JsCast;
use web_sys::HtmlLabelElement;

use crate::{Error, QueryElement};

/**
Enables queries by `label text`.

_See each trait function for examples._
*/
pub trait ByLabelText {
    /**
    Get a generic element by the first label element which matches the label text and has the correct
    associated element type.

    The possible elements that can be returned are:
    - [`HtmlInputElement`](web_sys::HtmlElement)
    - [`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)

    Using one of the generic types above as `T` will skip any elements of the other type - if you
    want to find the first element that matches the label text then use [`HtmlElement`](web_sys::HtmlElement).

    _See [`get_by_label_text_inc`](ByLabelText::get_by_label_text_inc) for getting the element and
    the label element._

    # Errors

    - Label not found

        When no matching label text can be found.
    - No element found

        When at least a single label was found with the correct text but no associated element was
        found, this could happen for the following reasons:
        1. Label does not have a `for` attribute
        2. Label has a `for` attribute but the there is no element with a corresponding `id`.
        3. Label has a `for` attribute but the corresponding `id` is an element that is not the
            expected type, such as an output element when [`HtmlInputElement`](web_sys::HtmlInputElement)
            was used for `T`.

    # Panics
    _Nothing to see here._

    # Examples

    ## Get input by label text

    The label text matches the search and the label has a valid `for` attribute linking to a input
    element. This returns an [`Result::Ok`] with the value of `T`.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">What needs to be done?</label>
            <br />
            <input id="new-todo" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_input_by_label_text() {
        let rendered: QueryElement = // feature dependent rendering
            # QueryElement::new();
        let input: HtmlInputElement = rendered
            .get_by_label_text("What needs to be done?")
            .expect("To find the input by label text");

        assert_eq!("hi!".to_owned(), input.value());
    }
    ```
    ## Label not found

    When the searched text doesn't match any labels then a [`Result::Err`] will be returned
    with an error about the label not being found.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">What doesn't need to be done?</label>
            <br />
            <input id="new-todo" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::{HtmlElement, HtmlTextAreaElement};

    #[wasm_bindgen_test]
    fn label_not_found() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let result = rendered
            .get_by_label_text::<HtmlElement>("What needs to be done?");

        assert!(result.is_err());
    }
    ```
    ## Label found but `for` value doesn't match input `id`

    When a label element is found with the search text, however, the `for` value doesn't match the
    input element's `id`. This will return a [`Result::Err`] with an error complaining that no element
    could be found.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">"What needs to be done?</label>
            <br />
            <input id="typo-on-id" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::{HtmlElement, HtmlTextAreaElement};

    #[wasm_bindgen_test]
    fn label_found_but_no_matching_input_element() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let result = rendered
            .get_by_label_text::<HtmlElement>("What needs to be done?");

        assert!(result.is_err());
    }
    ```
    */
    fn get_by_label_text<T>(&self, search: &str) -> Result<T, Error>
    where
        T: JsCast,
    {
        self.get_by_label_text_inc(search).map(|(e, _)| e)
    }

    /// A convenient method which unwraps the result of
    /// [`get_by_label_text`](ByLabelText::get_by_label_text).
    fn assert_by_label_text<T>(&self, search: &str) -> T
    where
        T: JsCast,
    {
        self.assert_by_label_text_inc(search).0
    }

    /**
    Get a generic element and it's associated label, by the first label element which matches the
    label text and has the correct associated element type.

    The possible elements that can be returned with the [`HtmlLabelElement`] are:
    - [`HtmlInputElement`](web_sys::HtmlInputElement)
    - [`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)

    Using one of the generic types above as `T` will skip any elements of the other type - if you
    want to find the first element that matches the label text then use [`HtmlElement`](web_sys::HtmlElement).

    # Errors

    - Label not found

        When no matching label text can be found.
    - No element found

        When at least a single label was found with the correct text but no associated element was
        found, this could happen for the following reasons:
        1. Label does not have a `for` attribute
        2. Label has a `for` attribute but the there is no element with a corresponding `id`.
        3. Label has a `for` attribute but the corresponding `id` is an element that is not the
            expected type, such as an output element when [`HtmlInputElement`](web_sys::HtmlInputElement)
            was used for `T`.

    # Panics
    _Nothing to see here._

    # Examples

    ## Get input by label text

    The label text matches the search and the label has a valid `for` attribute linking to a input
    element. This returns an [`Result::Ok`] with the value of `T`.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">What needs to be done?</label>
            <br />
            <input id="new-todo" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::{HtmlInputElement, HtmlLabelElement};

    #[wasm_bindgen_test]
    fn get_input_by_label_text() {
        let rendered: QueryElement = // feature dependent rendering
            # QueryElement::new();
        // turbo fish is recommended over this approach
        let (input, label): (HtmlInputElement, HtmlLabelElement) = rendered
            .get_by_label_text_inc("What needs to be done?")
            .expect("To find the input by label text");

        assert_eq!("hi!".to_owned(), input.value());
    }
    ```
    ## Label not found

    When the searched text doesn't match any labels then a [`Result::Err`] will be returned
    with an error complaining that the label could not be found.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">What doesn't need to be done?</label>
            <br />
            <input id="new-todo" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::{HtmlElement, HtmlTextAreaElement};

    #[wasm_bindgen_test]
    fn label_not_found() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let result = rendered
            .get_by_label_text_inc::<HtmlElement>("What needs to be done?");

        assert!(result.is_err());
    }
    ```
    ## Label found but `for` value doesn't match input `id`

    When a label element is found with the search text, however, the `for` value doesn't match the
    input element's `id`. This will return a [`Result::Err`] with an error complaining that no
    element could be found.

    Rendered html:
    ```html
    <div>
        <form>
            <label for="new-todo">"What needs to be done?</label>
            <br />
            <input id="typo-on-id" value="hi!" />
        </form>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use hyphae::prelude::*;
    use web_sys::{HtmlElement, HtmlTextAreaElement};

    #[wasm_bindgen_test]
    fn label_found_but_no_matching_input_element() {
        let rendered: QueryElement = // feature dependent rendering
        # QueryElement::new();
        let result = rendered
            .get_by_label_text_inc::<HtmlElement>("What needs to be done?");

        assert!(result.is_err());
    }
    ```
    */
    fn get_by_label_text_inc<T>(&self, search: &str) -> Result<(T, HtmlLabelElement), Error>
    where
        T: JsCast;

    /// A convenient method which unwraps the result of
    /// [`get_by_label_text_inc`](ByLabelText::get_by_label_text_inc).
    fn assert_by_label_text_inc<T>(&self, search: &str) -> (T, HtmlLabelElement)
    where
        T: JsCast;
}

impl ByLabelText for QueryElement {
    fn assert_by_label_text_inc<T>(&self, search: &str) -> (T, HtmlLabelElement)
    where
        T: JsCast,
    {
        let result = self.get_by_label_text_inc(search);
        if result.is_err() {
            self.remove();
        }
        result.unwrap()
    }

    fn get_by_label_text_inc<T>(&self, search: &str) -> Result<(T, HtmlLabelElement), Error>
    where
        T: JsCast,
    {
        let labels = match self.query_selector_all("label") {
            Ok(labels) => labels,
            Err(_) => {
                return Err(Box::new(ByLabelTextError::LabelNotFound {
                    search_term: search.to_owned(),
                    inner_html: self.inner_html(),
                }))
            }
        };

        let mut labels_matching_search = 0;
        let mut ids_found = vec![];

        for i in 0..labels.length() {
            let label = labels.get(i).unwrap();
            if label
                .text_content()
                .map(|text| text == search)
                .unwrap_or_default()
            {
                labels_matching_search += 1;
                let label_element: HtmlLabelElement = label.unchecked_into();
                if let Some(id) = label_element.get_attribute("for") {
                    let node_list = self
                        .query_selector_all(&format!("output[id={0}], input[id={0}]", id))
                        .unwrap();

                    for j in 0..node_list.length() {
                        let node = node_list.get(j).unwrap();
                        if let Ok(element) = node.dyn_into() {
                            return Ok((element, label_element));
                        }
                    }
                    // only push at the end - happy path == no allocation for vec
                    ids_found.push(id);
                }
            }
        }

        if labels_matching_search == 0 {
            Err(Box::new(ByLabelTextError::LabelNotFound {
                search_term: search.to_owned(),
                inner_html: self.inner_html(),
            }))
        } else {
            Err(Box::new(ByLabelTextError::NoElementFound {
                search_term: search.to_owned(),
                no_of_labels: labels_matching_search,
                ids_found,
                inner_html: self.inner_html(),
            }))
        }
    }
}

/**
The label text was not found or no element could be found associated with the label element found.
*/
enum ByLabelTextError {
    /// No [`HtmlLabelElement`] could be found with a text content that matches the search term.
    LabelNotFound {
        search_term: String,
        inner_html: String,
    },
    /**
    A [`HtmlLabelElement`] was found but either had `for` attribute or no
    [`Element`](web_sys::Element) could be found with an `id` matching the value of the `for`
    attribute.

    ## Values:
    The first value is the original search term - used for displaying a useful error message on unwraps.

    The second value is the number of labels found that match the search term.

    The third is the ids of the found labels.

    Note: The number of labels found and the number of ids can differ when a label with the correct
    search term doesn't have a 'for' attribute
     */
    NoElementFound {
        search_term: String,
        no_of_labels: usize,
        ids_found: Vec<String>,
        inner_html: String,
    },
}

impl std::fmt::Debug for ByLabelTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByLabelTextError::LabelNotFound {
                search_term,
                inner_html,
            } => {
                writeln!(
                    f,
                    "No label found with text: '{}' in the following HTML:{}",
                    search_term,
                    hyphae_utils::format_html(inner_html)
                )
            }
            ByLabelTextError::NoElementFound {
                search_term,
                no_of_labels,
                ids_found,
                inner_html,
            } => {
                if *no_of_labels == 1 {
                    write!(f, "Found a label ")?;
                } else {
                    write!(f, "Found {} labels ", no_of_labels)?;
                }
                write!(f, "with the text: '{}'\n however, ", search_term,)?;

                match &ids_found[..] {
                    [] => {
                        f.write_str("no 'for' attributes were found.")?;
                        return Ok(());
                    },
                    [id] => writeln!(f, "no element of the correct type was found associated with the following id '{}' ", id)?,
                    _ => writeln!(f, "no element of the correct type was found associated with the following ids: '{}' ", ids_found.join(","))?,
                }

                writeln!(
                    f,
                    "in the following HTML:{}",
                    hyphae_utils::format_html(inner_html)
                )?;

                if *no_of_labels != ids_found.len() {
                    writeln!(
                        f,
                        "Note: Some labels found to match don't have a 'for' attribute!"
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl Display for ByLabelTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for ByLabelTextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use hyphae::QueryElement;
    use hyphae_utils::make_element_with_html_string;

    use web_sys::{HtmlElement, HtmlInputElement};

    fn input_label_text() -> QueryElement {
        make_element_with_html_string(
            r#"""
            <div>
                <form>
                    <label for="new-todo">What needs to be done?</label>
                    <br />
                    <input id="new-todo" value="hi!" />
                </form>
            </div>
        """#,
        )
        .into()
    }

    fn input_label_text_label_after_input() -> QueryElement {
        make_element_with_html_string(
            r#"""
            <div>
                <form>
                    <input id="new-todo" value="hi!" />
                    <br />
                    <label for="new-todo">What needs to be done?</label>
                </form>
            </div>
        """#,
        )
        .into()
    }

    fn input_label_text_different_parents() -> QueryElement {
        make_element_with_html_string(
            r#"""
            <div>
                <form>
                    <div>
                        <label for="new-todo">What needs to be done?</label>
                    </div>
                    <br />
                    <input id="new-todo" value="hi!" />
                </form>
            </div>
        """#,
        )
        .into()
    }

    #[wasm_bindgen_test]
    fn get_inputs_by_label_text() {
        let mut tests = vec![
            input_label_text(),
            input_label_text_different_parents(),
            input_label_text_label_after_input(),
        ];

        for test in tests.drain(..) {
            let result = test.get_by_label_text("What needs to be done?");

            let input: HtmlInputElement = result.unwrap();
            assert_eq!("hi!".to_owned(), input.value());
        }
    }

    #[wasm_bindgen_test]
    fn no_element_found_when_id_and_for_do_not_match() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                <form>
                    <label for="new-todoz">What needs to be done?</label>
                    <br />
                    <input id="new_todo" value="hi!" />
                </form>
            </div>
        """#,
        )
        .into();

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn text_not_found_when_search_term_not_found_in_label() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <div>
                <form>
                    <label for="new-todo">What doesn't need to be done?</label>
                    <br />
                    <input id="new-todo" value="hi!" />
                </form>
            </div>
        """#,
        )
        .into();

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn input_value_change() {
        let label_text = "What needs to be done?";
        let rendered: QueryElement = make_element_with_html_string(
            r#"""
            <>
                <label for="todo">What needs to be done?</label>
                <input type="text" id="todo" value="" />
            </>
        """#,
        )
        .into();

        let new_value = "Gardening";

        let input: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        input.set_value(new_value);

        let input_after: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        assert_eq!(new_value, input_after.value());
        assert_eq!(input, input_after);
    }
}
