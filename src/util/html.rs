use std::iter::repeat;

use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(module = "/js/utils.js")]
extern "C" {
    fn format(str: JsValue) -> JsValue;
}

pub fn format_html(html: &str) -> String {
    format(html.into()).as_string().unwrap()
}

fn element_selection_string(element: &Element) -> String {
    let html = element.outer_html();

    let (opening_tag, rest) = html.split_at(html.find('>').unwrap());
    if rest.starts_with('\n') {
        opening_tag.to_owned()
    } else {
        html
    }
}

fn preceding_space(value: &str, idx: usize) -> String {
    if idx == 0 {
        return String::new();
    }

    value
        .split_at(idx)
        .0
        .chars()
        .rev()
        .take_while(|c| *c == ' ')
        .collect()
}

pub fn format_html_with_closest(html: &str, closest: &Element) -> String {
    let mut html = format_html(&html);
    let closest_opening_tag = element_selection_string(closest);
    let closest_pos = html.find(&closest_opening_tag).unwrap();
    let ws = preceding_space(&html, closest_pos);
    let selection = repeat('^')
        .take(closest_opening_tag.len())
        .collect::<String>();
    html.insert_str(
        closest_pos + 1 + closest_opening_tag.len(),
        &format!(
            "{}{} {}\n",
            ws, selection, "Did you mean to find this element?",
        ),
    );
    html
}

#[cfg(test)]
mod browser_tests {

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn todo_check() {
        let result = format(
            r##"<div class="todomvc-wrapper"><section class="todoapp"><header class="header"><h1>todos</h1><input placeholder="What needs to be done?" class="new-todo"></header><section class="main hidden"><input aria-label="toggle all todo items" id="toggle-all" type="checkbox" class="toggle-all"><label for="toggle-all"></label><ul class="todo-list"></ul></section><footer class="footer hidden"><span class="todo-count"><strong>0</strong> item(s) left</span><ul class="filters"><li><a href="#/" class="selected">All</a></li><li><a href=\#/active" class="not-selected">Active</a></li><li><a href="#/completed" class="not-selected">Completed</a></li></ul><button class="clear-completed">Clear completed (0)</button></footer></section><footer class="info"><p>Double-click to edit a todo</p><p>Written by <a href="https:/github.com/DenisKolodin/" target="_blank">Denis Kolodin</a></p><p>Part of <a href="http:/todomvc.com/" target="_blank">TodoMVC</a></p></footer></div>"##.into(),
        ).as_string().unwrap();

        let expected = r##"
<div class="todomvc-wrapper">
  <section class="todoapp">
    <header class="header">
      <h1>todos</h1>
      <input placeholder="What needs to be done?" class="new-todo">
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
          <a href="\#/active&quot;" class="not-selected">Active</a>
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
      <a href="https:/github.com/DenisKolodin/" target="_blank">Denis Kolodin</a>
    </p>
    <p>Part of 
      <a href="http:/todomvc.com/" target="_blank">TodoMVC</a>
    </p>
  </footer>
</div>
"##;

        assert_eq!(expected, result);
    }
}
