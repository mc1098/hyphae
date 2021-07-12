/*!
Supports finding elements generically by ARIA property, role, state and accessible name.

The sections below will give an overview of ARIA; so that you are able to query for elements effectively.
ARIA gives a descriptive way to query the DOM for particular elements and also shows how accessible
the application is - if you can't find the the element using ARIA then how will a screen reader
find it?

# What is ARIA?
Accessible Rich Internet Applications (ARIA) is a set of attributes, implicit roles, that define
ways to make web content and web applications more accessible to people with disabilities.

It is important to consider accessiblility when designing a web application - one of the easiest ways
to do this is to use the correct semantic HTML 5 element as these come with built-in
keyboard accessibility, roles and states.

[Read more about ARIA.](https://www.w3.org/TR/wai-aria-1.1/#introduction)

# Accessible Name

The name of the element used by accessible APIs, such as screen readers. The accessible name of an
element may be derived from visible or invisible elements in the DOM.

<details>
<summary style="cursor: pointer;">Click here to toggle examples</summary>

A simple case of this is a single "Ok" button:

```html
<button>Ok</button>
```
_Accessible name: "Ok"._

The buttons text content has been used for the accessible name.

A slightly more complex case using multiple elements:

```html
<button>
    <span>
        Delete <!--1. Text node-->
    </span>
    <span>
        <img src="pict.jpg" alt="Profile" /> <!--2. alt-->
        Joe Bloggs <!--3. Text node-->
    </span>
</button>
```
_Accessible name: "Delete Profile Joe Bloggs"._

The accessible name is derived as it traverses the
[Accessibility Tree](https://www.w3.org/TR/accname-1.1/#dfn-accessibility-tree).
1. Span text node "Delete"
2. Img alt value "Profile"
3. Span text node "Joe Bloggs"

So far no aria-* properties and this is due to normal elements having text alternatives which help
build up accessible names even when no aria-* properties have been used, thus correct use of HTML
semantics helps accessiblilty without you thinking about it.

The aria-label property can be used to provide a custom accessible name to an element and stops
the traversal of the accessibility tree:

```html
<div aria-label="My very best link!">
    <a href="someurlhere">A very Ok link</a>
</div>
```
_Accessible name: "My very best link!"_.

It's worth noting that the div's accessible name is "My very best link!" and yet the anchors
accessible name is "A very Ok link".

For a user who is tabbing through your website, they will skip the div and focus on the anchor which
will read the anchors accessiblilty name. Keep this mind if adding some wrapper around an anchor,
button or other embedded control element as this will behave differently for accessibility users.

Accessible names will generally only be built up from visible elements:

**We are considering the accessible name of the input and not the div**

```html
<div id="parentId"> <!--2. followed labelledby ref-->
    Password: <!--3. Ignored-->
    <input aria-labelledby="parentId" type="password" /> <!--1. labelledby--><!--4. Ignored-->
    <div style="display:none;"> <!--5. & 6. Not displayed-->
        Input typed in this field is hidden.
    </div>
</div>
```
_Accessible name: "Password:"_.

_Yikes_ that's a mess! but lets go through it step by step:

1. Input aria-labelledby reference - traversal pauses here and we follow the id reference
2. Div is the referenced element - so we traverse from here
3. Div text node "Password:"
4. Input aria-labelledby reference - but we've seen it before so we ignore it
5. Div text node is ignored due to the "style=display:none" - it's not rendered so won't be considered
6. The second traversal ended and so we resume the first - again div is ignored

The aria-hidden state attribute can be used to exclude or include elements when calculating the
accessible name:

**We are considering the accessible name of the input and not the div**

```html
<div id="parentId">
    Password:
    <input aria-labelledby="parentId" type="password" />
    <div style="display:none;" aria-hidden="false">
        Input typed in this field is hidden.
    </div>
</div>
```
_Accessible name: "Password: Input typed in this field is hidden."_

The steps are the same until we get to the div with aria-hidden="false" which includes this element
as part of the accessible name. This means that this information is only for accessibility users -
this is a silly example but using this technique you can provide extra information to provide a much
better experience for accessibility users.

The above example used was bit contrived so we could cover aria-labelledby too, the example would be
better written using the label element instead of the div:

```html
<label for="user-password">
    Password:
    <input id="user-password" type="password" />
    <div style="display:none;" aria-hidden="false">
        Input typed in this field is hidden.
    </div>
</label>
```

The use of the label element is preferred here - for mouse users, clicking "Password:" will bring
the input to focus when using a label and won't when using a div.

Accessible name of an element tend to get more complicated with the more child elements (and grandchild).
if you are unsure about what the accessible name is, then you can use your browser to navigate through
the accessible tree.

</details>

# Implicit Roles

ARIA roles provide assistive technologies information about how to handle each element. Using roles
make elements more accessible but having to add a role specifically to each element used would be
tedious, error-prone and developers just wouldn't do it!

In order to make HTML more accessible some elements have implicit ARIA roles so they don't need to
be defined, again another tick for using correct HTML semantics.

Some of these roles depend on the attributes of elements, for instance an anchor element only has a
"link" role when it has a href attribute.

[A table of implicit roles by element.](https://www.w3.org/TR/html-aria/#docconformance)

# ARIA Attribute Parity

ARIA properties and state attributes share similar names with native HTML features, to avoid explicitly
setting both versions the native HTML features have implicit ARIA semantics.

For example:

```html
<input required type="text" />
       ^^^^^^^^ boolean attribute
```
The required boolean attribute has the implicit semantics of "aria-required='true'" so the aria-*
version does not need to be explicitly set.

[A table of native HTML features aria-* attribute parity.](https://www.w3.org/TR/html-aria/#docconformance-attr)

 */
