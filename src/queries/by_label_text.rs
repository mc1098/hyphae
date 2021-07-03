use wasm_bindgen::JsCast;
use web_sys::HtmlLabelElement;

use crate::TestRender;

/**
Error indicating that either a [`HtmlLabelElement`] with a text content equal to the search term
was not found or that no linked [`Element`](web_sys::Element) could be found.
*/
pub enum LabelByTextError<'search> {
    /// No [`HtmlLabelElement`] could be found with a text content that matches the search term.
    LabelNotFound(&'search str),
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
    NoElementFound((&'search str, usize, Vec<String>)),
}

impl std::fmt::Debug for LabelByTextError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelByTextError::LabelNotFound(text) => {
                writeln!(f, "No label found with text: '{}'.", text)
            }
            LabelByTextError::NoElementFound((text, no_of_labels, ids)) => {
                if *no_of_labels == 1 {
                    write!(f, "Found a label ")?;
                } else {
                    write!(f, "Found {} labels ", no_of_labels)?;
                }
                write!(f, "with the text: '{}'\n however, ", text,)?;

                if ids.is_empty() {
                    f.write_str("no 'for' attributes were found.")?;
                    return Ok(());
                } else if ids.len() == 1 {
                    writeln!(
                        f,
                        "no element was found associated with the following id '{}'.",
                        ids[0]
                    )?;
                } else {
                    writeln!(
                        f,
                        "no element was found associated with the following ids: '{}'.",
                        ids.join(",")
                    )?;
                }

                if *no_of_labels != ids.len() {
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

/**
Enables queries by lable text content.
The following [`Element`](web_sys::Element)s can have labels:
- [`HtmlInputElement`](web_sys::HtmlInputElement)
- [`HtmlOutputElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlOutputElement.html)
*/
pub trait ByLabelText {
    /**
    Get an [`Element`](web_sys::Element) which is associated to the first [`HtmlLabelElement`] found
    with a text content matching the search term.

    When no [`HtmlLabelElement`] with the text content matching the search term can be found then
    this function will return `Err(LabelByTextError::LabelNotFound)`.

    When a [`HtmlLabelElement`] with the text content is found but has no 'for' attribute or no
    output or input element can be found with an `id` matching the value of the `for` attribute;
    then this function will return `Err(LabelByTextError::NoElementFound)`.

    # Examples

    ## Happy path:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What needs to be done?" }</label>
            <br />
            <input id="new-todo" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    let result = test.get_by_label_text("What needs to be done?");
    let input: HtmlInputElement = result.unwrap();

    assert_eq!("hi!".to_owned(), input.value());
    ```
    ## Label not found:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What doesn't needs to be done?" }</label>
            <br />
            <input id="new-todo" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    // Note: that the search term doesn't match the text in the label above
    let result = test.get_by_label_text::<HtmlElement>("What needs to be done?");

    assert!(matches!(result, Err(LabelByTextError::LabelNotFound(_))));
    ```
    ## Label found but `for` value doesn't match input `id`:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What doesn't needs to be done?" }</label>
            <br />
            <input id="typo-on-id" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    let result = test.get_by_label_text::<HtmlElement>("What needs to be done?");

    assert!(matches!(result, Err(LabelByTextError::NoElementFound(_))));
    ```
    */
    fn get_by_label_text<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<T, LabelByTextError<'search>>
    where
        T: JsCast,
    {
        self.get_by_label_text_inc(search).map(|(e, _)| e)
    }

    /**
    Get an [`Element`](web_sys::Element) and the associated first [`HtmlLabelElement`] found
    with a text content matching the search term.

    When no [`HtmlLabelElement`] with the text content matching the search term can be found then
    this function will return `Err(LabelByTextError::LabelNotFound)`.

    When a [`HtmlLabelElement`] with the text content is found but has no 'for' attribute or no
    output or input element can be found with an `id` matching the value of the `for` attribute;
    then this function will return `Err(LabelByTextError::NoElementFound)`.

    # Examples

    ## Happy path:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What needs to be done?" }</label>
            <br />
            <input id="new-todo" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    let (input, label) = test.get_by_label_text_inc::<HtmlInputElement>("What needs to be done?").unwrap();

    assert_eq!("hi!".to_owned(), input.value());
    ```
    ## Label not found:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What doesn't needs to be done?" }</label>
            <br />
            <input id="new-todo" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    // Note: that the search term doesn't match the text in the label above
    let result = test.get_by_label_text_inc::<HtmlElement>("What needs to be done?");

    assert!(matches!(result, Err(LabelByTextError::LabelNotFound(_))));
    ```
    ## Label found but `for` value doesn't match input `id`:
    ```no_run
    use sap::prelude::*;
    /*
    rendered with the effective html:
    <div>
        <form>
            <label for="new-todo">{ "What doesn't needs to be done?" }</label>
            <br />
            <input id="typo-on-id" value={"hi!"} />
        </form>
    </div>
    */
    let rendered: TestRender = //..
    let result = test.get_by_label_text_inc::<HtmlElement>("What needs to be done?");

    assert!(matches!(result, Err(LabelByTextError::NoElementFound(_))));
    ```
    */
    fn get_by_label_text_inc<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<(T, HtmlLabelElement), LabelByTextError<'search>>
    where
        T: JsCast;
}

impl ByLabelText for TestRender {
    fn get_by_label_text_inc<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<(T, HtmlLabelElement), LabelByTextError<'search>>
    where
        T: JsCast,
    {
        let labels = match self.root_element.query_selector_all("label") {
            Ok(labels) => labels,
            Err(_) => return Err(LabelByTextError::LabelNotFound(search)),
        };

        let mut labels_matching_search = 0;
        let mut ids_matching = vec![];

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
                    ids_matching.push(id);
                }
            }
        }

        if labels_matching_search == 0 {
            Err(LabelByTextError::LabelNotFound(search))
        } else {
            Err(LabelByTextError::NoElementFound((
                search,
                labels_matching_search,
                ids_matching,
            )))
        }
    }
}

#[cfg(all(test, feature = "Yew"))]
pub mod tests {

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    use crate::{test_render, TestRender};
    use web_sys::{HtmlElement, HtmlInputElement};

    fn input_label_text() -> TestRender {
        test_render! {
            <div>
                <form>
                    <label for="new-todo">{"What needs to be done?"}</label>
                    <br />
                    <input id="new-todo" value={"hi!"} />
                </form>
            </div>
        }
    }

    fn input_label_text_label_after_input() -> TestRender {
        test_render! {
            <div>
                <form>
                    <input id="new-todo" value={"hi!"} />
                    <br />
                    <label for="new-todo">{"What needs to be done?"}</label>
                </form>
            </div>
        }
    }

    fn input_label_text_different_parents() -> TestRender {
        test_render! {
            <div>
                <form>
                    <div>
                        <label for="new-todo">{"What needs to be done?"}</label>
                    </div>
                    <br />
                    <input id="new-todo" value={"hi!"} />
                </form>
            </div>
        }
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
        let rendered = test_render! {
            <div>
                <form>
                    <label for="new-todoz">{ "What needs to be done?" }</label>
                    <br />
                    <input id="new_todo" value={"hi!"} />
                </form>
            </div>
        };

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");
        assert!(matches!(result, Err(LabelByTextError::NoElementFound(_))));
    }

    #[wasm_bindgen_test]
    fn text_not_found_when_search_term_not_found_in_label() {
        let rendered = test_render! {
            <div>
                <form>
                    <label for="new-todo">{ "What doesn't need to be done?" }</label>
                    <br />
                    <input id="new-todo" value={"hi!"} />
                </form>
            </div>
        };

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");

        assert!(matches!(result, Err(LabelByTextError::LabelNotFound(_))));
    }

    #[wasm_bindgen_test]
    fn input_value_change() {
        let label_text = "What needs to be done?";
        let rendered = test_render! {
            <>
                <label for="todo">{ "What needs to be done?" }</label>
                <input type="text" id="todo" value="" />
            </>
        };

        let new_value = "Gardening";

        let input: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        input.set_value(new_value);

        let input_after: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        assert_eq!(new_value, input_after.value());
        assert_eq!(input, input_after);
    }
}
