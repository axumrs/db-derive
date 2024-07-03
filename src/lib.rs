#[proc_macro_derive(Db)]
pub fn db_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("{:#?}", input);

    r#"
    impl User {
        pub fn hi(&self) -> &'static str {
            "Hello, axum.rs!"
        }
    }
    "#
    .parse()
    .unwrap()
}