use sap_aria::{AriaProperty, AriaRole, AriaState, ToQueryString};
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::TestRender;

/**
Enables querying elements generically by ARIA roles, properties, and state.

_See the [module page for more on ARIA.](super::by_aria)_

_See each trait function for examples._
*/
pub trait ByAria {
    /**

    Get a generic element by ARIA role and accessible name.

    Using an explicit element type as `T` will essentially skip the other types of elements - if
    you want to find the very first element that matches the ARIA role and accessible name then use
    [`HtmlElement`](web_sys::HtmlElement).

    # Panics
    _Nothing to see here._

    # Examples

    ## Get button by role
    The button element has an implicit ARIA button role and the accessible name for the button is
    equal to it's text content [^note].

    [^note] _The accessible name is only the text content as there are no label or aria referenced
    elements. See the [module page for more on Accessible name](super::by_aria)_

    Rendered html:
    ```html
    <div>
        <div id="not-mybtn">
            <button id="mybtn">Click me!</button>
        </div>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlButtonElement;

    #[wasm_bindgen_test]
    fn get_button_by_role() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                #   <div id="not-mybtn">
                #       <button id="mybtn">{ "Click me!" }</button>
                #   </div>
                # </div>
            # };

        let button: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "Click me!")
            .expect("to find button by it's alternative text!");

        assert_eq!("mybtn", button.id());
    }
    ```

    ## Get checkbox by role

    An input element with a type "checkbox" has the ARIA checkbox role and with a aria-label attribute
    the accessible name is equal to this value.

    Rendered html:
    ```html
    <div>
        <input type="checkbox" aria-label="toggle all todo items" />
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_button_by_role() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                    # <input id="toggle-all" type="checkbox" aria-label="Toggle all todo items" />
                # </div>
            # };

        let checkbox: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::Checkbox, "Toggle all todo items")
            .expect("to find checkbox using aria-label value");

        assert_eq!("toggle-all", checkbox.id());
    }
    ```
    _Note: This comes from the todo example which has a toggle all checkbox but does not
    have an associated label which makes it not very accessible. The aria-label was added to help with
    testing but also improved the accessiblity of the todo example in the process._
    */
    fn get_by_aria_role<T>(&self, role: AriaRole, name: &str) -> Option<T>
    where
        T: JsCast;

