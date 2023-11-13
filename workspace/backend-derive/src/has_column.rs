use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path};

pub fn impl_has_column(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let Data::Struct(ds) = ast.data else {
        panic!("Not a struct");
    };

    let Fields::Named(fields) = ds.fields else {
        panic!("No named fields");
    };

    let name = &ast.ident;

    let fields = fields.named.into_iter().filter_map(|f| {
        for attr in &f.attrs {
            if attr.path().is_ident("serde") {
                let Ok(param) = attr.parse_args::<Path>() else {
                    continue;
                };
                if param.is_ident("skip") {
                    return None;
                }
            }
        }

        Some(f.ident.expect("Fields should be named").to_string())
    });

    let out = quote! {
        impl crate::models::HasColumn for #name {
            fn has_column(col: &str) -> bool {
                [#(#fields),*].contains(&col)
            }
        }
    };

    out.into()
}
