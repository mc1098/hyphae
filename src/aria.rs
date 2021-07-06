use std::collections::HashSet;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    window, Element, HtmlAreaElement, HtmlButtonElement, HtmlElement, HtmlImageElement,
    HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Node, NodeList,
};

#[allow(dead_code)]
enum NodeType {
    Element,
    Attribute,
    Text,
    CdataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

#[allow(clippy::clippy::from_over_into)]
impl Into<u16> for NodeType {
    fn into(self) -> u16 {
        match self {
            NodeType::Element => 1,
            NodeType::Attribute => 2,
            NodeType::Text => 3,
            NodeType::CdataSection => 4,
            NodeType::ProcessingInstruction => 7,
            NodeType::Comment => 8,
            NodeType::Document => 9,
            NodeType::DocumentType => 10,
            NodeType::DocumentFragment => 11,
        }
    }
}

impl PartialEq<u16> for NodeType {
    fn eq(&self, other: &u16) -> bool {
        let this = match self {
            NodeType::Element => 1,
            NodeType::Attribute => 2,
            NodeType::Text => 3,
            NodeType::CdataSection => 4,
            NodeType::ProcessingInstruction => 7,
            NodeType::Comment => 8,
            NodeType::Document => 9,
            NodeType::DocumentType => 10,
            NodeType::DocumentFragment => 11,
        };
        this == *other
    }
}

fn id_refs_to_query_string(id_refs: String) -> String {
    id_refs
        .split_whitespace()
        .fold(String::new(), |mut acc, id| {
            acc.push('#');
            acc.push_str(id);
            acc.push(' ');
            acc
        })
}

fn textbox_value(element: &HtmlElement) -> Option<String> {
    if let Some(aria_multiline) = element.get_attribute("aria-multiline") {
        if aria_multiline == "true" {
            return Some(element.inner_text());
        }
    }
    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        match input.type_().as_str() {
            "email" | "tel" | "text" | "url" => return Some(input.value()),
            _ => {}
        }
    }

    element
        .dyn_ref::<HtmlTextAreaElement>()
        .map(|text_area| text_area.value())
}

fn button_text_alternative(element: &HtmlElement) -> Option<String> {
    if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
        return Some(button.inner_text());
    }

    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        match input.type_().as_str() {
            "button" | "reset" | "submit" => return Some(input.value()),
            "image" => return Some(input.alt()),
            _ => todo!(),
        }
    }

    if element.tag_name() == "summary" {
        element.text_content()
    } else {
        None
    }
}

fn combobox_or_listbox_text_alternative(element: &HtmlElement) -> Option<String> {
    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        if input.get_attribute("list").is_some() {
            match input.type_().as_str() {
                "text" | "search" | "tel" | "url" | "email" => return Some(input.value()),
                _ => {}
            }
        }
    }

    // Select is either combobox or listbox so get value if this cast works
    element
        .dyn_ref::<HtmlSelectElement>()
        .map(|select| select.value())
}

fn range_value(element: &HtmlElement) -> Option<String> {
    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        match input.type_().as_str() {
            "range" | "number" => {
                return input
                    .get_attribute("aria-valuetext")
                    .or_else(|| input.get_attribute("aria-valuenow"))
                    .or_else(|| Some(input.value()));
            }
            _ => {}
        }
    }
    None
}

fn get_css_pseudo_elt_content(element: &HtmlElement, pseudo: &str) -> Option<String> {
    let style = window()?
        .get_computed_style_with_pseudo_elt(element, pseudo)
        .ok()
        .flatten()?;
    style.get_property_value("content").ok()
}

fn supports_name_from_content(element: &HtmlElement) -> bool {
    match element.tag_name().as_str() {
        "button" | "td" | "th" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => true,
        "input" => {
            todo!()
        }
        _ => false,
    }
}

#[inline]
fn is_hidden_and_no_aria_idref_label(node: &Node) -> bool {
    if let Some(element) = node.dyn_ref::<HtmlElement>() {
        element.hidden() && element.get_attribute("aria-labelledby").is_none()
    } else {
        false
    }
}

