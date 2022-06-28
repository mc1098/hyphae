use crate::utils::*;

macro_rules! aria_property {
    ($(#[$enum_comment:meta])+ $enum_name:ident {$( $(#[$var_comment:meta])+ $var_name:ident($var_type:ty)),*$(,)?}) => {
            $(#[$enum_comment])+
            pub enum $enum_name {
                $(
                    $(#[$var_comment])+
                    #[allow(dead_code, deprecated)]
                    $var_name($var_type),
                )*
            }

            #[allow(deprecated)]
            impl ToQueryString for $enum_name {
                fn to_query_string(&self) -> String {
                    match self {
                        $(
                            $enum_name::$var_name(value) => format!("[aria-{}={}]",
                                    stringify!($var_name).to_lowercase(),
                                    value.to_query_string(),
                                ),
                        )*
                    }
                }
            }
        };
}

aria_property! {
    /// Attributes that are essential to the nature of a given object, or that represent a
    /// data value associated with the object. A change of a property may significantly
    /// impact the meaning or presentation of an object. Certain properties (for example,
    /// aria-multiline) are less likely to change than states, but note that the frequency of
    /// change difference is not a rule. A few properties, such as aria-activedescendant,
    /// aria-valuenow, and aria-valuetext are expected to change often.
    AriaProperty {
        /// Indicates whether assistive technologies will present all, or only parts of,
        /// the changed region based on the change notifications defined by the aria-relevant
        /// attribute.
        Atomic(bool),
        /// Identifies the element (or elements) whose contents or presence are controlled by
        /// the current element.
        Controls(IdReferenceList),
        /// Identifies the element (or elements) that describes the object.
        DescribedBy(IdReferenceList),
        /// Identifies the element that provides a detailed, extended description for the object.
        Details(IdReference),
        #[deprecated(note = "Deprecated in ARIA 1.1")]
        /// Indicates what functions can be performed when a dragged object is released on the drop target.
        DropEffect(TokenList<DropEffectToken>), //(&'a [DropEffectToken]),
        /// Identifies the element that provides an error message for the object.
        ErrorMessage(IdReference),
        /// Identifies the currently active element when DOM focus is on a composite widget,
        /// textbox, group, or application.
        ActiveDescendant(IdReference),
        /// Indicates whether inputting text could trigger display of one or more predictions
        /// of the user's intended value for an input and specifies how predictions would be
        /// presented if they are made.
        AutoComplete(AutoCompleteToken),
        /// Defines the total number of columns in a table, grid, or treegrid.
        ColCount(i32),
        /// Defines an element's column index or position with respect to the total number of
        /// columns within a table, grid, or treegrid.
        ColIndex(i32),
        /// Defines the number of columns spanned by a cell or gridcell within a table, grid,
        /// or treegrid.
        ColSpan(i32),
        /// Identifies the next element (or elements) in an alternate reading order of content
        /// which, at the user's discretion, allows assistive technology to override the
        /// general default of reading in document source order.
        FlowTo(IdReferenceList),
        /// Indicates the availability and type of interactive popup element, such as menu or
        /// dialog, that can be triggered by an element.
        HasPopup(HasPopupToken),
        /// Indicates keyboard shortcuts that an author has implemented to activate or give
        /// focus to an element.
        KeyShortcuts(String),
        /// Defines a string value that labels the current element.
        Label(String),
        /// Identifies the element (or elements) that labels the current element.
        LabelledBy(IdReferenceList),
        /// Defines the hierarchical level of an element within a structure.
        Level(i32),
        /// Indicates that an element will be updated, and describes the types of updates the
        /// user agents, assistive technologies, and user can expect from the live region.
        Live(LiveToken),
        /// Indicates whether an element is modal when displayed.
        Modal(bool),
        /// Indicates whether a text box accepts multiple lines of input or only a single line.
        MultiLine(bool),
        /// Indicates that the user may select more than one item from the current selectable
        /// descendants.
        MultiSelectable(bool),
        /// Indicates whether the element's orientation is horizontal, vertical, or
        /// unknown/ambiguous.
        Orientation(OrientationToken),
        /// Identifies an element (or elements) in order to define a visual, functional, or
        /// contextual parent/child relationship between DOM elements where the DOM hierarchy
        /// cannot be used to represent the relationship.
        Owns(IdReferenceList),
        /// Defines a short hint (a word or short phrase) intended to aid the user with data
        /// entry when the control has no value. A hint could be a sample value or a brief
        /// description of the expected format.
        Placeholder(String),
        /// Defines an element's number or position in the current set of listitems or
        /// treeitems. Not required if all elements in the set are present in the DOM.
        PosInSet(i32),
        /// Indicates that the element is not editable, but is otherwise operable.
        ReadOnly(bool),
        /// Indicates what notifications the user agent will trigger when the accessibility
        /// tree within a live region is modified.
        Relevant(TokenList<RelevantToken>),
        /// Indicates that user input is required on the element before a form may be submitted.
        Required(bool),
        /// Defines a human-readable, author-localized description for the role of an element.
        RoleDescription(String),
        /// Defines the total number of rows in a table, grid, or treegrid.
        RowCount(i32),
        /// Defines an element's row index or position with respect to the total number of rows
        /// within a table, grid, or treegrid.
        RowIndex(i32),
        /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or
        /// treegrid.
        RowSpan(i32),
        /// Defines the number of items in the current set of listitems or treeitems.
        /// Not required if all elements in the set are present in the DOM.
        SetSize(i32),
        /// Indicates if items in a table or grid are sorted in ascending or descending order.
        Sort(SortToken),
        /// Defines the maximum allowed value for a range widget.
        ValueMax(f32),
        /// Defines the minimum allowed value for a range widget.
        ValueMin(f32),
        /// Defines the current value for a range widget.
        ValueNow(f32),
        /// Defines the human readable text alternative of aria-valuenow for a range widget.
        ValueText(String),
    }
}

enum_to_lowercase_string_impl! {
    #[deprecated(note = "Deprecated in ARIA 1.1")]
    /// Indicates what functions can be performed when a dragged object is released on the drop target.
    DropEffectToken {
        /// A duplicate of the source object will be dropped into the target.
        Copy,
        /// A function supported by the drop target is executed, using the drag source as an input.
        Execute,
        /// A reference or shortcut to the dragged object will be created in the target object.
        Link,
        /// The source object will be removed from its current location and dropped into the target.
        Move,
        /// No operation can be performed; effectively cancels the drag operation if an attempt
        /// is made to drop on this object. Ignored if combined with any other token value.
        /// e.g., 'none copy' is equivalent to a 'copy' value.
        None,
        /// There is a popup menu or dialog that allows the user to choose one of the drag
        /// operations (copy, move, link, execute) and any other drag functionality,
        /// such as cancel.
        Popup,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates whether inputting text could trigger display of one or more predictions of
    /// the user's intended value for an input and specifies how predictions would be presented
    /// if they are made.
    AutoCompleteToken {
        /// When a user is providing input, text suggesting one way to complete the provided
        /// input may be dynamically inserted after the caret.
        Inline,
        /// When a user is providing input, an element containing a collection of values that
        /// could complete the provided input may be displayed.
        List,
        /// When a user is providing input, an element containing a collection of values that
        /// could complete the provided input may be displayed. If displayed, one value in the
        /// collection is automatically selected, and the text needed to complete the
        /// automatically selected value appears after the caret in the input.
        Both,
        /// When a user is providing input, an automatic suggestion that attempts to predict
        /// how the user intends to complete the input is not displayed.
        None,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the availability and type of interactive popup element, such as menu or
    /// dialog, that can be triggered by an element.
    HasPopupToken {
        /// Indicates the element does not have a popup.
        False,
        /// Indicates the popup is a menu.
        True,
        /// Indicates the popup is a menu.
        Menu,
        /// Indicates the popup is a listbox.
        ListBox,
        /// Indicates the popup is a tree.
        Tree,
        /// Indicates the popup is a grid.
        Grid,
        /// Indicates the popup is a dialog.
        Dialog,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    LiveToken {
        /// Indicates that updates to the region have the highest priority and should be
        /// presented the user immediately.
        Assertive,
        /// Indicates that updates to the region should not be presented to the user unless
        /// the used is currently focused on that region.
        Off,
        /// Indicates that updates to the region should be presented at the next graceful
        /// opportunity, such as at the end of speaking the current sentence or when the user
        /// pauses typing.
        Polite,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates whether the element's orientation is horizontal, vertical, or
    /// unknown/ambiguous.
    OrientationToken {
        /// The element is oriented horizontally.
        Horizontal,
        /// The element's orientation is unknown/ambiguous.
        Undefined,
        /// The element is oriented vertically.
        Vertical,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates what notifications the user agent will trigger when the accessibility tree
    /// within a live region is modified. See related aria-atomic.
    RelevantToken {
        /// Element nodes are added to the accessibility tree within the live region.
        Additions,
        /// Equivalent to the combination of values, "additions text".
        AdditionsText,
        /// Equivalent to the combination of all values, "additions removals text".
        All,
        /// Text content, a text alternative, or an element node within the live region is
        /// removed from the accessibility tree.
        Removals,
        /// Text content or a text alternative is added to any descendant in the accessibility
        /// tree of the live region.
        Text,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    SortToken {
        /// Items are sorted in ascending order by this column.
        Ascending,
        /// Items are sorted in descending order by this column.
        Descending,
        /// There is no defined sort applied to the column.
        None,
        /// A sort algorithm other than ascending or descending has been applied.
        Other,
    }
}

/// Reference to the ID of another element in the same document
pub type IdReference = String;

/// A list of one or more [`IdReference`]s.
pub struct IdReferenceList(Vec<String>);

impl ToQueryString for IdReferenceList {
    fn to_query_string(&self) -> String {
        self.0.join(" ")
    }
}

impl<S> From<S> for IdReferenceList
where
    S: AsRef<[String]>,
{
    fn from(slice: S) -> Self {
        IdReferenceList(slice.as_ref().to_owned())
    }
}

/// A list of one or more tokens.
pub struct TokenList<T>(Vec<T>);

impl<'a, S, T> From<&'a S> for TokenList<T>
where
    S: AsRef<[T]>,
    T: ToQueryString + Copy,
{
    fn from(slice: &'a S) -> Self {
        TokenList(slice.as_ref().to_vec())
    }
}

impl<T> ToQueryString for TokenList<T>
where
    T: ToQueryString,
{
    fn to_query_string(&self) -> String {
        self.0
            .iter()
            .map(ToQueryString::to_query_string)
            .fold(String::new(), |mut acc, t| {
                if !acc.is_empty() {
                    acc.push(',')
                }
                acc.push_str(&t);
                acc
            })
    }
}
