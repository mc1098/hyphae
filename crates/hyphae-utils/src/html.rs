use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Element;

#[wasm_bindgen(module = "/js/hyphae-utils.js")]
extern "C" {
    fn format(str: JsValue) -> JsValue;
}

macro_rules! get_js_property_impl {
    ($getter:ident, $setter:ident, $mapper:ident, $property_name:literal:$property_type:ty) => {
        pub fn $getter<T: JsCast>(element: &T) -> Option<$property_type> {
            js_sys::Reflect::get(&element.into(), &$property_name.into())
                .ok()
                .and_then(|v| v.as_string())
        }

        pub fn $setter<T: JsCast, V: Into<JsValue>>(element: &T, value: V) -> bool {
            js_sys::Reflect::set(&element.into(), &$property_name.into(), &value.into())
                .expect("implementations of JsCast should be Objects")
        }

        pub fn $mapper<T: JsCast, V: Into<JsValue>, F: FnMut($property_type) -> V>(
            element: &T,
            mut f: F,
        ) -> bool {
            $getter(element)
                .map(move |prop| $setter(element, f(prop)))
                .unwrap_or_default()
        }
    };
}

get_js_property_impl! {
    get_element_value, set_element_value, map_element_value, "value":String
}

pub fn format_html(html: &str) -> String {
    format(html.into()).as_string().unwrap()
}

fn element_selection_string(element: &Element) -> String {
    let html = format_html(&element.outer_html());

    let (opening_tag, rest) = html.split_at(html.find('>').unwrap());
    if rest.starts_with(">\n") {
        let mut opening_tag = opening_tag.to_owned();
        opening_tag.push('>');
        opening_tag.trim().to_owned()
    } else {
        html.trim().to_owned()
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
    let mut html = format_html(html);
    let closest_opening_tag = element_selection_string(closest);
    let closest_pos = html.find(&closest_opening_tag).unwrap();
    let ws = preceding_space(&html, closest_pos);
    let selection = "^".repeat(closest_opening_tag.len());
    let to_insert = format!(
        "{}{} {}\n",
        ws, selection, "Did you mean to find this element?"
    );

    if html.len() <= closest_pos + closest_opening_tag.len() + 1 {
        html.push_str(&to_insert);
    } else {
        html.insert_str(closest_pos + closest_opening_tag.len() + 1, &to_insert);
    }
    html
}

pub fn make_element_with_html_string(inner_html: &str) -> web_sys::HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    // remove \n & \t and 4 x spaces which are just formatting to avoid text nodes being added
    let inner_html = inner_html
        .chars()
        .fold((String::new(), 0), |(mut s, ws), c| match c {
            ' ' if ws == 3 => {
                s.truncate(s.len() - 3);
                (s, 0)
            }
            ' ' => {
                s.push(c);
                (s, ws + 1)
            }
            '\n' | '\t' => (s, 0),
            _ => {
                s.push(c);
                (s, 0)
            }
        })
        .0;
    div.set_inner_html(&inner_html);

    document.body().unwrap().append_child(&div).unwrap();
    div.unchecked_into()
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
