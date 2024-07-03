#[proc_macro_derive(Db)]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("{:#?}", input);

    r#"{println!("Hello, axum.rs!");}"#.parse().unwrap()
}