    /**

    Get a generic element by ARIA property and optional accessible name.

    Some [`AriaProperty`] are so descriptive that the accessible name is not required, such is the
    case with [`AriaProperty::Label`] [^note] - the content of this property is the accessible name.
    When this is the case the accessible name can be [`None`].

    Using an explicit element type as `T` will essentially skip the other types of elements - if
    you want to find the very first element that matches the ARIA property and accessible name
    then use [`HtmlElement`](web_sys::HtmlElement).

    [^note]_If an accessible name is provided then it must match the content of
    [`AriaProperty::Label`]._

    # Panics

    _Nothing to see here._

    # Examples

    ## Get input by required property and accessible name

    There are two required inputs in the form and so the accessible name is needed to differentiate
    between the two. We use the [`AriaProperty::Required`] enum variant which accepts a `bool` to
    assert that the element is required.

    Rendered html:
    ```html
    <form>
        <label for="user-email">Email:</label>
        <input type="email" id="user-email" required />
        <label for="user-password">Password:</label>
        <input type="password" id="user-password" required />
    </form>
    ```
    Code:
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_required_input_by_name() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <form>
                #   <label for="user-email">{ "Email:" }</label>
                #   <input type="email" id="user-email" required=true />
                #   <label for="user-password">{ "Password:" }</label>
                #   <input type="password" id="user-password" required=true />
                # </form>
            # };

        let email_input: HtmlInputElement = rendered
            .get_by_aria_prop(AriaProperty::Required(true), "Email:")
            .expect("to find required email input");

        assert_eq!("user-email", email_input.id());
    }
    ```

    ## Get button by aria-label

    Getting a button like this instead of by role and accessible name is probably not what you want
    to do, but this does assert that this element's accessible name won't change due to aria-label
    being set.

    Rendered html:
    ```html
    <div>
        <div id="not-mybtn">
            <button id="mybtn" aria-label="ok" /> <!-- No text on button -->
        </div>
    </div>
    ```
    Code:
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlButtonElement;

    #[wasm_bindgen_test]
    fn get_button_aria_label() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <div>
                    # <div id="not-mybtn">
                        # <button id="mybtn" aria-label="ok"/>
                    # </div>
                # </div>
            # };

        let button: HtmlButtonElement = rendered
            .get_by_aria_prop(AriaProperty::Label("ok"), None)
            .expect("to get button by it's aria-label value");

        assert_eq!("mybtn", button.id());
    }
    ```
    */
    fn get_by_aria_prop<'a, S, T>(&self, property: AriaProperty, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast;

    /**

    Get a generic element by ARIA state and optional accessible name.

    Some [`AriaState`] can be so descriptive in subsections of the DOM that an accessible name is
    not requried to identify a single element, such is the case with [`AriaState::Selected`] on a
    subsection that can only have a single selected item. When the accessible name is not required
    the value can be [`None`].

    Using an explicit element type as `T` will essentially skip the other types of elements - if
    you want to find the very first element that matches the ARIA state and accessible name then
    use [`HtmlElement`](web_sys::HtmlElement).

    # Panics

    _Nothing to see here._

    # Examples

    ## Get input with invalid spelling

    One input element has a mispelled animal value which has been validated and the
    "aria-invalid=spelling" state has been applied.

    Rendered html:
    ```html
    <form>
        <input id="best-pet" aria-invalid="spelling" value="doge" aria-label="best pet" />
        <input id="second-best-pet" value="cat" aria-label="second best pet" />
    </form>
    ```
    Code:
    ```no_run
    # fn main() {}
    # use yew::prelude::*;
    # use sap_yew::test_render;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use sap::prelude::*;
    use web_sys::HtmlInputElement;

    #[wasm_bindgen_test]
    fn get_invalid_spelling_input() {
        let rendered: TestRender = // feature dependent rendering
            # test_render! {
                # <form>
                    # <input id="best-pet" aria-invalid="spelling" value="doge" aria-label="best pet" />
                    # <input id="second-best-pet" value="cat" aria-label="second best pet" />
                # </form>
            # };

        let spelling_error_input: HtmlInputElement = rendered
            .get_by_aria_state(AriaState::Invalid(InvalidToken::Spelling), "best pet")
            .expect("expect to find input by spelling error - doge is a typo!");

        assert_eq!("best-pet", spelling_error_input.id());
    }
    ```
    */
    fn get_by_aria_state<'a, S, T>(&self, state: AriaState, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast;
}

#[inline]
fn get_by_aria_impl<S, T>(root: &Element, aria: S, name: Option<&str>) -> Option<T>
where
    S: ToQueryString,
    T: JsCast,
{
    let node_list = root.query_selector_all(&aria.to_query_string()).ok()?;
    if let Some(name) = name {
        for i in 0..node_list.length() {
            let node = node_list.get(i).and_then(|node| node.dyn_into::<T>().ok());

            if let Some(element) = node {
                let acc_name = sap_aria::element_accessible_name(&element.unchecked_ref()).ok()?;
                if name == acc_name {
                    return Some(element);
                }
            }
        }
        None
    } else {
        node_list.get(0).and_then(|e| e.dyn_into().ok())
    }
}

