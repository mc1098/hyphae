/*!
Queries for finding [`Element`](web_sys::Element)s.

This module helps to query the DOM of a rendered root element. The goal is to use high/medium level
APIs so that the DOM can be queried in a manner similar to how a user might navigate the UI.
*/

#[doc(inline)]
pub mod by_aria;
#[doc(inline)]
pub mod by_display_value;
#[doc(inline)]
pub mod by_label_text;
#[doc(inline)]
pub mod by_placeholder_text;
#[doc(inline)]
pub mod by_text;
