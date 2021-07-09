/**
Asserts that a [`Node`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)'s
text content is equal to the expected String value (using [`PartialEq`](std::cmp::PartialEq)).

# Examples
The expected text content is the first argument and the node is the second:
```no_run
# use sap::assert_text_content;
# use web_sys::Node;
# fn test_assert_text_context(node: Node) {
let node: Node = //.. some function to get Node with text content with "Hello, World!"
    # node;
assert_text_content!("Hello, World!", node);
# }
```
A second version is available to add a custom panic message when the equality fails:
```no_run
# use sap::assert_text_content;
# use web_sys::Node;
# fn test_assert_text_content(node: Node) {
let node: Node = //.. some function to get Node with text content with "Hello, World!"
 # node;
assert_text_content!("Hello, Rust!", node, "oops, that isn't correct!");
# }
```
*/
#[macro_export]
macro_rules! assert_text_content {
    ($expected: expr, $element:expr $(,)?) => {
        if let Some(text) = $element.text_content() {
            assert_eq!($expected.to_string(), text);
        } else {
            panic!("Node does not have any text content");
        }
    };
    ($expected: expr, $element:expr, $($arg:tt)+) => {
        if let Some(text) = $element.text_content() {
            assert_eq!($expected.to_string(), text, $($arg)+);
        } else {
            panic!($($arg)+);
        }

    };
}

#[cfg(all(test, feature = "Yew"))]
mod tests {

    use crate::prelude::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::Element;

    #[wasm_bindgen_test]
    fn assert_div_has_text_content() {
        let rendered = test_render! {
            <div id="mydiv">
                { "div text content!" }
            </div>
        };

        let result = rendered.get_by_id::<Element>("mydiv").unwrap();
        assert_text_content!("div text content!", result);
    }

    #[wasm_bindgen_test]
    fn text_content_does_include_child_text_content() {
        let rendered = test_render! {
            <div id="mydiv">
                { "text content " }
                <strong>
                    { "is broken up!" }
                </strong>
            </div>
        };

        let not_found = rendered.get_by_text::<Element>("text content is broken up!");
        assert!(not_found.is_none());

        let result = rendered.get_by_id::<Element>("mydiv").unwrap();
        assert_text_content!("text content is broken up!", result);
    }
}
