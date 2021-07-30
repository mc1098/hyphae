/*!
Supports finding: [`HtmlInputElement`], [`HtmlSelectElement`], [`HtmlTextAreaElement`] generically
by `display value`.

# Display value

The `display value` for each element:
- [`HtmlInputElement`]\:
    ```html
    <input type="text" value="Welcome" />
                              ^^^^^^^ the "display value"
    ```
- [`HtmlSelectElement`]\:

    The `display value`s possible are listed by the `option` elements - the current `display value`
    will be whichever option is selected by the user.
    ```html
    <select>
        <option value="first">First Value</option>
        <option value="second" selected>Second Value</option>
                       ^^^^^^ default "display value"
        <option value="third">Third Value</option>
    </select>
    ```
    The second `option` is the default due to the `selected` boolean attribute but without the
    default will normally be the first `option`[^note].

    [^note] _Needs to be confirmed that this is the standard_
- [`HtmlTextAreaElement`]\:

    The `display value` will be current text found in the textarea element.
    ```html
    <textarea rows="10" cols="80">Write something here</textarea>
                                  ^^^^^^^^^^^^^^^^^^^^ default "display value"
    ```
    This may seem the same as getting the textContent of the element, however, when the user
    edits the text in the `textarea` the `display value` will reflect this change and the
    textContent won't.

# Generics
Each trait function supports generics for convenience and to help narrow the scope of the search. If
you are querying for a [`HtmlInputElement`] by `display value` then you won't find either
[`HtmlSelectElement`], [`HtmlTextAreaElement`].

In [`Sap`](crate) the [`HtmlElement`](web_sys::HtmlElement) can be used as a "catch all" generic
type[^note].

[^note] _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._

# What is [`JsCast`]?

The generic type returned needs to impl [`JsCast`] which is a trait from [`wasm_bindgen`] crate for
performing checked and unchecked casting between JS types.

*/
use std::fmt::Debug;

use wasm_bindgen::JsCast;
use web_sys::Node;

use crate::{RawNodeListIter, TestRender};

/**
Enables querying elements by `display value`.

_See each trait function for examples._
 */
pub trait ByDisplayValue {
    /**
    Get a generic element by the display value.

    The possible elements that can be returned are:
    - [`HtmlInputElement`]
    - [`HtmlSelectElement`]
    - [`HtmlTextAreaElement`]

    Using one of the generic types above as `T` will essentially skip the other two types of
    elements - if you want to find the very first element that matches the display value then use
    [`HtmlElement`](web_sys::HtmlElement).

    # Panics
    _Nothing to see here._

    # Examples

    Rendered html:
    ```html
    <div id="my-display-value-elements">
        <textarea id="greeting-textarea">Welcome</textarea>
        <select id="greeting-select">
            <option value="Welcome" selected>Welcome</option>
            <option value="Hello">Hello</option>
        </select>
        <input id="greeting-input" type="text" value="Welcome" />
    </div>
    ```

    ## Get input by display value

    The first element with the display value of "Welcome" is the textarea, however, this function
    will return the last element because of the [`HtmlInputElement`] generic.
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_input_by_display_value() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div id="my-display-value-elements">
            #   <textarea id="greeting-textarea">{ "Welcome" }</textarea>
            #   <select id="greeting-select">
            #       <option value="Welcome" selected=true>{ "Welcome" }</option>
            #       <option value="Hello">{ "Hello" }</option>
            #   </select>
            #   <input id="greeting-input" type="text" value="Welcome" />
            # </div>
        # };
        let input: HtmlInputElement = rendered
            .get_by_display_value("Welcome")
            .unwrap();

        assert_eq!("greeting-input", input.id());
    }
    ```

    ## Get select by display value

    The first element with the display value of "Welcome" is the textarea, however, this function
    will return the second element because of the [`HtmlSelectElement`] generic.
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlSelectElement;

    #[wasm_bindgen_test]
    fn get_select_by_display_value() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div id="my-display-value-elements">
            #   <textarea id="greeting-textarea">{ "Welcome" }</textarea>
            #   <select id="greeting-select">
            #       <option value="Welcome" selected=true>{ "Welcome" }</option>
            #       <option value="Hello">{ "Hello" }</option>
            #   </select>
            #   <input id="greeting-input" type="text" value="Welcome" />
            # </div>
        # };
        let select = rendered
            .get_by_display_value::<HtmlSelectElement>("Welcome") // can use turbo fish
            .unwrap();

        assert_eq!("greeting-select", select.id());
    }
    ```

    ## Get textarea by display value

    The first element with the display value of "Welcome" is the textarea and this is the element
    that will be returned by this function.

    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlTextAreaElement;

    #[wasm_bindgen_test]
    fn get_text_area_by_display_value() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div id="my-display-value-elements">
            #   <textarea id="greeting-textarea">{ "Welcome" }</textarea>
            #   <select id="greeting-select">
            #       <option value="Welcome" selected=true>{ "Welcome" }</option>
            #       <option value="Hello">{ "Hello" }</option>
            #   </select>
            #   <input id="greeting-input" type="text" value="Welcome" />
            # </div>
        # };
        let text_area: HtmlTextAreaElement = rendered
            .get_by_display_value("Welcome")
            .unwrap();

        assert_eq!("greeting-textarea", text_area.id());
    }
    ```

    ## Get first element with display value

    When using [`HtmlElement`](web_sys::Element) type as the generic the function will return the
    first element which has the correct display value[^note].

    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn get_text_area_by_display_value() {
        let rendered: TestRender = // feature dependent rendering
        # test_render! {
            # <div id="my-display-value-elements">
            #   <textarea id="greeting-textarea">{ "Welcome" }</textarea>
            #   <select id="greeting-select">
            #       <option value="Welcome" selected=true>{ "Welcome" }</option>
            #       <option value="Hello">{ "Hello" }</option>
            #   </select>
            #   <input id="greeting-input" type="text" value="Welcome" />
            # </div>
            # };
        let element: HtmlElement = rendered
            .get_by_display_value("Welcome")
            .unwrap();

        assert_eq!("greeting-textarea", element.id());
    }
    ```
    [^note] _Use [`HtmlElement`](web_sys::HtmlElement) with care and only when you truly want to
    find the first element with a display value regardless of it's type._
    */
    fn get_by_display_value<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<T, ByDisplayValueError<'search>>
    where
        T: JsCast;
}

