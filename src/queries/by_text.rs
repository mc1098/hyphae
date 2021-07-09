/*!
Supports finding elements generically by their text node.

The text node of an element is the text raw text between the opening and closing tags.

```html
<div>
    div text node
    <button>button text node</button>
</div>
```

Text nodes are broken up when using elements such as `strong`:
```html
<div>
    div text node <strong>strong text node</strong>
</div>
```

# Generics
Each trait function supports generics for convenience and to help narrow the scope of the search. If
you are querying for a [`HtmlButtonElement`](web_sys::HtmlInputElement) then you won't find a
[`HtmlDivElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDivElement.html)
and vice versa.

In [`Sap`](crate) the [`HtmlElement`](web_sys::HtmlElement) can be used as a "catch all" generic
type[^note].

[^note] _[`Element`](web_sys::Element) and [`Node`](web_sys::Node) can also be used as a 'catch all'
type, however, [`HtmlElement`](web_sys::HtmlElement) has more useful functions for making assertions
or performing certain actions, such as [`click`](web_sys::HtmlElement::click)._

# What is [`JsCast`]?

The generic type returned needs to impl [`JsCast`] which is a trait from [`wasm_bindgen`] crate for
performing checked and unchecked casting between JS types.
 */
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Node, NodeFilter};

use crate::TestRender;

/**
Enables queries by text node.

_See each trait function for examples._
*/
pub trait ByText {
    /**

    Get a generic element by the text node.

    Using one of the generic types above as `T` will essentially skip the other two types of
    elements - if you want to find the very first element that matches the display value then use
    [`HtmlElement`](web_sys::HtmlElement).

    # Panics
    _Nothing to see here_

    # Examples

    Rendered html:
    ```html
    <div>
        Hello, <strong>World!</strong>
        <div id="text-div">Hello, World!</div>
        <button id="text-button">Hello, World!</button>
        <label id="text-label">Hello, World!</label>
    </div>
    ```

    ## Get button by text:

    The button is the second element with the correct text node and will be returned due to
    `T` being [`HtmlButtonElement`](web_sys::HtmlButtonElement).

    ```no_run
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlButtonElement;

    #[wasm_bindgen_test]
    fn get_button_by_text() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                    # { "Hello, "}<strong>{ "World!" }</strong>
                    # <div id="text-div">{ "Hello, World!" }</div>
                    # <button id="text-button">{ "Hello, World!" }</button>
                    # <label id="text-label">{ "Hello, World!" }</label>
                # </div>
            # };
        let button: HtmlButtonElement = rendered
            .get_by_text("Hello, World!")
            .expect("skip the div elements to find the button by text");

        assert_eq!("text-button", button.id());
    }
    ```

    ## Get label by text:

    The label is the last element with the correct text node and will be returned due to `T` being
    [`HtmlLabelElement`](web_sys::HtmlLabelElement).

    ```no_run
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlLabelElement;

    #[wasm_bindgen_test]
    fn get_label_by_text() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                    # { "Hello, "}<strong>{ "World!" }</strong>
                    # <div id="text-div">{ "Hello, World!" }</div>
                    # <button id="text-button">{ "Hello, World!" }</button>
                    # <label id="text-label">{ "Hello, World!" }</label>
                # </div>
            # };
        let label: HtmlLabelElement = rendered
            .get_by_text("Hello, World!")
            .expect("skip the div elements and the button to find the label by text");

        assert_eq!("text-label", label.id());
    }
    ```

    ## Get first element by text:

    The inner div element is the first element with the correct text node and will be returned due
    to `T` being [`HtmlElement`](web_sys::HtmlElement)[^note].

    [^note]_`T` could be
    [`HtmlDivElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDivElement.html)
    to be even more restrictive, however, it is not required in this case._

    ```no_run
    # #[cfg(feature = "Yew")]
    # fn main() {}
    # use yew::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn get_first_element_by_text() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                    # { "Hello, "}<strong>{ "World!" }</strong>
                    # <div id="text-div">{ "Hello, World!" }</div>
                    # <button id="text-button">{ "Hello, World!" }</button>
                    # <label id="text-label">{ "Hello, World!" }</label>
                # </div>
            # };
        let element: HtmlElement = rendered
            .get_by_text("Hello, World!")
            .expect("skip the div elements and the button to find the label by text");

        assert_eq!("text-div", element.id());
    }
    ```
    This might seem suprising but the outer div element contains a text node "Hello, " as the strong
    element breaks the text node - take care trying to find elements by text.
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
            let node = walker.next_node().ok().flatten()?;
            if let Some(result) = node.parent_element().and_then(|e| e.dyn_into().ok()) {
                break Some(result);
            }
        }
    }
}

#[cfg(all(test, feature = "Yew"))]
mod tests {

    use crate::test_render;

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{Element, HtmlButtonElement, HtmlElement, HtmlLabelElement};
    use yew::{html, prelude::*};

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
        let test = test_render! {
            <div>
                <div>
                    { "Hello, World!" }
                </div>
            </div>
        };

        let result = test.get_by_text::<Element>("Hello, World!");
        assert!(result.is_some());
    }

    #[wasm_bindgen_test]
    fn search_for_text_narrow_with_generics() {
        let rendered = test_render! {
            <div>
                <div id="div">{ "Hello!" }</div>
                <label id="label">{ "Hello!" }</label>
                <button id="button">{ "Hello!" }</button>
            </div>
        };

        let button: HtmlButtonElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("button", button.id());

        let div: Element = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("div", div.id());

        let label: HtmlLabelElement = rendered.get_by_text("Hello!").unwrap();
        assert_eq!("label", label.id());
    }

    #[wasm_bindgen_test]
    fn by_text_uses_text_nodes_not_text_content() {
        let rendered = test_render! {
            <div>
                { "Hello, " }
                <strong>{ "World!" }</strong>
            </div>
        };
        // can't find `Hello, World!` as they are two distinct text nodes :(
        let not_found = rendered.get_by_text::<Element>("Hello, World!");
        assert!(not_found.is_none());

        let found = rendered.get_by_text::<Element>("Hello, ");
        assert!(found.is_some())
    }

    #[wasm_bindgen_test]
    fn button_click_test() {
        let rendered = test_render! {
            <Counter />
        };

        let button: HtmlElement = rendered.get_by_text("Click me!").unwrap();
        button.click();

        let count = rendered.get_by_text::<Element>("Count: 1");
        assert!(count.is_some());

        button.click();
        let count = rendered.get_by_text::<Element>("Count: 2");
        assert!(count.is_some());
    }
}
