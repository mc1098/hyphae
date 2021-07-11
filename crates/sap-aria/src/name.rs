use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    window, Element, HtmlAreaElement, HtmlElement, HtmlImageElement, HtmlInputElement,
    HtmlTextAreaElement, Node,
};

fn id_refs_to_query_string(id_refs: String) -> String {
    id_refs
        .split_whitespace()
        .map(|id| format!("#{}", id))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(feature = "Unsupported")]
fn get_css_pseudo_elt_content(element: &HtmlElement, pseudo: &str) -> Option<String> {
    let style = window()?
        .get_computed_style_with_pseudo_elt(element, pseudo)
        .ok()
        .flatten()?;
    style.get_property_value("content").ok()
}

#[inline]
fn is_hidden_and_no_aria_idref_label(node: &Node) -> Result<bool, JsValue> {
    if let Some(element) = node.dyn_ref::<HtmlElement>() {
        let style_hidden = if let Some(style) = window().unwrap().get_computed_style(element)? {
            style.get_property_value("display")? == "none"
                || style.get_property_value("visibility")? == "hidden"
        } else {
            false
        };

        let aria_hidden = if let Some(at_value) = element.get_attribute("aria-hidden") {
            match at_value.as_str() {
                "true" => true,
                "false" => return Ok(false),
                _ => false,
            }
        } else {
            false
        };

        Ok((aria_hidden || style_hidden || element.hidden())
            && element.get_attribute("aria-labelledby").is_none())
    } else {
        Ok(false)
    }
}

/// Embedded control as defined by [W3C](https://www.w3.org/TR/2014/REC-html5-20141028/embedded-content-0.html)
#[inline]
#[allow(dead_code)]
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

#[inline]
fn add_node_to_traversed(node: &Node, traversed: &mut Vec<Node>) {
    traversed.push(node.clone());
}

#[inline]
fn is_node_part_of_traversal(node: &Node, traversed: &[Node]) -> bool {
    traversed.contains(node)
}