impl ByDisplayValue for TestRender {
    fn get_by_display_value<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<T, ByDisplayValueError<'search>>
    where
        T: JsCast,
    {
        let elements = self
            .root_element
            .query_selector_all("input, select, textarea")
            .ok();

        let display_values = RawNodeListIter::<T>::new(elements).filter_map(|element| {
            sap_utils::get_element_value(&element).map(|value| (value, element))
        });

        if let Some((dv, e)) = sap_utils::closest(search, display_values, |(k, _)| k) {
            if search == dv {
                Ok(e)
            } else {
                Err(ByDisplayValueError::Closest((
                    search,
                    self.inner_html(),
                    e.unchecked_into(),
                )))
            }
        } else {
            Err(ByDisplayValueError::NotFound((search, self.inner_html())))
        }
    }
}

/**
An error indicating that no element with a display value was an equal match for a given search term.
*/
pub enum ByDisplayValueError<'search> {
    /// No element could be found with the given search term.
    NotFound((&'search str, String)),
    /**
    No element display value was an exact match for the search term could be found, however, an
    element with a similar display value as the search term was found.

    This should help find elements when a user has made a typo in either the test or the
    implementation being tested or when trying to find text with a dynamic number that may be
    incorrect
    */
    Closest((&'search str, String, Node)),
}

impl Debug for ByDisplayValueError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByDisplayValueError::NotFound((search, html)) => {
                write!(
                    f,
                    "\nNo element found with a display value equal or similar to '{}' in the following HTML:{}",
                    search,
                    sap_utils::format_html(html)
                )
            }
            ByDisplayValueError::Closest((search, html, closest)) => {
                write!(
                    f,
                    "\nNo exact match found for a display value of: '{}'.\nA similar match was found in the following HTML:{}",
                    search,
                    sap_utils::format_html_with_closest(html, closest.unchecked_ref()),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use web_sys::{Element, HtmlInputElement, HtmlTextAreaElement};

    use crate::TestRender;
    use sap_yew::test_render;

    #[wasm_bindgen_test]
    fn get_input_by_display_value() {
        let rendered = test_render! {
            <input type="text" id="greeting" value="Welcome" />
        };

        let input: HtmlInputElement = rendered.get_by_display_value("Welcome").unwrap();
        assert_eq!("greeting", input.id());
    }

    #[wasm_bindgen_test]
    fn get_text_area_due_to_type() {
        let rendered = test_render! {
            <>
                <input type="text" id="input" value="hello" />
                <textarea id="textarea" value="hello" />
            </>
        };

        let text_area: HtmlTextAreaElement = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("textarea", text_area.id());

        let input: HtmlInputElement = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("input", input.id());

        let first: Element = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("input", first.id());
    }

    #[wasm_bindgen_test]
    fn get_errors() {
        let rendered = test_render! {
            <input type="text" value="this is it!" />
        };

        let result = rendered.get_by_display_value::<HtmlInputElement>("this isn't it!");

        match result {
            Ok(_) => {
                panic!(
                    "Should not have found the input as the display value is not an exact match!"
                )
            }
            Err(error) => {
                let expected = format!(
                    "\nNo exact match found for a display value of: '{}'.\nA similar match was found in the following HTML:{}",
                    // "\nNo exact match found for a display value of: '{}'\nDid you mean to find this Element:\n\t{}\n",
                    "this isn't it!",
                    r#"
<input type="text">
^^^^^^^^^^^^^^^^^^^ Did you mean to find this element?
"#
                    // "<input type=\"text\">"
                );

                assert_eq!(expected, format!("{:?}", error));
            }
        }

        drop(rendered);

        let rendered = test_render! {
            <span>
                { "Not my bio!" }
            </span>
        };

        let result = rendered.get_by_display_value::<HtmlTextAreaElement>("My bio!");

        match result {
            Ok(_) => panic!("Should not have found the div as the text is not a match and the generic type is too restrictive"),
            Err(err) => {
                let expected = format!(
                    "\nNo element found with a display value equal or similar to '{}' in the following HTML:{}",
                    // "\nNo element found with a display value equal or similar to '{}'\n",
                    "My bio!",
                    r#"
<span>Not my bio!</span>
"#
                );
                assert_eq!(expected, format!("{:?}", err));
            }
        }
    }
}
