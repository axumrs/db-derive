use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Field, Ident, LitStr,
    Type,
};

#[derive(Debug)]
pub(crate) struct DbMeta {
    pub(crate) ident: Ident,
    pub(crate) table: String,
    pub(crate) pk: String,
    pub(crate) is_view: bool,
    pub(crate) fields: Vec<DbField>,
}

impl DbMeta {
    pub(crate) fn insert_fileds(&self) -> Vec<Ident> {
        self.fields
            .iter()
            .filter(|f| f.skip_insert == false)
            .map(|f| f.name.clone())
            .collect()
    }
    pub(crate) fn update_fileds(&self) -> Vec<Ident> {
        self.fields
            .iter()
            .filter(|f| f.skip_update == false)
            .map(|f| f.name.clone())
            .collect()
    }

    pub(crate) fn pk_ident(&self) -> Ident {
        Ident::new(&self.pk, self.ident.clone().span())
    }
    pub(crate) fn pk_type(&self) -> Type {
        self.fields
            .iter()
            .find(|f| f.name.to_string() == self.pk)
            .take()
            .unwrap()
            .ty
            .clone()
    }
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

#[derive(Debug)]
pub(crate) struct DbField {
    pub(crate) name: Ident,
    pub(crate) ty: Type,
    pub(crate) skip_update: bool,
    pub(crate) skip_insert: bool,
    pub(crate) find: bool,
    pub(crate) find_opt: bool,
    pub(crate) list: bool,
    pub(crate) list_opt: bool,
    pub(crate) opt_like: bool,
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
        fields: vec![],
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

    // 解析字段
    let meta_fields = parse_fields(ast);

    let mut fields = vec![];

    // 字段属性
    for f in meta_fields {
        let name = f.ident.clone();
        let ty = f.ty.clone();
        let attrs = f
            .attrs
            .clone()
            .into_iter()
            .filter(|a| a.path().is_ident("db"))
            .collect::<Vec<_>>();

        let mut db_field = DbField {
            name: name.unwrap(),
            ty,
            skip_insert: false,
            skip_update: false,
            find: false,
            find_opt: false,
            list: false,
            list_opt: false,
            opt_like: false,
        };

        // 解析字段属性
        for a in attrs.iter() {
            a.parse_nested_meta(|mt| {
                if mt.path.is_ident("skip_update") {
                    db_field.skip_update = true;
                    return Ok(());
                }
                if mt.path.is_ident("skip_insert") {
                    db_field.skip_insert = true;
                    return Ok(());
                }
                if mt.path.is_ident("find") {
                    db_field.find = true;
                    return Ok(());
                }
                if mt.path.is_ident("find_opt") {
                    db_field.find_opt = true;
                    return Ok(());
                }
                if mt.path.is_ident("list") {
                    db_field.list = true;
                    return Ok(());
                }
                if mt.path.is_ident("list_opt") {
                    db_field.list_opt = true;
                    return Ok(());
                }
                if mt.path.is_ident("opt_like") {
                    db_field.opt_like = true;
                    return Ok(());
                }

                Ok(())
            })
            .unwrap();
        }
        fields.push(db_field);
    }

    dm.fields = fields;

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

pub(crate) fn insert_ts(dm: &DbMeta) -> proc_macro2::TokenStream {
    if dm.is_view {
        panic!("视图不提供插入方法");
    }
    let field_list = dm.insert_fileds();
    let field_list_str = field_list
        .iter()
        .map(|f| format!(r#""{}""#, f.to_string()))
        .collect::<Vec<_>>()
        .join(",");
    let table = dm.table.clone();
    let sql = format!("INSERT INTO {:?} ({})", &table, &field_list_str);
    let pk = dm.pk_ident();
    let pk_type = dm.pk_type();

    quote! {
        pub async fn insert<'a>(&self, e: impl  ::sqlx::PgExecutor<'a>) -> ::sqlx::Result<#pk_type> {
            let id = self.#pk.clone();
           let sql = #sql;
           let mut q = ::sqlx::QueryBuilder::new(sql);
           q.push_values(&[self], |mut b, m| {
                #(b.push_bind(&m.#field_list);)*
           });
           q.build().execute(e).await?;
            Ok(id)
        }
    }
}

pub(crate) fn update_ts(dm: &DbMeta) -> proc_macro2::TokenStream {
    let field_list = dm.update_fileds();
    let field_list_str = field_list
        .iter()
        .map(|f| format!("{:?} = ", f.to_string()))
        .collect::<Vec<_>>();
    let field_list_com = field_list
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("{}", if idx < field_list.len() - 1 { ", " } else { "" }))
        .collect::<Vec<_>>();

    let table = dm.table.clone();
    let sql = format!("UPDATE {:?} SET ", &table,);
    let pk = dm.pk_ident().clone();
    let pk_str = pk.to_string();

    quote! {
        pub async fn update<'a>(&self, e: impl  ::sqlx::PgExecutor<'a>) -> ::sqlx::Result<u64> {
            let sql = #sql;
            let mut q = ::sqlx::QueryBuilder::new(sql);
            #(
                q.push(#field_list_str)
                .push_bind(&self.#field_list)
                .push(#field_list_com);
            )*

            q.push(format!(" WHERE {} = ", #pk_str)).push_bind(&self.#pk);

            let aff = q.build().execute(e).await?.rows_affected();
            Ok(aff)
        }
    }
}
pub(crate) fn del_ts(dm: &DbMeta) -> proc_macro2::TokenStream {
    let table = dm.table.clone();
    let pk = dm.pk_ident().clone();
    let pk_str = pk.to_string();
    let sql = format!("DELETE FROM {:?} WHERE {:?} = ", &table, &pk_str);

    quote! {
        pub async fn delete<'a>(&self, e: impl  ::sqlx::PgExecutor<'a>) -> ::sqlx::Result<u64> {
            let sql = #sql;
            let mut q = ::sqlx::QueryBuilder::new(sql);
            q.push_bind(&self.#pk);
            let aff = q.build().execute(e).await?.rows_affected();
            Ok(aff)
        }
    }
}