/// Embedded control as defined by [W3C](https://www.w3.org/TR/2014/REC-html5-20141028/embedded-content-0.html)
#[inline]
fn is_element_an_embedded_control(node: &Node) -> bool {
    if let Some(element) = node.dyn_ref::<Element>() {
        matches!(
            element.tag_name().as_str(),
            "img"
                | "iframe"
                | "embed"
                | "object"
                | "param"
                | "video"
                | "audio"
                | "source"
                | "track"
                | "map"
                | "area"
        )
    } else {
        false
    }
}

/// True when an element has either of the following role values:
/// - presentation
/// - none
#[inline]
fn is_presentational(node: &Node) -> bool {
    node.dyn_ref::<Element>()
        .and_then(|element| element.get_attribute("role"))
        .map(|value| matches!(value.as_str(), "presentation" | "none"))
        .unwrap_or_default()
}

fn get_children_accessible_names(
    root: &Element,
    children: NodeList,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let mut names = vec![];
    for i in 0..children.length() {
        let child = children.get(i).unwrap();
        let name = get_element_accessible_name_impl(root, &child, labelledby_traversal)?;
        if !name.is_empty() {
            names.push(name);
        }
    }
    Ok(names.join(" "))
}

#[allow(dead_code)]
pub(crate) fn get_element_accessible_name(root: &Element, node: &Node) -> Result<String, JsValue> {
    get_element_accessible_name_impl(root, node, false)
}

macro_rules! text_alternative_alt_title {
    ($element:ident as HtmlAreaElement) => {
        match $element.dyn_ref::<HtmlAreaElement>().map(|e| e.alt()) {
            Some(alt) if alt.is_empty() => title_or_default($element),
            Some(alt) => alt,
            _ => String::new(),
        }
    };
    ($element:ident as HtmlImageElement) => {
        match $element.dyn_ref::<HtmlImageElement>().map(|e| e.alt()) {
            Some(alt) if alt.is_empty() => title_or_default($element),
            Some(alt) => alt,
            _ => String::new(),
        }
    };
    ($element:ident as HtmlInputElement) => {
        match $element.dyn_ref::<HtmlInputElement>().map(|e| e.alt()) {
            Some(alt) if alt.is_empty() => title_or_default($element),
            Some(alt) => alt,
            _ => String::new(),
        }
    };
}

