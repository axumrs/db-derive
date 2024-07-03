#[proc_macro_derive(Db, attributes(db))]
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

    let field_idents = fileds
        .iter()
        .map(|f| f.ident.clone().unwrap().clone())
        .collect::<Vec<_>>();

    let field_types = fileds.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();

    let setter_idents = field_idents
        .iter()
        .map(|f| {
            let ident_str = format!("set_{}", f.to_string());
            syn::Ident::new(&ident_str, f.span())
        })
        .collect::<Vec<_>>();

    quote::quote! {
        impl #name {
          #(
            pub fn #field_idents(&self) -> &#field_types {
                &self.#field_idents
            }
          )*

          #(
            pub fn #setter_idents(&mut self, v:#field_types) {
                self.#field_idents = v;
            }
          )*
        }
    }
    .into()
}
