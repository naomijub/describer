//! Helper crate that generates customizable Rust `Structs` describing strings.
#![doc = include_str!("../README.md")]

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, DataEnum, DeriveInput, Error, FieldsNamed, GenericArgument,
    Type, parse_macro_input, spanned::Spanned,
};

use crate::helpers::{
    get_explicit_collection, get_hide_name, get_hide_opt, is_keyval_collection,
    is_linear_collection, optional_type_path, set_tokens,
};

mod helpers;

#[proc_macro_derive(Describe, attributes(prettify))]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    let mut hide_opt = false;
    let mut explicit_collections = false;
    let mut separator = ", ".to_string();
    let mut spacing = " ".to_string();
    let mut keyval = ": ".to_string();
    let mut hide_name = false;

    if let Some(pretty) = attrs
        .iter()
        .find(|attr| attr.path().is_ident("prettify"))
        .and_then(|attr| attr.parse_args::<syn::Meta>().ok())
    {
        if let Some(err) = get_hide_opt(&mut hide_opt, &pretty) {
            return err;
        }
        if let Some(err) = get_explicit_collection(&mut explicit_collections, &pretty) {
            return err;
        }
        if let Some(err) = set_tokens(&mut separator, &mut spacing, &mut keyval, &pretty) {
            return err;
        }
        if let Some(err) = get_hide_name(&mut hide_name, &pretty) {
            return err;
        }
    }
    if !quote! {#generics}.is_empty() {
        return Error::new(
            generics.span(),
            "Generics not supported in the current version",
        )
        .to_compile_error()
        .into();
    }
    let field_names: Result<String, Error> = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named_struct(
                named,
                hide_opt,
                explicit_collections,
                &separator,
                &spacing,
                &keyval,
            ),
            syn::Fields::Unnamed(variant) => {
                // variant.unnamed.into_iter()
                Err(Error::new(
                    variant.span(),
                    "The Tuple Struct variant is not yet supported",
                ))
            }
            syn::Fields::Unit => Ok(String::new()),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            enum_variants(&ident, variants, &separator, &spacing)
        }
        _ => Err(Error::new(
            ident.span(),
            "The syn::Data::Union variant is not supported",
        )),
    };

    let field_names = match field_names {
        Ok(names) => names,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let expanded = quote! {
        impl #ident {
            fn describe() -> String {
                if (#field_names).is_empty() {
                    format!("{}",stringify!(#ident))
                } else if !#hide_name {
                    format!("{}{}{}",stringify!(#ident),#spacing, #field_names)
                } else {
                    format!("{}", #field_names)
                }

            }
        }
    };

    expanded.into()
}

fn enum_variants(
    ident: &syn::Ident,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    separator: &str,
    spacing: &str,
) -> Result<String, Error> {
    match variants
        .into_iter()
        .map(|var| {
            if var.fields.is_empty() {
                Ok(var.ident)
            } else {
                Err(Error::new(
                    ident.span(),
                    "Structured Enum not yet supported",
                ))
            }
        })
        .map(|var| {
            let var = var?;
            Ok(format!("{}", quote! {#var}))
        })
        .collect::<Result<Vec<String>, Error>>()
    {
        Ok(vars) => Ok(format!("#{{{spacing}{}{spacing}}}", vars.join(separator))),
        Err(err) => Err(err),
    }
}

fn named_struct(
    named: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    hide_opt: bool,
    explicit_collections: bool,
    separator: &str,
    spacing: &str,
    keyval: &str,
) -> Result<String, Error> {
    let fields = named
        .iter()
        .map(|f| &f.ident)
        .zip(named.iter().map(|f| &f.ty));

    match fields
        .map(|(name, ty)| {
            if let Type::Path(path) = ty
                && optional_type_path(path)
                && hide_opt
            {
                Ok(String::new())
            } else {
                let inner = flat_inner(ty, explicit_collections, separator)?;
                Ok(format!("{}{keyval}{}", quote!(#name), inner))
            }
        })
        .filter(|field| field.as_ref().map_or(true, |f| !f.is_empty()))
        .collect::<Result<Vec<_>, Error>>()
    {
        Ok(names) => Ok(format!("{{{spacing}{}{spacing}}}", names.join(separator))),
        Err(err) => Err(err),
    }
}

fn flat_inner(ty: &Type, explicit_collections: bool, separator: &str) -> Result<String, Error> {
    match ty {
        Type::Path(path) => {
            let segments = &path.path.segments;
            let inner = segments.first();
            match inner {
                None => Err(Error::new(ty.span(), "Type ident is required")),
                Some(inner) => {
                    let ident = inner.ident.clone();
                    let ident = format!("{}", quote! {#ident});

                    if ident == "Option" || ident == "qself" {
                        let value =
                            get_inner_arguments_type(inner, explicit_collections, separator)?;
                        Ok(value.strip_suffix('!').unwrap_or(&value).to_string())
                    } else if inner.arguments.is_empty() || inner.arguments.is_none() {
                        Ok(format!("{ident}!"))
                    } else if is_linear_collection(&ident) && !explicit_collections {
                        Ok(format!(
                            "[{}]!",
                            get_inner_arguments_type(inner, explicit_collections, separator)?
                        ))
                    } else if is_keyval_collection(&ident) && !explicit_collections {
                        Ok(format!(
                            "{{{}}}!",
                            get_inner_arguments_type(inner, explicit_collections, separator)?
                        ))
                    } else {
                        Ok(format!(
                            "{}<{}>!",
                            ident,
                            get_inner_arguments_type(inner, explicit_collections, separator)?
                        ))
                    }
                }
            }
        }
        variant => Err(Error::new(
            variant.span(),
            format!("Type {} not supported in current version", quote!(#ty)),
        )),
    }
}

fn get_inner_arguments_type(
    inner: &syn::PathSegment,
    explicit_collections: bool,
    separator: &str,
) -> Result<String, Error> {
    match &inner.arguments {
        syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => Ok(args
            .iter()
            .map(|arg| match arg {
                GenericArgument::Type(sub_type) => {
                    flat_inner(sub_type, explicit_collections, separator)
                }
                other => Err(Error::new(
                    other.span(),
                    "Generic SubType argument not yet supported",
                )),
            })
            .collect::<Result<Vec<String>, Error>>()?
            .join(separator)),
        syn::PathArguments::None => Ok(String::new()),
        argument => Err(Error::new(
            argument.span(),
            "Type argument not yet supported",
        )),
    }
}
