/**
Asserts that a [`Node`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)'s
text content is equal to the expected String value (using [`PartialEq`](std::cmp::PartialEq)).

If you want to take into account styling then you will want to use [`assert_inner_text`].

# Examples
The expected text content is the first argument and the node is the second:
```no_run
# use hyphae::assert_text_content;
# use web_sys::Node;
# fn test_assert_text_context(node: Node) {
let node: Node = //.. some function to get Node with text content with "Hello, World!"
    # node;
assert_text_content!("Hello, World!", node);
# }
```
A second version is available to add a custom panic message when the equality fails:
```no_run
# use hyphae::assert_text_content;
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

/**
Asserts that a [`HtmlElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html)'s
inner text is equal to the expected String value (using [`PartialEq`](std::cmp::PartialEq)).

If you want to exclude styling then you will want to use [`assert_text_content`].

# Examples
The expected inner text is the first argument and the HtmlElement is the second:
```no_run
# use hyphae::assert_inner_text;
# use web_sys::HtmlElement;
# fn test_assert_inner_text(element: HtmlElement) {
let element: HtmlElement = //.. some function to get Element with inner text of "Hello, World!"
    # element;
assert_inner_text!("Hello, World!", element);
# }
```
A second version is available to add a custom panic message when the equality fails:
```no_run
# use hyphae::assert_inner_text;
# use web_sys::HtmlElement;
# fn test_assert_inner_text(element: HtmlElement) {
let element: HtmlElement = //.. some function to get HtmlElement with inner text of "Hello, World!"
 # element;
assert_inner_text!("Hello, Rust!", element, "oops, that isn't correct!");
# }
```
*/
#[macro_export]
macro_rules! assert_inner_text {
    ($expected: expr, $element:expr $(,$($arg:tt)+)?) => {
        assert_eq!($expected.to_string(), $element.inner_text() $(, $($arg)+)?);
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use hyphae::prelude::*;
    use hyphae_utils::make_element_with_html_string;

    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn assert_div_has_inner_text() {
        let render = QueryElement::new();
        render.set_inner_html("<div id=\"mydiv\">div inner text!</div>");

        let result = render
            .query_selector("#mydiv")
            .unwrap()
            .unwrap()
            .unchecked_into::<HtmlElement>();
        assert_inner_text!("div inner text!", result);
    }

    #[wasm_bindgen_test]
    fn inner_text_of_parent_element() {
        let rendered: QueryElement = make_element_with_html_string(
            "
            <div>
                Hello,
                <strong> World!</strong>
            </div>
        ",
        )
        .into();
        assert_text_content!("Hello, World!", rendered);
    }

    #[wasm_bindgen_test]
    fn inner_text_will_ignore_hidden_elements() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div style="display:none">
                This is hidden
            </div>
        "#,
        )
        .into();
        assert_inner_text!("", rendered);
    }

    #[wasm_bindgen_test]
    fn t() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <div>
                <span>
                    <strong>1</strong>
                    <span> item</span>
                </span>
            </div>
        "#,
        )
        .into();
        assert_text_content!("1 item", rendered);
    }

    #[wasm_bindgen_test]
    fn assert_div_has_text_content() {
        let render = QueryElement::new();
        render.set_inner_html("<div id=\"mydiv\">div text content!</div>");

        let result = render.query_selector("#mydiv").unwrap().unwrap();
        assert_text_content!("div text content!", result);
    }

    #[wasm_bindgen_test]
    fn text_content_does_include_child_text_content() {
        let render = QueryElement::new();
        render
            .set_inner_html("<div id=\"mydiv\">text content <strong>is broken up!</strong></div>");

        let result = render.query_selector("#mydiv").unwrap().unwrap();
        assert_text_content!("text content is broken up!", result);
    }
}
