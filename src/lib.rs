use wasm_bindgen::{prelude::Closure, JsCast};

use web_sys::{
    Element, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Node, NodeFilter,
};
use yew::{html::Scope, prelude::*, utils::document, virtual_dom::VDiff};

#[non_exhaustive]
enum WhatToShow {
    ShowText,
}

#[allow(clippy::from_over_into)]
impl Into<u32> for WhatToShow {
    fn into(self) -> u32 {
        match self {
            WhatToShow::ShowText => 4,
        }
    }
}

pub enum LabelByTextError<'search> {
    LabelNotFound(&'search str),
    NoElementFound((&'search str, String)),
}

impl<'search> std::fmt::Debug for LabelByTextError<'search> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelByTextError::LabelNotFound(text) => {
                write!(f, "No label found with text: {}.", text)
            }
            LabelByTextError::NoElementFound((text, id)) => {
                write!(f, "Found a label with the text: '{}'\n however, no input was found with the id of '{}'", text, id)
            }
        }
    }
}

struct Empty;
impl Component for Empty {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}

pub struct Rendered {
    // parent_scope: AnyScope,
    parent_element: Element,
    // node: Html,
}

impl Rendered {
    pub fn render(mut html: Html) -> Self {
        let parent_scope = Scope::<Empty>::new(None).into();
        let parent_element = document().create_element("div").unwrap();

        html.apply(&parent_scope, &parent_element, NodeRef::default(), None);

        Self {
            parent_element,
            // parent_scope,
            // node: html,
        }
    }
}

impl Rendered {
    pub fn get_by_text<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast,
    {
        let mut filter = NodeFilter::new();
        let search = search.to_owned();

        let filter_on_text_value = move |node: Node| {
            node.text_content()
                .map(|text| text == search)
                .unwrap_or_default()
        };

        let cb = Closure::wrap(Box::new(filter_on_text_value) as Box<dyn Fn(Node) -> bool>);
        filter.accept_node(cb.as_ref().unchecked_ref());

        let walker = document()
            .create_tree_walker_with_what_to_show_and_filter(
                &self.parent_element,
                WhatToShow::ShowText.into(),
                Some(&filter),
            )
            .unwrap();

        walker
            .next_node()
            .unwrap()
            .and_then(|node| node.parent_element().and_then(|e| e.dyn_into().ok()))
    }

    pub fn get_by_placeholder_text<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast,
    {
        let holders = self
            .parent_element
            .query_selector_all(":placeholder-shown")
            .ok()?;

        for i in 0..holders.length() {
            let holder = holders.get(i)?;
            match holder.dyn_into::<HtmlInputElement>() {
                Ok(input) => {
                    if input.placeholder() == search {
                        return input.dyn_into().ok();
                    }
                }
                Err(node) => {
                    let text_area: HtmlTextAreaElement = node.dyn_into().ok()?;
                    if text_area.placeholder() == search {
                        return text_area.dyn_into().ok();
                    }
                }
            }
        }

        None
    }

