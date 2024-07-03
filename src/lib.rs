mod db;

#[proc_macro_derive(Db, attributes(db))]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let dm = db::parse_db_meta(&ast);
    let name = &dm.ident;
    println!("{:#?}", dm);

    quote::quote! {
        impl #name {

        }
    }
    .into()
}
