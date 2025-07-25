//! Helper crate that generates customizable Rust `Structs` describing strings.
#![doc = include_str!("../README.md")]

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, DataEnum, DeriveInput, Error, FieldsNamed, GenericArgument,
    Type, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

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
            syn::Fields::Named(FieldsNamed { named, .. }) => named_struct(named),
            syn::Fields::Unnamed(variant) => {
                // variant.unnamed.into_iter()
                Err(Error::new(
                    variant.span(),
                    "The Tuple Struct variant is not yet supported",
                ))
            }
            syn::Fields::Unit => Ok(String::new()),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => enum_variants(&ident, variants),
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
                } else {
                    format!("{} {}",stringify!(#ident), #field_names)
                }

            }
        }
    };

    expanded.into()
}

fn enum_variants(
    ident: &syn::Ident,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
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
        Ok(vars) => Ok(format!("#{{ {} }}", vars.join(", "))),
        Err(err) => Err(err),
    }
}

fn named_struct(
    named: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Result<String, Error> {
    let fields = named
        .iter()
        .map(|f| &f.ident)
        .zip(named.iter().map(|f| &f.ty));

    match fields
        .map(|(name, ty)| {
            let inner = flat_inner(ty)?;
            Ok(format!("{}: {}", quote!(#name), inner))
        })
        .collect::<Result<Vec<_>, Error>>()
    {
        Ok(names) => Ok(format!("{{{}}}", names.join(", "))),
        Err(err) => Err(err),
    }
}

fn flat_inner(ty: &Type) -> Result<String, Error> {
    match ty {
        Type::Path(path) => {
            let segments = &path.path.segments;
            let inner = segments.first();
            match inner {
                None => Err(Error::new(ty.span(), "Type ident is required")),
                Some(inner) => {
                    let ident = inner.ident.clone();
                    let ident = format!("{}", quote! {#ident});
                    // dbg!(&ident);
                    if ident == "Option" || ident == "qself" {
                        let value = get_inner_arguments_type(inner)?;
                        Ok(value.strip_suffix('!').unwrap_or(&value).to_string())
                    } else if inner.arguments.is_empty() || inner.arguments.is_none() {
                        Ok(format!("{ident}!"))
                    } else if ident == "Vec" {
                        Ok(format!("[{}]!", get_inner_arguments_type(inner)?))
                    } else {
                        Ok(format!("{}<{}>!", ident, get_inner_arguments_type(inner)?))
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

fn get_inner_arguments_type(inner: &syn::PathSegment) -> Result<String, Error> {
    match &inner.arguments {
        syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => Ok(args
            .iter()
            .map(|arg| match arg {
                GenericArgument::Type(sub_type) => flat_inner(sub_type),
                other => Err(Error::new(
                    other.span(),
                    "Generic SubType argument not yet supported",
                )),
            })
            .collect::<Result<Vec<String>, Error>>()?
            .join(", ")),
        syn::PathArguments::None => Ok(String::new()),
        argument => Err(Error::new(
            argument.span(),
            "Type argument not yet supported",
        )),
    }
}
