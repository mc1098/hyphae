use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use crate::{RawNodeListIter, TestRender};

/**
Enables querying by placeholder text.
The following [`Element`](web_sys::Element)s have placeholder text:
- [`HtmlInputElement`]
- [`HtmlTextAreaElement`]
 */
pub trait ByPlaceholderText {
    /**
    Get an [`Element`](web_sys::Element) by it's placeholder text.
    The following elements can be found by their placeholder text:
    - [`HtmlInputElement`]
    - [`HtmlTextAreaElement`]

    # Examples
    ```no_run
    /*
    rendered with effective html:
    <div>
        <input id="34" placeholder="Username" />
    </div>
    */
    let rendered: TestRender = //..
    let result: HtmlElement = rendered.get_by_placeholder_text("Username").unwrap();

    assert_eq!("34", result.id());
    ```
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