#[allow(dead_code)]
fn get_element_accessible_name_impl(
    root: &Element,
    node: &Node,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let mut accumlated_text = String::new();

    if is_hidden_and_no_aria_idref_label(node) {
        return Ok(accumlated_text);
    }

    if !labelledby_traversal {
        if let Some(labelled_by) = node
            .dyn_ref::<Element>()
            .and_then(|element| element.get_attribute("aria-labelledby"))
        {
            let selector_ids = id_refs_to_query_string(labelled_by);
            let labels = root.query_selector_all(&selector_ids)?;
            for i in 0..labels.length() {
                let label = labels.get(i).unwrap();
                if &label != node {
                    accumlated_text
                        .push_str(&get_element_accessible_name_impl(root, &label, true)?);
                }
            }
        }
    }

    if let Some(label) = node
        .dyn_ref::<Element>()
        .and_then(|element| element.get_attribute("aria-label"))
        .map(|value| value.trim().to_owned())
    {
        // accumlated_text.push_str("aria label found!");
        return Ok(label);
        // Not sure why this check is in the standard? - the next section always looks for aria-label..
        // if !label.is_empty() && (depth == 1 && !is_element_an_embedded_control(&node)) {
        //     return Ok(label);
        // }
    }

    if !is_presentational(node) {
        if let Some(node) = node.dyn_ref::<Element>() {
            // Text alternative info: https://www.w3.org/TR/html-aam-1.0/#accessible-name-and-description-computation
            let name = match node.tag_name().to_lowercase().as_str() {
                "input" => {
                    text_alternative_input(root, node.unchecked_ref(), labelledby_traversal)?
                }
                "textarea" => {
                    text_alternative_label_title_placeholder(root, node, labelledby_traversal)?
                }
                "button" => text_alternative_subtree_title(root, node, labelledby_traversal)?,
                "fieldset" => text_alternative_first_child_subtree_title(
                    root,
                    node,
                    "legend",
                    labelledby_traversal,
                )?,
                "output" => text_alternative_subtree_title(root, node, labelledby_traversal)?,
                "select" | "datalist" | "optgroup" | "option" | "keygen" | "progress" | "meter"
                | "legend" => text_alternative_label_title(root, node, labelledby_traversal)?,
                "label" => node.text_content().unwrap_or_default(),
                "summary" => text_alternative_summary(root, node, labelledby_traversal)?,
                "figure" => text_alternative_first_child_subtree_title(
                    root,
                    node,
                    "figcaption",
                    labelledby_traversal,
                )?,
                "img" => {
                    text_alternative_alt_title!(node as HtmlImageElement)
                }
                "table" => text_alternative_first_child_subtree_title(
                    root,
                    node,
                    "caption",
                    labelledby_traversal,
                )?,
                "a" => text_alternative_subtree_title(root, node, labelledby_traversal)?,
                "area" => text_alternative_alt_title!(node as HtmlAreaElement),
                _ => {
                    get_children_accessible_names(root, node.child_nodes(), labelledby_traversal)?
                    // title_or_default(node)
                }
            };
            accumlated_text.push_str(&name);
        }
    }
    //(E)
    //(F)
    //(G)
    if NodeType::Text == node.node_type() {
        accumlated_text.push_str(&node.text_content().unwrap_or_default().trim().to_owned());
    }
    //(H)
    //(I)

    Ok(accumlated_text)
}

fn text_alternative_input(
    root: &Element,
    element: &HtmlInputElement,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    match element.type_().as_str() {
        "text" | "password" | "search" | "tel" | "url" => {
            text_alternative_label_title_placeholder(root, element, labelledby_traversal)
        }
        "button" => {
            if element.value().is_empty() {
                Ok(title_or_default(element))
            } else {
                Ok(element.value())
            }
        }
        "submit" | "reset" => {
            if element.value().is_empty() {
                Ok(element.type_())
            } else {
                Ok(element.value())
            }
        }
        "image" => {
            let name = text_alternative_alt_title!(element as HtmlInputElement);
            if name.is_empty() {
                // W3C says this should be 'Submit Query' however browsers seems to just use 'Submit'
                Ok("Submit".to_owned())
            } else {
                Ok(name)
            }
        }
        _ => Ok(String::new()),
    }
}

fn text_alternative_summary(
    root: &Element,
    element: &Element,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let name = text_alternative_subtree_title(root, element, labelledby_traversal)?;

    if !name.is_empty() {
        return Ok(name);
    }

    if element
        .parent_node()
        .filter(|parent| parent.unchecked_ref::<Element>().tag_name() == "details")
        .is_some()
    {
        // return empty string
        Ok(name)
    } else {
        Ok("details".to_owned())
    }
}

fn text_alternative_first_child_subtree_title(
    root: &Element,
    element: &Element,
    child_tag: &str,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let mut name = String::new();
    let children = element.child_nodes();
    for i in 0..children.length() {
        let child = children.get(i).unwrap();
        if child
            .dyn_ref::<Element>()
            .map(|element| element.tag_name() == child_tag)
            .unwrap_or_default()
        {
            name = get_children_accessible_names(root, child.child_nodes(), labelledby_traversal)?;
            if !name.is_empty() {
                return Ok(name);
            } else {
                return Ok(title_or_default(element));
            }
        }
    }
    Ok(name)
}

