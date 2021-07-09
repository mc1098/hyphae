/*!
Supports finding: [`HtmlInputElement`] or [`HtmlTextAreaElement`] generically by `placeholder text`.

Using this trait to assert that a `placeholder text` is correct is valid, however, placeholders are
**not** accessible - consider whether you can find your element by the following (in the order given):
1. [`get_by_aria_role`](super::by_role::ByAria::get_by_aria_role)
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
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use crate::{RawNodeListIter, TestRender};

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
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_input_by_placeholder_text() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div>
            #   <input id="username-input" placeholder="Username" />
            #   <textarea id="username-textarea" placeholder="Username" />
            # </div>
        # };
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("Username")
            .expect("input to be found by it's placeholder text");

        assert_eq!("username-input", input.id());
    }
    ```

    ## Get textarea by placeholder text

    The first input element is skipped even though it has the correct placeholder text due to
    `T` being [`HtmlTextAreaElement`].

    ```no_run
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlTextAreaElement;

    #[wasm_bindgen_test]
    fn get_text_area_by_placeholder_text() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div>
            #   <input id="username-input" placeholder="Username" />
            #   <textarea id="username-textarea" placeholder="Username" />
            # </div>
        # };
        let text_area: HtmlTextAreaElement = rendered
            .get_by_placeholder_text("Username")
            .expect("textarea to be found by it's placeholder text");

        assert_eq!("username-textarea", text_area.id());
    }
    ```

    ## Get first element by placeholder text

    When using [`HtmlElement`](web_sys::HtmlElement) type as the generic the function will return
    the first element which has the correct placeholder text[^note].

    ```no_run
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn get_first_element_by_placeholder_text() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div>
            #   <input id="username-input" placeholder="Username" />
            #   <textarea id="username-textarea" placeholder="Username" />
            # </div>
        # };
        let element: HtmlElement = rendered
            .get_by_placeholder_text("Username")
            .expect("first element to be found with the placeholder `Username`");

        assert_eq!("username-element", element.id());
    }
    ```
    [^note]_Use [`HtmlElement`](web_sys::HtmlElement) with care and only when you truly want to
    find the first element with a display value regardless of it’s type._

    */
    fn get_by_placeholder_text<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast;
}

impl ByPlaceholderText for TestRender {
    fn get_by_placeholder_text<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast,
    {
        let holders = self
            .root_element
            .query_selector_all(":placeholder-shown")
            .ok();

        RawNodeListIter::<T>::new(holders).find_map(|mut holder| {
            holder = match holder.dyn_into::<HtmlInputElement>() {
                Ok(input) if input.placeholder() == search => return input.dyn_into().ok(),
                Err(node) => node.unchecked_into(),
                _ => return None,
            };

            match holder.dyn_into::<HtmlTextAreaElement>() {
                Ok(text_area) if text_area.placeholder() == search => text_area.dyn_into().ok(),
                _ => None,
            }
        })
    }
}

#[cfg(all(test, feature = "Yew"))]
mod tests {

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{Element, HtmlElement};

    use super::*;
    use crate::{test_render, TestRender};

    #[wasm_bindgen_test]
    fn get_input_by_placeholder_text() {
        let rendered = test_render! {
            <div>
                <input id="34" placeholder="Username" />
            </div>
        };

        let result: HtmlElement = rendered.get_by_placeholder_text("Username").unwrap();
        assert_eq!("34", result.id());
    }

    #[wasm_bindgen_test]
    fn get_textarea_by_placeholder_text() {
        let rendered = test_render! {
            <div>
                <textarea id="23" placeholder="Enter bio here" />
            </div>
        };

        let result: HtmlElement = rendered.get_by_placeholder_text("Enter bio here").unwrap();
        assert_eq!("23", result.id());

        assert!(rendered
            .get_by_placeholder_text::<Element>("Enter life story")
            .is_none());
    }
}