fn get_children_accessible_names(
    node: &Node,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    let children = node.child_nodes();
    let mut names = vec![];
    for i in 0..children.length() {
        let child = children.get(i).unwrap();
        if !is_node_part_of_traversal(&child, traversed) {
            add_node_to_traversed(&child, traversed);
            let name = element_accessible_name_impl(&child, traversed, is_albt)?;
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    Ok(names.join(" "))
}

pub fn element_accessible_name(node: &Node) -> Result<String, JsValue> {
    let mut traversed = vec![];
    element_accessible_name_impl(node, &mut traversed, false)
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

/**
Recursive function to calculate a nodes accessible name.

aria-labelledby traversal (albt)

NOTE: Pseudo elements are part of the standard but some browsers seem to ignore them and even my
screen reader does.
*/
#[allow(dead_code)]
fn element_accessible_name_impl(
    node: &Node,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    let mut accumlated_text = String::new();

    if is_hidden_and_no_aria_idref_label(node)? {
        return Ok(accumlated_text);
    }

    if !is_presentational(node) {
        if !is_albt {
            if let Some(labelled_by) = node
                .dyn_ref::<Element>()
                .and_then(|element| element.get_attribute("aria-labelledby"))
            {
                add_node_to_traversed(node, traversed);
                let selector_ids = id_refs_to_query_string(labelled_by);
                let labels = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .query_selector_all(&selector_ids)?;
                for i in 0..labels.length() {
                    let label = labels.get(i).unwrap();
                    if !is_node_part_of_traversal(&label, traversed) {
                        add_node_to_traversed(&label, traversed);
                        accumlated_text
                            .push_str(&element_accessible_name_impl(&label, traversed, true)?);
                    }
                }
            }
        }

        if let Some(label) = node
            .dyn_ref::<Element>()
            .and_then(|element| element.get_attribute("aria-label"))
            .map(|value| value.trim().to_owned())
        {
            return if accumlated_text.is_empty() {
                Ok(label)
            } else {
                Ok(format!("{} {}", label, accumlated_text))
            };
        }

        if let Some(node) = node.dyn_ref::<Element>() {
            // Text alternative info: https://www.w3.org/TR/html-aam-1.0/#accessible-name-and-description-computation
            let name = match node.tag_name().to_lowercase().as_str() {
                "input" => text_alternative_input(node.unchecked_ref(), traversed, is_albt)?,
                "textarea" => text_alternative_label_title_placeholder(node, traversed, is_albt)?,
                "button" => text_alternative_subtree_title(node, traversed, is_albt)?,
                "fieldset" => {
                    text_alternative_first_child_subtree_title(node, "legend", traversed, is_albt)?
                }
                "output" => text_alternative_subtree_title(node, traversed, is_albt)?,
                "select" | "datalist" | "optgroup" | "option" | "keygen" | "progress" | "meter"
                | "legend" => text_alternative_label_title(node, traversed, is_albt)?,
                "summary" => text_alternative_summary(node, traversed, is_albt)?,
                "figure" => text_alternative_first_child_subtree_title(
                    node,
                    "figcaption",
                    traversed,
                    is_albt,
                )?,
                "img" => {
                    text_alternative_alt_title!(node as HtmlImageElement)
                }
                "table" => {
                    text_alternative_first_child_subtree_title(node, "caption", traversed, is_albt)?
                }
                "a" => text_alternative_subtree_title(node, traversed, is_albt)?,
                "area" => text_alternative_alt_title!(node as HtmlAreaElement),
                _ => get_children_accessible_names(node, traversed, is_albt)?,
            };
            accumlated_text.push_str(&name);
        }
    }

    if is_presentational(node) {
        accumlated_text.push_str(&get_children_accessible_names(node, traversed, is_albt)?);
    }

    if Node::TEXT_NODE == node.node_type() {
        accumlated_text.push_str(&node.text_content().unwrap_or_default().trim().to_owned());
    }

    Ok(accumlated_text)
}

fn text_alternative_input(
    element: &HtmlInputElement,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    match element.type_().as_str() {
        "text" | "password" | "search" | "tel" | "url" => {
            text_alternative_label_title_placeholder(element, traversed, is_albt)
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
                // W3C says this should be 'Submit Query' however browsers seems to use 'Submit'
                Ok("Submit".to_owned())
            } else {
                Ok(name)
            }
        }
        "range" | "number" => Ok(element
            .get_attribute("aria-valuetext")
            .or_else(|| element.get_attribute("aria-valuenow"))
            .unwrap_or_else(|| element.value())),
        "checkbox" => text_alternative_label_title(element, traversed, is_albt),
        _ => Ok(String::new()),
    }
}

fn text_alternative_summary(
    element: &Element,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    let name = text_alternative_subtree_title(element, traversed, is_albt)?;

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
    element: &Element,
    child_tag: &str,
    traversed: &mut Vec<Node>,
    is_albt: bool,
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
            name = get_children_accessible_names(&child, traversed, is_albt)?;
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
    element: &Element,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    if !element.id().is_empty() {
        let labels = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector_all(&format!("label[for={}]", element.id()))?;
        let mut name = String::new();
        for i in 0..labels.length() {
            let label = labels.get(i).and_then(|n| n.dyn_into().ok()).unwrap();
            let label_name = element_accessible_name_impl(&label, traversed, is_albt)?;
            if !label_name.is_empty() {
                name.push_str(&label_name);
            }
        }
        if !name.is_empty() {
            return Ok(name);
        }
    }

    Ok(title_or_default(element))
}

fn text_alternative_label_title_placeholder(
    element: &Element,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    let name = text_alternative_label_title(element, traversed, is_albt)?;

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
    element: &Element,
    traversed: &mut Vec<Node>,
    is_albt: bool,
) -> Result<String, JsValue> {
    let subtree = get_children_accessible_names(element, traversed, is_albt)?;
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

#[cfg(test)]
mod tests {

    use std::ops::Deref;

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    struct ElementWrapper(Element);

    impl Deref for ElementWrapper {
        type Target = Element;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Drop for ElementWrapper {
        fn drop(&mut self) {
            self.0.remove()
        }
    }

    fn make_element_with_html_string(inner_html: &str) -> ElementWrapper {
        let document = window().unwrap().document().unwrap();
        let div = document.create_element("div").unwrap();
        // remove \n & \t which are just formatting to avoid text nodes being added
        div.set_inner_html(
            &inner_html
                .chars()
                .filter(|c| *c != '\n' && *c != '\t')
                .collect::<String>(),
        );

        document.body().unwrap().append_child(&div).unwrap();
        ElementWrapper(div)
    }

    #[wasm_bindgen_test]
    fn label_container() {
        let element = make_element_with_html_string(
            "<label for=\"user-password\">
                Password:
                <input id=\"user-password\" type=\"password\" />
            </label>",
        );

        let input = element.query_selector("#user-password").unwrap().unwrap();

        assert_eq!("Password:", element_accessible_name(&input).unwrap());
    }

    #[wasm_bindgen_test]
    fn checkbox_name_from_label() {
        let element = make_element_with_html_string(
            "<input id=\"myinput\" type=\"checkbox\"/>
            <label for=\"myinput\">My Input!</label>",
        );

        let input = element.query_selector("#myinput").unwrap().unwrap();

        assert_eq!("My Input!", element_accessible_name(&input).unwrap());
    }

    #[wasm_bindgen_test]
    fn simple_aria_label() {
        let element = make_element_with_html_string(
            "<input id=\"my_name\" aria-labelledby=\"my_name\" aria-label=\"Your name is?\" type=\"text\" />",
        );

        assert_eq!("Your name is?", element_accessible_name(&element).unwrap());
    }

    #[wasm_bindgen_test]
    fn recursive_button_name() {
        let element = make_element_with_html_string(
            r#"
            <button>
                <span class="action">Delete</span>
                <span class="profile">
                    <img src="pict.jpg" alt="Profile" />
                    Matt Tress
                </span>
            </button>
            "#,
        );

        assert_eq!(
            "Delete Profile Matt Tress",
            element_accessible_name(&element).unwrap()
        );

        let element = make_element_with_html_string(
            r#"
            <button>
                <span class="action">Delete</span>
                <span class="profile" aria-label="all records of Matt Tress" >
                    <img src="pict.jpg" alt="Profile" />
                    Matt Tress
                </span>
            </button>
            "#,
        );

        assert_eq!(
            "Delete all records of Matt Tress",
            element_accessible_name(&element.first_element_child().unwrap()).unwrap()
        );

        let element = make_element_with_html_string(
            r#"
            <button aria-label="Remove all trace of Matt Tress from the face of the Earth">
                <span class="action">Delete</span>
                <span class="profile" aria-label="all records of Matt Tress" >
                    <img src="pict.jpg" alt="Profile" />
                    Matt Tress
                </span>
            </button>
            "#,
        );

        assert_eq!(
            "Remove all trace of Matt Tress from the face of the Earth",
            element_accessible_name(&element).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn ignore_second_pass_of_aria_labelledby() {
        let element = make_element_with_html_string(
            r#"
            <div id="parentId">
                <button aria-labelledby="parentId" aria-label="Remove event:">X</button>
                <span class="event">Blindfolded Dart Throwing Contest</span>
            </div>
            "#,
        );

        assert_eq!(
            "Remove event: Blindfolded Dart Throwing Contest",
            element_accessible_name(&element).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn aria_labelledby_only_follow_once() {
        let element = make_element_with_html_string(
            "<div id=\"e11\" aria-labelledby=\"e13\"></div>
                <div id=\"e12\" aria-labelledby=\"e11\"></div>
                <div id=\"e13\">hello</div>
            ",
        );
        let nodes = element.child_nodes();

        assert_eq!(
            "hello",
            element_accessible_name(&nodes.item(0).unwrap()).unwrap()
        );

        assert_eq!(
            "",
            element_accessible_name(&nodes.item(1).unwrap()).unwrap()
        )
    }

    #[wasm_bindgen_test]
    fn multiple_aria_labelled_by() {
        // need to avoid whitespace between elements in string
        let element = make_element_with_html_string(&format!(
            "{}{}",
            r#"<a id="file_row1" href="./files/Documentation.pdf">Documentation.pdf</a>"#,
            r#"<span role="button" tabindex="0" id="del_row1" aria-label="Delete" aria-labelledby="del_row1 file_row1"></span>"#,
        ));

        let nodes = element.child_nodes();

        assert_eq!(
            "Delete Documentation.pdf",
            element_accessible_name(&nodes.get(1).unwrap()).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn css_display_none() {
        let element = make_element_with_html_string(
            "<div id=\"descId\">
                <span style=\"display:none;\">
                    Choose the country where you currently reside.
                </span>
            </div>",
        );

        assert_eq!("", element_accessible_name(&element).unwrap());
    }

    #[wasm_bindgen_test]
    fn aria_hidden() {
        let element = make_element_with_html_string(
            "<div id=\"parentId\">
                Email address:
                <input aria-labelledby=\"parentId\" type=\"text\" />
                <div class=\"validationError\" aria-hidden=\"true\" >
                    Error: A valid email address is required.
                </div>
            </div>",
        );

        assert_eq!("Email address:", element_accessible_name(&element).unwrap());

        drop(element);

        let element = make_element_with_html_string(
            "<div id=\"parentId\">
                Email address:
                <input aria-labelledby=\"parentId\" type=\"text\" />
                <div class=\"validationError\" style=\"display:none;\" aria-hidden=\"false\" >
                    Error: A valid email address is required.
                </div>
            </div>",
        );

        assert_eq!(
            "Email address: Error: A valid email address is required.",
            element_accessible_name(&element).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn css_visibility_hidden() {
        let element = make_element_with_html_string(
            "<input type=\"text\" />
                <div style=\"visibility:hidden;\">
                    <span>
                        Choose the country where you currently reside
                    </span>
                </div>",
        );

        assert_eq!("", element_accessible_name(&element).unwrap());

        let element = make_element_with_html_string(
            "<div id=\"parentId\">
                Email address:
                <input aria-labelledby=\"parentId\" type=\"text\" />
                <div class=\"validationError\" style=\"visibility:hidden;\" >
                    Error: A valid email address is required.
                </div>
            </div>",
        );

        assert_eq!("Email address:", element_accessible_name(&element).unwrap());
    }

    #[wasm_bindgen_test]
    fn ignore_presentation_or_role_none() {
        let element = make_element_with_html_string(
            "<button>
                <div aria-label=\"This is the best!\" role=\"presentation\">
                    <span>Wow!</span>
                </div>
            </button>",
        );

        assert_eq!("Wow!", element_accessible_name(&element).unwrap(),);
    }

    #[wasm_bindgen_test]
    fn checkbox_with_text_input() {
        let element = make_element_with_html_string(
            "<div role=\"checkbox\" aria-checked=\"false\">
                Flash the screen 
                <span role=\"textbox\" aria-multiline=\"false\"> 5 </span>
                times
            </div>",
        );

        assert_eq!(
            "Flash the screen 5 times",
            element_accessible_name(&element).unwrap()
        );
    }

    #[cfg(feature = "Unsupported")]
    #[wasm_bindgen_test]
    fn pseudo_elements() {
        let element = make_element_with_html_string(
            "<div>
                <a id=\"mylink\" href=\"https://google.com\" target=\"_blank\"> Search </a>
            </div>",
        );

        window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("head")
            .unwrap()
            .unwrap()
            .set_inner_html(
                "
            <style type='text/css'>
                #mylink:focus:after, #mylink:hover:after {
                    height: auto; width: auto;
                    position: absolute;
                    z-index: 1;
                    margin-top: 20px;
                    background-color: white;
                    color: blue;
                    font-size: 10px;
                    content: ' - Opens in new window ';
                }
            </style>
        ",
            );

        assert_eq!(
            "Search - Opens in new window",
            element_accessible_name(&element).unwrap()
        );
    }
}
