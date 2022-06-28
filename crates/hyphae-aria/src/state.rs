use crate::utils::*;

macro_rules! aria_state {
     ($(#[$enum_comment:meta])+ $enum_name:ident {$(
         $(#[$var_comment:meta])+ $var_name:ident($var_type:ty) => $implicit: expr
     ),*$(,)?}) => {
         $(#[$enum_comment])+
         pub enum $enum_name {
             $(
                 $(#[$var_comment])+
                 #[allow(dead_code)]
                 $var_name($var_type),
             )*
         }
         #[allow(deprecated)]
         impl ToQueryString for $enum_name {
             fn to_query_string(&self) -> String {
                 match self {
                     $(
                         $enum_name::$var_name(value) => format!("{}[aria-{}={}]",
                                 $implicit(value),
                                 stringify!($var_name).to_lowercase(),
                                 value.to_query_string(),
                             ),
                     )*
                 }
             }
         }
     };
 }

fn state_default<T>(_value: T) -> String {
    String::new()
}

fn state_checked(state: &TriState) -> String {
    if *state == TriState::True {
        "input[type=checkbox]:checked,input[type=radio]:checked,".to_string()
    } else {
        String::new()
    }
}

fn state_disabled(state: &bool) -> String {
    if *state {
        ":disabled,".to_string()
    } else {
        String::new()
    }
}

fn state_hidden(state: &DuoState) -> String {
    if *state == DuoState::True {
        ":hidden,".to_string()
    } else {
        String::new()
    }
}

aria_state! {
    /// A state is a dynamic property expressing characteristics of an object that may change
    /// in response to user action or automated processes.
    /// States do not affect the essential nature of the object, but represent data
    /// associated with the object or user interaction possibilities.
    AriaState {
        /// Indicates an element is being modified and that assistive technologies MAY want to
        /// wait until the modifications are complete before exposing them to the user.
        Busy(bool) => state_default,
        /**
        Indicates the current "checked" state of checkboxes, radio buttons,
        and other widgets.

        Attribute parity:
        - `input` elements of the following types that are checked:
            - `radio`
            - `checkbox`
        */
        Checked(TriState) => state_checked,
        /// Indicates the element that represents the current item within a container or set
        /// of related elements.
        Current(CurrentToken) => state_default,
        /**
        Indicates that the element is perceivable but disabled, so it is not editable or
        otherwise operable.

        Attribute parity:
        - any element which is disabled will be equivalent to aria-disabled="true"
        */
        Disabled(bool) => state_disabled,
        /// Indicates whether the element, or another grouping element it controls, is
        /// currently expanded or collapsed.
        Expanded(DuoState) => state_default,
        #[deprecated(note = "Deprecated in ARIA 1.1")]
        /// Indicates an element's "grabbed" state in a drag-and-drop operation.
        Grabbed(DuoState) => state_default,
        /**
        Indicates whether the element is exposed to an accessibility API.

        Attribute parity:
        - any element which is hidden will be equivalent to aria-hidden="true"
        */
        Hidden(DuoState) => state_hidden,
        /// Indicates the entered value does not conform to the format expected by the
        /// application.
        Invalid(InvalidToken) => state_default,
        /// Indicates the current "pressed" state of toggle buttons.
        Pressed(TriState) => state_default,
        /// Indicates the current "selected" state of various widgets.
        Selected(DuoState) => state_default,
    }
}

enum_to_lowercase_string_impl! {
    /// Value representing true, false, or not applicable.
    DuoState {
        /// Synonymous with boolean true
        True,
        /// Synonymous with boolean false
        False,
        /// Not applicable
        Undefined,
    }
}

enum_to_lowercase_string_impl! {
    /// Value representing true or false, with an intermediate "mixed" value. The default
    /// value for this value type is false unless otherwise specified.
    TriState {
        /// Synonymous with boolean false
        False,
        /// Intermediate value between true and false
        Mixed,
        /// Synonymous with boolean true
        True,
        /// Not defined
        Undefined,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the element that represents the current item within a container or set of
    /// related elements.
    CurrentToken {
        /// Represents the current page within a set of pages.
        Page,
        /// Represents the current step within a process.
        Step,
        /// Represents the current location within an environment or context.
        Location,
        /// Represents the current date within a collection of dates.
        Date,
        /// Represents the current time within a set of times.
        Time,
        /// Represents the current item within a set.
        True,
        /// Does not represent the current item within a set.
        False,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the entered value does not conform to the format expected by the application.
    InvalidToken {
        /// A grammatical error was detected.
        Grammar,
        /// There are no detected errors in the value.
        False,
        /// A spelling error was detected.
        Spelling,
        /// The value entered by the user has failed validation.
        True,
    }
}
