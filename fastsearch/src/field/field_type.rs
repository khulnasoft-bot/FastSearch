use std::collections::{BTreeMap, HashMap};

/// Type for a field. Currently it is a wrapping to a `String` but it could be extended to a enum
pub type FieldType = String;

/// Trait that should implement each type of a document, in order to properly serialize the
/// Collection Schema according to the Fastsearch reference.
pub trait ToFastsearchField {
    /// Static function that should implement the types of the fastsearch documents.
    fn to_fastsearch_type() -> &'static str;
}

/// macro used internally to add implementations of ToFastsearchField for several rust types.
#[macro_export]
macro_rules! impl_to_fastsearch_field (
    ($for:ty, $fastsearch_variant:expr) => {
        impl $crate::field::ToFastsearchField for $for {
            #[inline(always)]
            fn to_fastsearch_type() -> &'static str {
                $fastsearch_variant
            }
        }
    };
    ($for:ty, $fastsearch_variant:expr, $any:ident) => {
        impl<$any> $crate::field::ToFastsearchField for $for {
            #[inline(always)]
            fn to_fastsearch_type() -> &'static str {
                $fastsearch_variant
            }
        }
    };
);

impl_to_fastsearch_field!(String, "string");
impl_to_fastsearch_field!(i8, "int32");
impl_to_fastsearch_field!(u8, "int32");
impl_to_fastsearch_field!(i16, "int32");
impl_to_fastsearch_field!(u16, "int32");
impl_to_fastsearch_field!(i32, "int32");
impl_to_fastsearch_field!(u32, "int64");
impl_to_fastsearch_field!(i64, "int64");
impl_to_fastsearch_field!(u64, "int64");
impl_to_fastsearch_field!(isize, "int64");
impl_to_fastsearch_field!(usize, "int64");
impl_to_fastsearch_field!(f32, "float");
impl_to_fastsearch_field!(f64, "float");
impl_to_fastsearch_field!(bool, "bool");
impl_to_fastsearch_field!(HashMap<String, T>, "object", T);
impl_to_fastsearch_field!(BTreeMap<String, T>, "object", T);

impl_to_fastsearch_field!(Vec<String>, "string[]");
impl_to_fastsearch_field!(Vec<i8>, "int32[]");
impl_to_fastsearch_field!(Vec<u8>, "int32[]");
impl_to_fastsearch_field!(Vec<i16>, "int32[]");
impl_to_fastsearch_field!(Vec<u16>, "int32[]");
impl_to_fastsearch_field!(Vec<i32>, "int32[]");
impl_to_fastsearch_field!(Vec<u32>, "int64[]");
impl_to_fastsearch_field!(Vec<i64>, "int64[]");
impl_to_fastsearch_field!(Vec<u64>, "int64[]");
impl_to_fastsearch_field!(Vec<isize>, "int64[]");
impl_to_fastsearch_field!(Vec<usize>, "int64[]");
impl_to_fastsearch_field!(Vec<f32>, "float[]");
impl_to_fastsearch_field!(Vec<f64>, "float[]");
impl_to_fastsearch_field!(Vec<bool>, "bool[]");
impl_to_fastsearch_field!(Vec<HashMap<String, T>>, "object[]", T);
impl_to_fastsearch_field!(Vec<BTreeMap<String, T>>, "object[]", T);