    pub fn get_by_label_text<'search, T>(
        &self,
        search: &'search str,
    ) -> Result<T, LabelByTextError<'search>>
    where
        T: JsCast,
    {
        let labels = match self.parent_element.query_selector_all("label") {
            Ok(labels) => labels,
            Err(_) => return Err(LabelByTextError::LabelNotFound(search)),
        };

        for i in 0..labels.length() {
            let label = labels.get(i).unwrap();
            if label
                .text_content()
                .map(|text| text == search)
                .unwrap_or_default()
            {
                let label_element: Element = label.unchecked_into();
                if let Some(id) = label_element.get_attribute("for") {
                    return match self
                        .parent_element
                        .query_selector(&format!("#{}", id))
                        .unwrap()
                        .and_then(|element| element.dyn_into().ok())
                    {
                        Some(element) => Ok(element),
                        None => Err(LabelByTextError::NoElementFound((search, id))),
                    };
                } else {
                    return Err(LabelByTextError::LabelNotFound(search));
                };
            }
        }

        Err(LabelByTextError::LabelNotFound(search))
    }

    pub fn get_by_display_value<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast,
    {
        let displays = self
            .parent_element
            .query_selector_all("input, select, textarea")
            .ok()?;

        for i in 0..displays.length() {
            let display = displays.get(i)?;

            let display = match display.dyn_into::<HtmlInputElement>() {
                Ok(input) if input.value() == search => return input.dyn_into().ok(),
                Err(node) => node,
                _ => continue,
            };

            let display = match display.dyn_into::<HtmlTextAreaElement>() {
                Ok(area) if area.value() == search => return area.dyn_into().ok(),
                Err(node) => node,
                _ => continue,
            };

            if let Ok(select) = display.dyn_into::<HtmlSelectElement>() {
                if select.value() == search {
                    return select.dyn_into().ok();
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    struct Counter {
        count: usize,
        link: ComponentLink<Self>,
    }

    impl Component for Counter {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            Self { count: 0, link }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            self.count += 1;
            true
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    <p>{ format!("Count: {}", self.count) }</p>
                    <button onclick=self.link.callback(|_| ())>{ "Click me!" }</button>
                </div>
            }
        }
    }

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{HtmlElement, HtmlInputElement};

    use super::*;

    #[wasm_bindgen_test]
    fn button_click_test() {
        let rendered = Rendered::render(html! {
            <Counter />
        });

        let button: HtmlElement = rendered.get_by_text("Click me!").unwrap();
        button.click();

        let count = rendered.get_by_text::<Element>("Count: 1");
        assert!(count.is_some());

        button.click();
        let count = rendered.get_by_text::<Element>("Count: 2");
        assert!(count.is_some());
    }

    #[wasm_bindgen_test]
    fn input_value_change() {
        let label_text = "What needs to be done?";
        let rendered = Rendered::render(html! {
            <>
                <label for="todo">{ &label_text }</label>
                <input type="text" id="todo" value="" />
            </>
        });

        let new_value = "Gardening";

        let input: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        input.set_value(new_value);

        let input_after: HtmlInputElement = rendered.get_by_label_text(label_text).unwrap();
        assert_eq!(new_value, input_after.value());
        assert_eq!(input, input_after);
    }

    #[wasm_bindgen_test]
    fn get_input_by_display_value() {
        let rendered = Rendered::render(html! {
            <input type="text" id="greeting" value="Welcome" />
        });

        let input: HtmlInputElement = rendered.get_by_display_value("Welcome").unwrap();
        assert_eq!("greeting", input.id());
    }

    #[wasm_bindgen_test]
    fn text_search() {
        let test = Rendered::render(html! {
            <div>
                <div>
                    { "Hello, World!" }
                </div>
            </div>
        });

        let result = test.get_by_text::<Element>("Hello, World!");
        assert!(result.is_some());
    }

    #[wasm_bindgen_test]
    fn get_inputs_by_label_text() {
        let mut tests = vec![
            input_label_text(),
            input_label_text_different_parents(),
            input_label_text_label_after_input(),
        ];

        for test in tests.drain(..) {
            let test = Rendered::render(test);

            let result = test.get_by_label_text("What needs to be done?");

            let input: HtmlInputElement = result.unwrap();
            assert_eq!("hi!".to_owned(), input.value());
        }
    }

    // TODO:
    // [ ] - support aria-* attributes?

    #[wasm_bindgen_test]
    fn get_input_by_placeholder_text() {
        let rendered = Rendered::render(html! {
            <div>
                <input id="34" placeholder="Username" />
            </div>
        });

        let result: HtmlElement = rendered.get_by_placeholder_text("Username").unwrap();
        assert_eq!("34", result.id());
    }

    #[wasm_bindgen_test]
    fn get_textarea_by_placeholder_text() {
        let rendered = Rendered::render(html! {
            <div>
                <textarea id="23" placeholder="Enter bio here" />
            </div>
        });

        let result: HtmlElement = rendered.get_by_placeholder_text("Enter bio here").unwrap();
        assert_eq!("23", result.id());

        let rendered = Rendered::render(html! {
            <div>
                <textarea placeholder="Enter life story here" />
            </div>
        });

        assert!(rendered
            .get_by_placeholder_text::<Element>("Enter bio here")
            .is_none());
    }

    #[wasm_bindgen_test]
    fn no_element_found_when_id_and_for_do_not_match() {
        let rendered = Rendered::render(html! {
            <div>
                <form>
                    <label for="new-todoz">{ "What needs to be done?" }</label>
                    <br />
                    <input id="new-todo" value={"hi!"} />
                </form>
            </div>
        });

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");
        assert!(matches!(result, Err(LabelByTextError::NoElementFound(_))));
    }

    #[wasm_bindgen_test]
    fn text_not_found_when_search_term_not_found_in_label() {
        let rendered = Rendered::render(html! {
            <div>
                <form>
                    <label for="new-todo">{ "What doesn't need to be done?" }</label>
                    <br />
                    <input id="new-todo" value={ "hi!" } />
                </form>
            </div>
        });

        let result = rendered.get_by_label_text::<HtmlElement>("What needs to be done?");

        assert!(matches!(result, Err(LabelByTextError::LabelNotFound(_))));
    }

    fn input_label_text() -> Html {
        html! {
            <div>
                <form>
                    <label for="new-todo">{"What needs to be done?"}</label>
                    <br />
                    <input id="new-todo" value={"hi!"} />
                </form>
            </div>
        }
    }

    fn input_label_text_label_after_input() -> Html {
        html! {
            <div>
                <form>
                    <input id="new-todo" value={"hi!"} />
                    <br />
                    <label for="new-todo">{"What needs to be done?"}</label>
                </form>
            </div>
        }
    }

    fn input_label_text_different_parents() -> Html {
        html! {
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
}
