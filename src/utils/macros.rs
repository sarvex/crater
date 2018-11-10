macro_rules! string_enum {
    (pub enum $name:ident { $($item:ident => $str:expr,)* }) => {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum $name {
            $($item,)*
        }

        impl ::std::str::FromStr for $name {
            type Err = ::failure::Error;

            fn from_str(s: &str) -> ::failure::Fallible<$name> {
                match s {
                    $($str => Ok($name::$item),)*
                    s => bail!("invalid {}: {}", stringify!($name), s),
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }

        impl $name {
            pub fn to_str(&self) -> &'static str {
                match *self {
                    $($name::$item => $str,)*
                }
            }

            pub fn possible_values() -> &'static [&'static str] {
                &[$($str,)*]
            }
        }
    }
}

macro_rules! impl_serde_from_parse {
    ($for:ident, expecting=$expecting:expr) => {
        item! {
            struct [<$for Visitor>];

            impl<'de> ::serde::de::Visitor<'de> for [<$for Visitor>] {
                type Value = $for;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str($expecting)
                }

                fn visit_str<E: ::serde::de::Error>(self, input: &str) -> Result<$for, E> {
                    $for::from_str(input).map_err(E::custom)
                }
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $for {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_str(expr! { [<$for Visitor>] })
            }
        }

        impl ::serde::ser::Serialize for $for {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
}
