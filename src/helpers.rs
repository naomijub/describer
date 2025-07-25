use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{Error, ExprLit, Lit, LitBool, LitStr, TypePath, spanned::Spanned};

pub fn get_hide_opt(hide_opt: &mut bool, pretty: &syn::Meta) -> Option<TokenStream> {
    if let syn::Meta::NameValue(named) = pretty
        && named.path.is_ident("hide_opt")
    {
        if named.path.segments.len() > 1 {
            return Some(
                Error::new(
                    named.path.span(),
                    "Only single paths segments are permitted",
                )
                .to_compile_error()
                .into(),
            );
        }
        if let syn::Expr::Lit(ExprLit {
            lit: Lit::Bool(LitBool { value, .. }),
            ..
        }) = named.value
        {
            *hide_opt = value;
        } else {
            return Some(
                Error::new(named.value.span(), "`hide_opt` supports only boolean types")
                    .to_compile_error()
                    .into(),
            );
        };
    }
    None
}

pub fn get_explicit_collection(
    explicit_collections: &mut bool,
    pretty: &syn::Meta,
) -> Option<TokenStream> {
    if let syn::Meta::NameValue(named) = pretty
        && named.path.is_ident("explicit_collections")
    {
        if named.path.segments.len() > 1 {
            return Some(
                Error::new(
                    named.path.span(),
                    "Only single paths segments are permitted",
                )
                .to_compile_error()
                .into(),
            );
        }
        if let syn::Expr::Lit(ExprLit {
            lit: Lit::Bool(LitBool { value, .. }),
            ..
        }) = named.value
        {
            *explicit_collections = value;
        } else {
            return Some(
                Error::new(
                    named.value.span(),
                    "`explicit_collections` supports only boolean types",
                )
                .to_compile_error()
                .into(),
            );
        };
    }
    None
}

pub fn set_tokens(
    separator: &mut String,
    spacing: &mut String,
    keyval: &mut String,
    pretty: &syn::Meta,
) -> Option<TokenStream> {
    if let syn::Meta::List(list) = pretty
        && list.path.is_ident("tokens")
    {
        let res = list.parse_nested_meta(|meta| {
            if meta.path.is_ident("separator") {
                let Ok(value) = meta.value() else {
                    return Ok(());
                };
                let Some(sep): Option<LitStr> = value.parse().ok() else {
                    return Err(Error::new(
                        meta.path.span(),
                        "`separator` supports only &str types",
                    ));
                };
                *separator = sep.value();
            }

            if meta.path.is_ident("spacing") {
                let Ok(value) = meta.value() else {
                    return Ok(());
                };
                let Some(sp): Option<LitStr> = value.parse().ok() else {
                    return Err(Error::new(
                        meta.path.span(),
                        "`spacing` supports only &str types",
                    ));
                };
                *spacing = sp.value();
            }

            if meta.path.is_ident("keyval") {
                let Ok(value) = meta.value() else {
                    return Ok(());
                };
                let Some(sp): Option<LitStr> = value.parse().ok() else {
                    return Err(Error::new(
                        meta.path.span(),
                        "`keyval` supports only &str types",
                    ));
                };
                *keyval = sp.value();
            }
            Ok(())
        });

        if let Err(err) = res {
            return Some(err.to_compile_error().into());
        }
    }
    None
}

pub fn optional_type_path(path: &TypePath) -> bool {
    path.path.segments.first().is_some_and(|ident| {
        let ident = &ident.ident;
        let ident = format!("{}", quote! {#ident});
        ident.starts_with("Option")
    })
}

const LINEAR_COLLECTIONS: &[&str] = &["Vec", "HashSet", "BTreeSet", "IndexSet"];
pub fn is_linear_collection(ident: &str) -> bool {
    LINEAR_COLLECTIONS.contains(&ident)
}

const KEYVAL_COLLECTIONS: &[&str] = &["HashMap", "BTreeMap", "IndexMap"];
pub fn is_keyval_collection(ident: &str) -> bool {
    KEYVAL_COLLECTIONS.contains(&ident)
}
