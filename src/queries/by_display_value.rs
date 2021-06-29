use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use crate::TestRender;

/**
Enables querying elements by display value.
 */
pub trait ByDisplayValue {
    /**
    Get a [`Element`](web_sys::Element) by the display value.

    Returns the first [`Element`](web_sys::Element) with a value that is equal to the search term.

    The possible elements that can be returned are:
    - [`HtmlInputElement`]
    - [`HtmlSelectElement`]
    - [`HtmlTextAreaElement`]

    # Examples
    Effective html rendered:
    ```html
    <input id="greeting" type="text" value="Welcome" />
    ```
    ```no_run
    let rendered: TestRender = // ..
    let input: HtmlInputElement = rendered
        .get_by_display_value("Welcome")
        .unwrap();
    assert_eq!("greeting", input.id());
    ```
    ## Narrowing search using generics
    Effective html rendered:
    ```html
    <input type="text" id="input" value="hello" />
    <textarea id="textarea" value="hello" />
    ```
    ```no_run
    let rendered: TestRender = // ..

    // Skip the first input element because it's not a `HtmlTextAreaElement`
    let text_area: HtmlTextAreaElement = rendered
        .get_by_display_value("hello")
        .unwrap();
    assert_eq!("textarea", text_area.id());

    // Should find the first input element.
    let input: HtmlInputElement = rendered.get_by_display_value("hello").unwrap();
    assert_eq!("input", input.id());

    // Using a generic `Element` type will find the first element that matches!
    let first: Element = rendered.get_by_display_value("hello").unwrap();
    assert_eq!("input", first.id());
    ```
    */
    fn get_by_display_value<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast;

    /**
    Expect only a single [`Element`](web_sys::Element) from this query - if no or many results
    come from this query then it will panic.
    */
    fn expect_by_display_value<T>(&self, search: &'_ str) -> T
    where
        T: JsCast;
}

impl ByDisplayValue for TestRender {
    fn get_by_display_value<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast,
    {
        let displays = self
            .root_element
            .query_selector_all("input, select, textarea")
            .ok()?;

        for i in 0..displays.length() {
            let display = displays.get(i)?;

            /*
            T is an unknown type so can't call T::value().
            So convert to input, textarea, select for the value method to check if it matches
            search.

            A node might match an element type and have the correct value but if it can not be
            converted to T, using JsCast::dyn_into, then it is skipped and we continue looking.
            This behaviour allows a user to narrow the search based on the type they provide.
            */

            let display = match display.dyn_into::<HtmlInputElement>() {
                Ok(input) if input.value() == search => match input.dyn_into() {
                    Ok(result) => return Some(result),
                    Err(node) => node.unchecked_into(),
                },
                Err(node) => node,
                _ => continue,
            };

            let display = match display.dyn_into::<HtmlTextAreaElement>() {
                Ok(area) if area.value() == search => match area.dyn_into() {
                    Ok(result) => return Some(result),
                    Err(node) => node.unchecked_into(),
                },
                Err(node) => node,
                _ => continue,
            };

            if let Ok(select) = display.dyn_into::<HtmlSelectElement>() {
                if select.value() == search {
                    if let Ok(result) = select.dyn_into() {
                        return Some(result);
                    }
                }
            }
        }

        None
    }

    fn expect_by_display_value<T>(&self, search: &'_ str) -> T
    where
        T: JsCast,
    {
        let displays = self
            .root_element
            .query_selector_all("input, select, textarea")
            .expect("Error trying to query the DOM");

        let mut found = vec![];

        for i in 0..displays.length() {
            let display = displays.get(i).unwrap();

            let display = match display.dyn_into::<HtmlInputElement>() {
                Ok(input) if input.value() == search => {
                    match input.dyn_into::<T>() {
                        Ok(result) => {
                            found.push(result);
                            continue;
                        }
                        Err(node) => node.unchecked_into(), // safe as it must be a node!
                    }
                }
                Err(node) => node,
                _ => continue,
            };

            let display = match display.dyn_into::<HtmlTextAreaElement>() {
                Ok(area) if area.value() == search => match area.dyn_into::<T>() {
                    Ok(result) => {
                        found.push(result);
                        continue;
                    }
                    Err(node) => node.unchecked_into(),
                },
                Err(node) => node,
                _ => continue,
            };

            if let Ok(select) = display.dyn_into::<HtmlSelectElement>() {
                if select.value() == search {
                    if let Ok(result) = select.dyn_into::<T>() {
                        found.push(result);
                    }
                }
            }
        }

        if found.is_empty() {
            panic!(
                "No input, select, or textarea elements could be found with the display value of:
                {} in the following html:\n {:?}",
                search,
                self.inner_html()
            )
        } else if found.len() == 1 {
            found.pop().unwrap()
        } else {
            panic!(
                "Expected only one element to match!\n
                Found {} elements with a value of `{}` in the following html:\n {:?}",
                found.len(),
                search,
                self.inner_html(),
            )
        }
    }
}

#[cfg(test)]
mod tests {

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use web_sys::{Element, HtmlInputElement};
    use yew::{html, virtual_dom::test_render};

    use crate::TestRender;

    #[wasm_bindgen_test]
    fn get_input_by_display_value() {
        let rendered = TestRender::new(test_render(html! {
            <input type="text" id="greeting" value="Welcome" />
        }));

        let input: HtmlInputElement = rendered.get_by_display_value("Welcome").unwrap();
        assert_eq!("greeting", input.id());
    }

    #[wasm_bindgen_test]
    fn get_text_area_due_to_type() {
        let rendered: TestRender = test_render(html! {
            <>
                <input type="text" id="input" value="hello" />
                <textarea id="textarea" value="hello" />
            </>
        })
        .into();

        let text_area: HtmlTextAreaElement = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("textarea", text_area.id());

        let input: HtmlInputElement = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("input", input.id());

        let first: Element = rendered.get_by_display_value("hello").unwrap();
        assert_eq!("input", first.id());
    }
}
