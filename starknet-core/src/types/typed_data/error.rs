use alloc::string::*;
use core::fmt::Display;

use super::{revision::Revision, value::ValueKind};

/// Possible errors when processing [`TypedData`](super::TypedData) and its related types.
#[derive(Debug)]
pub enum TypedDataError {
    /// Revision implied by `types` is different from revision specified by `domain`.
    InconsistentRevision {
        /// The revision implied from `types` with the domain type definition.
        types: Revision,
        /// The revision specified by `domain`.
        domain: Revision,
    },
    /// The type name is invalid.
    InvalidTypeName(
        /// Type name.
        String,
    ),
    /// The `contains` field exists when it's expected to be absent.
    UnexpectedContainsField,
    /// A referenced custom type is not defined.
    CustomTypeNotFound(String),
    /// An expected field is not found.
    FieldNotFound(
        /// Field name.
        String,
    ),
    /// The value is of a different type than expected>
    UnexpectedValueType {
        /// The list of expected value types.
        expected: &'static [ValueKind],
        /// The actual value type.
        actual: ValueKind,
    },
    /// The number of fields from struct definition is different from the one in value.
    StructFieldCountMismatch {
        /// The number of fields specified by the struct definition.
        expected: usize,
        /// The actual number of fields found in value.
        actual: usize,
    },
    /// The number of elements from enum variant definition is different from the one in value.
    EnumElementCountMismatch {
        /// The number of elements specified by the enum variant definition.
        expected: usize,
        /// The actual number of elements found in value.
        actual: usize,
    },
    /// The object representation of an enum value does not have exactly one field.
    InvalidEnumFieldCount,
    /// The variant name is not found in the enum definition.
    EnumVariantNotFound(
        /// Variant name.
        String,
    ),
    /// Found a struct when an enum is expected.
    UnexpectedStruct(
        /// Name of the struct type.
        String,
    ),
    /// Found an enum when a struct is expected.
    UnexpectedEnum(
        /// Name of the enum type.
        String,
    ),
    /// A Cairo short string cannot be parsed.
    InvalidShortString(
        /// The Cairo short string.
        String,
    ),
    /// Invalid function selector.
    InvalidSelector(
        /// The function selector.
        String,
    ),
    /// The string value cannot be parsed into a number.
    InvalidNumber(
        /// The string value.
        String,
    ),
    /// The Merkle tree is empty.
    EmptyMerkleTree,
}

#[cfg(feature = "std")]
impl std::error::Error for TypedDataError {}

impl Display for TypedDataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InconsistentRevision { types, domain } => {
                write!(
                    f,
                    "`types` implies revision {types} but `domain` uses revision {domain}"
                )
            }
            Self::InvalidTypeName(type_name) => write!(f, "invalid type name: {type_name}"),
            Self::UnexpectedContainsField => {
                write!(f, "unexpected presence of the `contains` field")
            }
            Self::CustomTypeNotFound(type_name) => {
                write!(f, "type `{type_name}` not defined")
            }
            Self::FieldNotFound(field_name) => {
                write!(f, "field `{field_name}` not found in value")
            }
            Self::UnexpectedValueType { expected, actual } => {
                write!(f, "unexpected value type {actual}, expecting")?;

                let mut kind_iter = expected.iter().peekable();
                while let Some(kind) = kind_iter.next() {
                    write!(f, " {kind}")?;
                    if kind_iter.peek().is_some() {
                        write!(f, ",")?;
                    }
                }
                Ok(())
            }
            Self::StructFieldCountMismatch { expected, actual } => {
                write!(f, "expected {expected} fields in struct but found {actual}")
            }
            Self::EnumElementCountMismatch { expected, actual } => {
                write!(
                    f,
                    "expected {expected} elements in enum variant but found {actual}"
                )
            }
            Self::InvalidEnumFieldCount => {
                write!(f, "enum values must have 1 and only 1 field")
            }
            Self::EnumVariantNotFound(variant_name) => {
                write!(f, "enum variant `{variant_name}` not defined")
            }
            Self::UnexpectedStruct(type_name) => {
                write!(f, "expected type `{type_name}` to be enum but is struct")
            }
            Self::UnexpectedEnum(type_name) => {
                write!(f, "expected type `{type_name}` to be struct but is enum")
            }
            Self::InvalidShortString(short_string) => {
                write!(f, "\"{short_string}\" is not a valid Cairo short string")
            }
            Self::InvalidSelector(selector) => {
                write!(f, "\"{selector}\" is not a valid function selector")
            }
            Self::InvalidNumber(string_repr) => {
                write!(f, "\"{string_repr}\" is not a valid number")
            }
            Self::EmptyMerkleTree => {
                write!(f, "`merkletree` values must not be empty")
            }
        }
    }
}
