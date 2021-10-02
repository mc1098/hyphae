pub trait ToQueryString {
    fn to_query_string(&self) -> String;
}

// blanket impl for 'primitive' types that have ToString.
impl<S> ToQueryString for S
where
    S: ToString,
{
    fn to_query_string(&self) -> String {
        self.to_string()
    }
}

macro_rules! enum_to_lowercase_string_impl {
    (
        $(#[$enum_comment:meta])+
        $enum_name:ident {
            $( $(#[$var_comment:meta])+
                $variant:ident,)*$(,)?
        }
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        $(#[$enum_comment])+
        pub enum $enum_name {
            $(
                $(#[$var_comment])+
                $variant,
            )*
        }

        #[allow(deprecated)]
        impl ToQueryString for $enum_name {
            fn to_query_string(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant => stringify!($variant).to_lowercase(),
                    )*
                }
            }
        }
    };
}

pub(crate) use enum_to_lowercase_string_impl;
