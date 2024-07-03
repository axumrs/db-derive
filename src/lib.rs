#[proc_macro_derive(Db)]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = ast.ident;

    quote::quote! {
        impl #name {
            pub fn hi(&self) -> &'static str {
                "Hello, axum.rs!"
            }
        }
    }
    .into()
}
