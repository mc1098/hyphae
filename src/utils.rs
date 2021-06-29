use wasm_bindgen::JsValue;
use web_sys::{Element, Node};

#[inline]
pub(crate) fn is_text_content_from_query_select(
    result: Result<Option<Element>, JsValue>,
    s: &str,
) -> bool {
    result
        .ok()
        .flatten()
        .map(|e| has_text_content(&e, s))
        .unwrap_or_default()
}

#[inline]
pub(crate) fn has_text_content(node: &Node, s: &str) -> bool {
    node.text_content()
        .map(|text| text == s)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::Document;
    wasm_bindgen_test_configure!(run_in_browser);

    fn create_element_with_text_content(element_name: &str, text: &str) -> Element {
        let document = Document::new().unwrap();
        let element = document.create_element(element_name).unwrap();
        element.set_text_content(text.into());
        element
    }

    #[wasm_bindgen_test]
    fn node_has_text_content() {
        let content = "Hello, World!";
        let div = create_element_with_text_content("div", content);

        assert!(has_text_content(&div, content));
    }

    #[wasm_bindgen_test]
    fn node_does_not_have_text_content() {
        let button = create_element_with_text_content("button", "");

        assert!(!has_text_content(&button, "Hello, World!"));
    }
}
