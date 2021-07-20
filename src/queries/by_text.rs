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
use std::{fmt::Debug, ops::Deref};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Node, NodeFilter, TreeWalker};

use crate::{util, TestRender};

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
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
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
            .unwrap();

        assert_eq!("text-button", button.id());
    }
    ```

    ## Get label by text:

    The label is the last element with the correct text node and will be returned due to `T` being
    [`HtmlLabelElement`](web_sys::HtmlLabelElement).

    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
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
            .unwrap();

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
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
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
            .unwrap();

        assert_eq!("text-div", element.id());
    }
    ```
    This might seem surprising but the outer div element contains a text node "Hello, " as the strong
    element breaks the text node - take care trying to find elements by text.
    */
    fn get_by_text<'search, T>(&self, search: &'search str) -> Result<T, ByTextError<'search>>
    where
        T: JsCast;
}

impl ByText for TestRender {
    fn get_by_text<'search, T>(&self, search: &'search str) -> Result<T, ByTextError<'search>>
    where
        T: JsCast,
    {
        let search_string = search.to_owned();

        let filter_on_text_value = move |node: Node| {
            if node
                .parent_element()
                .and_then(|e| e.dyn_into::<T>().ok())
                .is_some()
            {
                node.text_content()
                    .map(|text| text == search_string)
                    .unwrap_or_default()
            } else {
                false
            }
        };

        let walker = create_filtered_tree_walker(
            &self.root_element,
            WhatToShow::ShowText,
            filter_on_text_value,
        );

        if let Some(node) = walker.next_node().unwrap() {
            Ok(node.parent_element().unwrap().unchecked_into())
        } else {
            // nothing found - lets go back over each text node and find 'close' matches
            let walker = create_filtered_tree_walker(
                &self.root_element,
                WhatToShow::ShowText,
                move |node: Node| {
                    node.parent_element()
                        .and_then(|e| e.dyn_into::<T>().ok())
                        .is_some()
                },
            );

            let iter = std::iter::from_fn(move || walker.next_node().ok().flatten())
                .filter_map(|node| node.text_content().map(|text| (text, node)));

            if let Some(closest) = util::closest(search, iter, |(key, _)| key) {
                Err(ByTextError::Closest((search, closest.1)))
            } else {
                Err(ByTextError::NotFound(search))
            }
        }
    }
}

/**
An error indicating that no text node was an equal match for a given search term.
*/
pub enum ByTextError<'search> {
    /// No text node could be found with the given search term.
    NotFound(&'search str),
    /**
    No text node with an exact match for the search term could be found, however, a text node
    with a similar content as the search term was found.

    This should help find elements when a user has made a typo in either the test or the
    implementation being tested or when trying to find text with a dynamic number that may be
    incorrect
    */
    Closest((&'search str, Node)),
}

impl Debug for ByTextError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByTextError::NotFound(search) => {
                write!(
                    f,
                    "\nNo text node found with text equal or similar to '{}'\n",
                    search
                )
            }
            ByTextError::Closest((search, closest)) => {
                write!(
                    f,
                    "\nNo exact match found for the text: '{}'\nDid you mean to find this Element:\n\t{}\n",
                    search,
                    closest.parent_element().unwrap().outer_html()
                )
            }
        }
    }
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

struct FilteredTreeWalker {
    walker: TreeWalker,
    _filter_cb: Closure<dyn Fn(Node) -> bool>,
}

impl Deref for FilteredTreeWalker {
    type Target = TreeWalker;

    fn deref(&self) -> &Self::Target {
        &self.walker
    }
}

fn create_filtered_tree_walker<F>(
    root: &Node,
    what_to_show: WhatToShow,
    filter: F,
) -> FilteredTreeWalker
where
    F: Fn(Node) -> bool + 'static,
{
    let mut node_filter = NodeFilter::new();
    let cb = Closure::wrap(Box::new(filter) as Box<dyn Fn(Node) -> bool>);
    node_filter.accept_node(cb.as_ref().unchecked_ref());
    let document = web_sys::Document::new().expect("No global 'document' object!");
    let walker = document
        .create_tree_walker_with_what_to_show_and_filter(
            root,
            what_to_show.into(),
            Some(&node_filter),
        )
        .expect("Unable to create a TreeWalker object!");

    FilteredTreeWalker {
        walker,
        _filter_cb: cb,
    }
}

#[cfg(test)]
mod tests {

    use sap_yew::test_render;

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
                    <button onclick={self.link.callback(|_| ())}>{ "Click me!" }</button>
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
        assert!(result.is_ok());
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
        assert!(not_found.is_err());

        let found = rendered.get_by_text::<Element>("Hello, ");
        assert!(found.is_ok())
    }

    #[wasm_bindgen_test]
    fn button_click_test() {
        let rendered = test_render! {
            <Counter />
        };

        let button: HtmlElement = rendered.get_by_text("Click me!").unwrap();
        button.click();

        let count = rendered.get_by_text::<Element>("Count: 1");
        assert!(count.is_ok());

        button.click();
        let count = rendered.get_by_text::<Element>("Count: 2");
        assert!(count.is_ok());
    }

    #[wasm_bindgen_test]
    fn find_close_match() {
        let rendered = test_render! {
            <button>{ "Click me!" }</button>
        };

        let result = rendered.get_by_text::<HtmlButtonElement>("Click me");

        match result {
            Ok(_) => panic!("Should not have found the button as the text is not an exact match!"),
            Err(error) => {
                let expected = format!(
                    "\nNo exact match found for the text: '{}'\nDid you mean to find this Element:\n\t{}\n",
                    "Click me",
                    "<button>Click me!</button>"
                );

                assert_eq!(expected, format!("{:?}", error));
            }
        }

        drop(rendered);

        let rendered = test_render! {
            <div>
                { "Click me!" }
            </div>
        };

        let result = rendered.get_by_text::<HtmlButtonElement>("Click me");

        match result {
            Ok(_) => panic!("Should not have found the div as the text is not a match and the generic type is too restrictive"),
            Err(err) => {
                let expected = format!("\nNo text node found with text equal or similar to '{}'\n",
                    "Click me"
                );
                assert_eq!(expected, format!("{:?}", err));
            }
        }
    }
}
