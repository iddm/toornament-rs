use chrono::NaiveDate;
use std::fmt;

/// A common type for toornament dates.
pub type Date = NaiveDate;

macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de>
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                        where E: serde::de::Error
                    {
                        // Rust does not come with a simple way of converting a
                        // number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}

/// Team size bounds (minimum and maximum).
#[derive(
    Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct TeamSize {
    /// Minimum team size
    pub min: i64,
    /// Maximum team size
    pub max: i64,
}

enum_number!(MatchResultSimple {
    Win = 1,
    Draw = 2,
    Loss = 3,
});
