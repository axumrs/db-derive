#[proc_macro_derive(Db)]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = ast.ident;

    let fileds = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }, ..),
        ..
    }) = ast.data
    {
        named
    } else {
        unreachable!()
    };

    let field_str = fileds
        .iter()
        .map(|f| f.ident.clone().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");

    quote::quote! {
        impl #name {
            pub fn hi(&self) -> &'static str {
                #field_str
            }
        }
    }
    .into()
}
