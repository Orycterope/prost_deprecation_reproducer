#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shirt {
    #[deprecated]
    #[prost(enumeration = "Color", tag = "1")]
    pub deprecated_color: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Color {
    Blue = 0,
    Red = 1,
    Green = 2,
}
impl Color {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Color::Blue => "BLUE",
            Color::Red => "RED",
            Color::Green => "GREEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLUE" => Some(Self::Blue),
            "RED" => Some(Self::Red),
            "GREEN" => Some(Self::Green),
            _ => None,
        }
    }
}
