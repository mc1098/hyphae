#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate self as hyphae;

mod asserts;
pub mod event;
mod iter;
pub mod queries;

/// Utility functions.
pub mod utils {
    pub use hyphae_utils::{effect_dom, wait_ms};
}

pub use iter::*;
pub use queries::QueryElement;

/// Alias for boxed error
pub type Error = Box<dyn std::error::Error>;

/// hyphae Prelude
///
/// Convenient module to import the most used imports for hyphae.
///
/// ```no_run
/// use hyphae::prelude::*;
/// ```
pub mod prelude {
    pub use hyphae::{
        assert_inner_text, assert_text_content,
        iter::*,
        queries::{
            by_aria::*, by_display_value::*, by_label_text::*, by_placeholder_text::*,
            by_selector::*, by_text::*, QueryElement,
        },
        Error,
    };
    pub use hyphae_aria::{property::*, role::*, state::*};
}
