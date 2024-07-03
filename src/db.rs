use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Field, Ident, LitStr,
};

#[derive(Debug)]
pub(crate) struct DbMeta {
    pub(crate) ident: Ident,
    pub(crate) table: String,
    pub(crate) pk: String,
    pub(crate) is_view: bool,
}

pub(crate) struct DbMetaParser {
    pub(crate) table: Option<LitStr>,
    pub(crate) pk: Option<LitStr>,
    pub(crate) is_view: bool,
}

impl std::default::Default for DbMetaParser {
    fn default() -> Self {
        Self {
            table: None,
            pk: None,
            is_view: false,
        }
    }
}

/// 解析字段
pub(crate) fn parse_fields(ast: &DeriveInput) -> Punctuated<Field, Comma> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }, ..),
        ..
    }) = &ast.data
    {
        named.to_owned()
    } else {
        unreachable!()
    }
}

/// 解析数据表元数据
pub(crate) fn parse_db_meta(ast: &DeriveInput) -> DbMeta {
    let mut dmp = DbMetaParser::default();
    let ident = ast.ident.clone();
    let ident_str = ident.to_string();
    let table = _gen_table_name(&ident_str);
    let mut dm = DbMeta {
        ident,
        table,
        pk: "id".to_string(),
        is_view: false,
    };

    for a in ast.attrs.iter() {
        if let syn::Meta::List(syn::MetaList { path, tokens, .. }) = &a.meta {
            if let Some(seg) = path.segments.first() {
                if seg.ident == "db" {
                    _db_meta_parser(tokens, &mut dmp);
                    if let Some(v) = &dmp.table {
                        dm.table = v.token().to_string().replace("\"", "");
                    }
                    if let Some(v) = &dmp.pk {
                        dm.pk = v.token().to_string().replace("\"", "");
                    }
                    dm.is_view = dmp.is_view;
                }
            }
        }
    }
    dm
}

/// 生成表名
fn _gen_table_name(tn: &str) -> String {
    let mut ss = String::new();

    for (idx, c) in tn.chars().enumerate() {
        if idx == 0 {
            ss.push(c);
            continue;
        }
        if c.is_lowercase() {
            ss.push(c);
            continue;
        }
        if c.is_uppercase() {
            ss.extend(['_', c]);
            continue;
        }
    }

    ss.push('s');

    ss.to_lowercase()
}

/// 解析元数据
fn _db_meta_parser(
    tokens: &proc_macro2::TokenStream,
    dmp: &mut DbMetaParser,
) -> proc_macro::TokenStream {
    let parser = syn::meta::parser(|mt| {
        if mt.path.is_ident("table") {
            dmp.table = Some(mt.value()?.parse()?);
            return Ok(());
        }
        if mt.path.is_ident("pk") {
            dmp.pk = Some(mt.value()?.parse()?);
            return Ok(());
        }
        if mt.path.is_ident("is_view") {
            dmp.is_view = true;
            return Ok(());
        }

        Ok(())
    });

    let tokens = tokens.to_owned().into();
    parse_macro_input!(tokens with parser);

    quote! {}.into()
}
