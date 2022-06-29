# hyphae

<!-- [![Crates.io][crates-badge]][crates-url] -->
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

<!-- [crates-badge]: https://img.shields.io/crates/v/hyphae.svg
[crates-url]: https://crates.io/crates/hyphae -->
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/mc1098/hyphae/blob/main/LICENSE
[actions-badge]: https://github.com/mc1098/hyphae/workflows/CI/badge.svg
[actions-url]: https://github.com/mc1098/hyphae/actions?query=workflow%3ACI+branch%3Astaging

hyphae is a testing library that provides abstractions on top of `wasm_bindgen` for testing DOM nodes.

The main feature of this crate is using `queries` to find elements in the DOM and perform actions
that simulate user behaviour to assert that your application behaves correctly.

The recommended query set to use is in the `by_aria` module - this queries
by ARIA and using this will also help you consider the accessibility of your application.


Requirements:
- [`wasm-bindgen-test`](https://crates.io/crates/wasm-bindgen) in dev-dependencies

All hyphae functions are assuming they will be in wasm-bindgen-tests:

```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test() {
    // .. test code here
}
```

Running wasm-bindgen-tests

Multiple browsers can be used here or just one:
```bash
$ wasm-pack test --headless --firefox --chrome
```

## Example

One of the tests found in the `todo` example.

```rust ,ignore
use super::*;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use hyphae::{event::*, prelude::*, type_to};

use web_sys::{HtmlButtonElement, HtmlInputElement};

#[wasm_bindgen_test]
fn make_new_todo_item_complete_it_then_clear_completed() {
    let rendered = QueryElement::new();
    yew::start_app_in_element::<Model>(rendered.clone().into());

    // get todo input
    let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

    // type 'Gardening' into input and confirm
    type_to!(input, "Gardening", Key::Enter);

    // 'Gardening' todo item has been rendered - lets just get the completed checkbox
    let checkbox: HtmlInputElement = rendered.assert_by_label_text("Gardening");
    // confirm that our new todo items is not completed
    assert!(!checkbox.checked());

    // get clear completed button
    let clear_completed_btn: HtmlButtonElement =
        rendered.assert_by_aria_role(AriaRole::Button, "Clear completed (0)");

    // click the todo checkbox - marking it complete
    checkbox.click();
    // confirm that a todo item was marked completed
    assert_text_content!("Clear completed (1)", clear_completed_btn);

    // click to clear all completed todo items
    clear_completed_btn.click();
    // confirm that the todo item has been removed
    assert!(!rendered.contains(Some(&checkbox)));
}
```

## Helpful error messages for typos

### Query typos

We all make mistakes and when we do so in tests it can be even more confusing, does the code I'm
testing work?, is it the test?, is it hyphae?! (how dare you! ;) ).

Small typos when trying to find elements using queries is easily done, so hyphae tries to find similar
string values when a query fails to find an exact match:

In the `todo` example if you tried to find the input with the placeholder text "What needs to be done?",
it is easy to type this with no capital and forget the question mark (or something similar).

```rust ,ignore
let input: HtmlInputElement = rendered.assert_by_placeholder_test("what needs to be done");
```

This will cause the test to fail because an exact match wasn't found but will contain the following
output to help you:

```bash
No exact match found for the placeholder text: 'what needs to be done'.
A similar match was found in the following HTML:
<div class="todomvc-wrapper">
  <section class="todoapp">
    <header class="header">
      <h1>todos</h1>
      <input placeholder="What needs to be done?" class="new-todo">
      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Did you mean to find this element?
    </header>
    <section class="main hidden">
      <input aria-label="toggle all todo items" id="toggle-all" type="checkbox" class="toggle-all">
      <label for="toggle-all"></label>
      <ul class="todo-list"></ul>
    </section>
    <footer class="footer hidden">
      <span class="todo-count">
        <strong>0</strong> item(s) left
      </span>
      <ul class="filters">
        <li>
          <a href="#/" class="selected">All</a>
        </li>
        <li>
          <a href="#/active" class="not-selected">Active</a>
        </li>
        <li>
          <a href="#/completed" class="not-selected">Completed</a>
        </li>
      </ul>
      <button class="clear-completed">Clear completed (0)</button>
    </footer>
  </section>
  <footer class="info">
    <p>Double-click to edit a todo</p>
    <p>Written by
      <a href="https://github.com/DenisKolodin/" target="_blank">Denis Kolodin</a>
    </p>
    <p>Part of
      <a href="http://todomvc.com/" target="_blank">TodoMVC</a>
    </p>
  </footer>
</div>
```
This provides a "did you mean" hint and also shows the current state of the DOM when the query failed.
