/*!
Queries for finding [`Element`](web_sys::Element)s.

This module helps to query the DOM of a rendered root element. The goal is to use high/medium level
APIs so that the DOM can be queried in a manner similar to how a user might navigate the UI.
*/

pub mod by_aria;
pub mod by_display_value;
pub mod by_label_text;
pub mod by_placeholder_text;
pub mod by_text;