impl ByAria for TestRender {
    fn get_by_aria_role<T>(&self, role: AriaRole, name: &str) -> Option<T>
    where
        T: JsCast,
    {
        get_by_aria_impl(self, role, name.into())
    }

    fn get_by_aria_prop<'a, S, T>(&self, prop: AriaProperty, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast,
    {
        let name = name.into();
        if let AriaProperty::Label(ref label) = prop {
            // if someone specified the name while using label then they must match
            if name.map(|name| name != label).unwrap_or_default() {
                None
            } else {
                self.query_selector(&prop.to_query_string())
                    .ok()
                    .flatten()
                    .and_then(|e| e.dyn_into().ok())
            }
        } else {
            get_by_aria_impl(self, prop, name)
        }
    }

    fn get_by_aria_state<'a, S, T>(&self, state: AriaState, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast,
    {
        get_by_aria_impl(self, state, name.into())
    }
}

#[cfg(test)]
mod tests {

    use sap_yew::test_render;

    use super::*;
    use sap_aria::InvalidToken;
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlImageElement, HtmlInputElement};
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn get_by_button_role_with_text_content() {
        let rendered = test_render! {
            <div>
                <div id="not-mybtn">
                    { "click me" }
                    <button id="mybtn">{ "click me!" }</button>
                </div>
            </div>
        };
        let button: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "click me!")
            .unwrap();

        assert_eq!("mybtn", button.id());
    }

    #[wasm_bindgen_test]
    fn get_by_aria_label() {
        let rendered = test_render! {
            <div>
                <div id="not-mybtn">
                    <button id="mybtn" aria-label="ok" /> // No text on button
                </div>
            </div>
        };

        let button: HtmlButtonElement = rendered
            .get_by_aria_prop(AriaProperty::Label("ok".to_owned()), None)
            .unwrap();

        assert_eq!("mybtn", button.id());
    }

    #[wasm_bindgen_test]
    fn get_by_aria_disabled_state() {
        let rendered = test_render! {
            <div>
                <input type="email" id="my-input" aria-disabled="true" />
            </div>
        };

        let input: HtmlInputElement = rendered
            .get_by_aria_state(AriaState::Disabled(true), None)
            .unwrap();

        assert_eq!("my-input", input.id());
    }

    #[wasm_bindgen_test]
    fn get_single_input_with_spelling_error() {
        let rendered = test_render! {
            <form>
                <input id="best-pet" aria-label="best pet" aria-invalid="spelling" value="doge" />
                <input id="second-best-pet" aria-label="second best pet" aria-invalid="false" value="cat"  />
            </form>
        };
        let spelling_error_input: HtmlInputElement = rendered
            .get_by_aria_state(AriaState::Invalid(InvalidToken::Spelling), "best pet")
            .unwrap();

        assert_eq!("best-pet", spelling_error_input.id());
    }

    #[wasm_bindgen_test]
    fn get_input_by_role_with_aria_label() {
        let rendered = test_render! {
            <div>
                <input id="myinput" type="text" aria-label="username" />
            </div>
        };

        let input: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::TextBox, "username")
            .unwrap();

        assert_eq!("myinput", input.id());
    }

    #[wasm_bindgen_test]
    fn get_button_by_role_with_aria_labelledby() {
        let rendered = test_render! {
            <>
                <div id="button-label" >
                    { "My custom button label" }
                </div>
                <button aria-labelledby="button-label" />
            </>
        };

        let button: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "My custom button label")
            .unwrap();

        assert_eq!(
            "button-label",
            button.get_attribute("aria-labelledby").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn get_input_by_role_with_label() {
        let rendered = test_render! {
            <div>
                <div>
                    <label for="my-input">{ "My input label" }</label>
                </div>
                <input id="my-input" type="search" />
            </div>
        };

        let input: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::Searchbox, "My input label")
            .unwrap();

        assert_eq!("my-input", input.id());
    }

    #[wasm_bindgen_test]
    fn get_img_by_role_with_alt() {
        let rendered = test_render! {
            <div>
                <img id="no" src="first-img.jpg" />
                <img id="yes" src="somg-img.jpg" alt="The best image ever!" />
            </div>
        };

        let img: HtmlImageElement = rendered
            .get_by_aria_role(AriaRole::Image, "The best image ever!")
            .unwrap();

        assert_eq!("yes", img.id());
    }
}
