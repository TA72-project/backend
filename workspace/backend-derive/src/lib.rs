use proc_macro::TokenStream;

mod has_column;

/// Implement the `HasColumn` trait.
///
/// Every struct field is considered a column.
/// Fields marked with `serde(skip)` are not included.
#[proc_macro_derive(HasColumn)]
pub fn has_column_derive(input: TokenStream) -> TokenStream {
    has_column::impl_has_column(input)
}
