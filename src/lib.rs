mod db;

#[proc_macro_derive(Db, attributes(db))]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let dm = db::parse_db_meta(&ast);
    let name = &dm.ident;

    let insert_ts = db::insert_ts(&dm);
    let update_ts = db::update_ts(&dm);
    let del_ts = db::del_ts(&dm);

    let find_by_ts = db::find_by_ts(&dm);
    let find_ts = db::find_ts(&dm);

    let list_filter_ts = db::list_filter_ts(&dm);
    let list_ts = db::list_ts(&dm);

    quote::quote! {
        impl #name {
            #insert_ts
            #update_ts
            #del_ts
            #find_ts
            #list_ts
        }

        #find_by_ts
        #list_filter_ts
    }
    .into()
}