fn text_alternative_label_title(
    root: &Element,
    element: &Element,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    if !element.id().is_empty() {
        let labels = root.query_selector_all(&format!("#{}", element.id()))?;
        let mut name = String::new();
        for i in 0..labels.length() {
            let label = labels.get(i).and_then(|n| n.dyn_into().ok()).unwrap();
            let label_name = get_element_accessible_name_impl(root, &label, labelledby_traversal)?;
            if !label_name.is_empty() {
                name.push_str(&format!("{} ", label_name));
            }
        }
        if !name.is_empty() {
            return Ok(name);
        }
    }

    Ok(title_or_default(element))
}

fn text_alternative_label_title_placeholder(
    root: &Element,
    element: &Element,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let name = text_alternative_label_title(root, element, labelledby_traversal)?;

    if name.is_empty() {
        let input = element
            .dyn_ref::<HtmlInputElement>()
            .map(|e| e.placeholder());
        let text_area = element
            .dyn_ref::<HtmlTextAreaElement>()
            .map(|e| e.placeholder());
        Ok(input.or(text_area).unwrap_or_default())
    } else {
        Ok(name)
    }
}

fn text_alternative_subtree_title(
    root: &Element,
    element: &Element,
    labelledby_traversal: bool,
) -> Result<String, JsValue> {
    let subtree = get_children_accessible_names(root, element.child_nodes(), labelledby_traversal)?;
    if subtree.is_empty() {
        let title = element
            .dyn_ref::<HtmlElement>()
            .map(|e| e.title())
            .unwrap_or_default();
        Ok(title)
    } else {
        Ok(subtree)
    }
}

#[inline]
fn title_or_default(element: &Element) -> String {
    element
        .dyn_ref::<HtmlElement>()
        .map(|e| e.title())
        .unwrap_or_default()
}

#[cfg(all(test, feature = "Yew"))]
mod tests {

    use crate::{test_render, TestRender};

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn simple_aria_label() {
        let rendered = test_render! {
            <input id="my_name" aria-labelledby="my_name" aria-label="Your name is?" type="text" />
        };

        let element: &HtmlElement = rendered.dyn_ref().unwrap();

        assert_eq!(
            "Your name is?",
            get_element_accessible_name(element, &element.first_element_child().unwrap()).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn recursive_button_name() {
        let rendered = test_render! {
            <button>
                <span class="action">{ "Delete" }</span>
                <span class="profile">
                    <img src="pict.jpg" alt="Profile" />
                    { "Bryan Garaventa" }
                </span>
            </button>
        };

        assert_eq!(
            "Delete Profile Bryan Garaventa",
            get_element_accessible_name(&rendered, &rendered.first_element_child().unwrap())
                .unwrap()
        );

        let rendered = test_render! {
            <button>
                <span class="action">{ "Delete" }</span>
                <span class="profile" aria-label="all records of Bryan Garaventa">
                    <img src="pict.jpg" alt="Profile" />
                    { "Bryan Garaventa" }
                </span>
            </button>
        };

        // web_sys::console::log_1(&rendered.inner_html().into());

        assert_eq!(
            "Delete all records of Bryan Garaventa",
            get_element_accessible_name(&rendered, &rendered.first_element_child().unwrap())
                .unwrap()
        );

        let rendered = test_render! {
            <button aria-label="Remove all trace of Bryan Garaventa from the face of the Earth">
                <span class="action">{ "Delete" }</span>
                <span class="profile" aria-label="all records of Bryan Garaventa">
                    <img src="pict.jpg" alt="Profile" />
                    { "Bryan Garaventa" }
                </span>
            </button>
        };

        assert_eq!(
            "Remove all trace of Bryan Garaventa from the face of the Earth",
            get_element_accessible_name(&rendered, &rendered.first_element_child().unwrap())
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn ignore_second_pass_of_aria_labelledby() {
        let rendered = test_render! {
            <div id="parentId">
                <button aria-labelledby="parentId" aria-label="Remove event:">{ "X" }</button>
                <span class="event">{ "Blindfolded Dart Throwing Contest" }</span>
            </div>
        };

        assert_eq!(
            "Remove event: Blindfolded Dart Throwing Contest",
            get_element_accessible_name(&rendered, &rendered.first_element_child().unwrap())
                .unwrap()
        );
    }
}
