use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Node, NodeFilter};

use crate::TestRender;

/// Enables queries by text nodes
pub trait ByText {
    /**
    Get the first [`Element`](web_sys::Element) that has a text node matching the search term.

    Note: checks for equality, using [`PartialEq`], against all text nodes in the
    root element. This will not find text that is broken up by `strong` or similar tags.

    # Examples

    ## Basic usage:
    Effective html rendered:
    ```html
    <div>
        <div id="inner-div">Hello, World!</div>
    </div>
    ```
    ```no_run
    let rendered: TestRender = // ...
    let div: Element = rendered.get_by_text("Hello, World!").unwrap();

    assert_eq!("inner-div", div.id());
    ```

    ## When text is broken up between tags:
    Effective html rendered:
    ```html
    <div>
        Hello, <strong>World!</strong>
    </div>
    ```
    ```no_run
    let rendered: TestRender = // ...
    // can't find `Hello, World!` as they are two distinct text nodes :(
    let not_found = rendered.get_by_text::<Element>("Hello, World!");
    assert!(not_found.is_none());

    let found = rendered.get_by_text::<Element>("Hello, ");
    assert!(found.is_some())
    ```

    ## Narrow search using generics:
    Effective html rendered:
    ```html
    <div>
        <div id="div">{ "Hello!" }</div>
        <label id="label">{ "Hello!" }</label>
        <button id="button">{ "Hello!" }</button>
    </div>
    ```
    ```no_run
    let rendered: TestRender = // ...
    let button: HtmlButtonElement = rendered.get_by_text("Hello!").unwrap();
    assert_eq!("button", button.id());

    // `Element` generic so will default to the first element
    let div: Element = rendered.get_by_text("Hello!").unwrap();
    assert_eq!("div", div.id());

    let label: HtmlLabelElement = rendered.get_by_text("Hello!").unwrap();
    assert_eq!("label", label.id());
    ```
    */
    fn get_by_text<T>(&self, search: &'_ str) -> Option<T>
    where
        T: JsCast;
}

#[non_exhaustive]
enum WhatToShow {
    ShowText,
}

impl From<WhatToShow> for u32 {
    fn from(show: WhatToShow) -> Self {
        match show {
            WhatToShow::ShowText => 4,
        }
    }
}

impl ByText for TestRender {
    fn get_by_text<T>(&self, search: &'_ str) -> Option<T>
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
        let document = web_sys::Document::new().ok()?;
        let walker = document
            .create_tree_walker_with_what_to_show_and_filter(
                &self.root_element,
                WhatToShow::ShowText.into(),
                Some(&filter),
            )
            .unwrap();

        // loop until we find a parent element which is of type T or return None when we run out of
        // nodes.
        loop {
            let node = walker.next_node().unwrap()?;
            if let Some(result) = node.parent_element().and_then(|e| e.dyn_into().ok()) {
                break Some(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{Element, HtmlButtonElement, HtmlElement, HtmlLabelElement};
    use yew::{html, prelude::*, virtual_dom::test_render};

    pub(crate) struct Counter {
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

    #[wasm_bindgen_test]
    fn text_search() {
        let test = TestRender::new(test_render(html! {
            <div>
                <div>
                    { "Hello, World!" }
                </div>
            </div>
        }));

        let result = test.get_by_text::<Element>("Hello, World!");
        assert!(result.is_some());
    }

    #[wasm_bindgen_test]
    fn search_for_text_narrow_with_generics() {
        let rendered: TestRender = test_render(html! {
            <div>
                <div id="div">{ "Hello!" }</div>
                <label id="label">{ "Hello!" }</label>
                <button id="button">{ "Hello!" }</button>
            </div>
        })
        .into();

        let button: HtmlButtonElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("button", button.id());

        let div: Element = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("div", div.id());

        let label: HtmlLabelElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("label", label.id());
    }

    #[wasm_bindgen_test]
    fn by_text_uses_text_nodes_not_text_content() {
        let rendered: TestRender = test_render(html! {
            <div>
                { "Hello, " }
                <strong>{ "World!" }</strong>
            </div>
        })
        .into();
        // can't find `Hello, World!` as they are two distinct text nodes :(
        let not_found = rendered.get_by_text::<Element>("Hello, World!");
        assert!(not_found.is_none());

        let found = rendered.get_by_text::<Element>("Hello, ");
        assert!(found.is_some())
    }

    #[wasm_bindgen_test]
    fn button_click_test() {
        let rendered: TestRender = test_render(html! {
            <Counter />
        })
        .into();

        let button: HtmlElement = rendered.get_by_text("Click me!").unwrap();
        button.click();

        let count = rendered.get_by_text::<Element>("Count: 1");
        assert!(count.is_some());

        button.click();
        let count = rendered.get_by_text::<Element>("Count: 2");
        assert!(count.is_some());
    }
}
