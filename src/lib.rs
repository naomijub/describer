//! Helper crate that generates customizable Rust `Structs` describing strings.

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, FieldsNamed, Type, parse_macro_input, spanned::Spanned};

/// ```rust
/// use describer::Describe;
///
/// #[derive(Describe)]
/// struct MyStruct {
///     opt: Option<bool>,
///     my_string: String,
/// }
///
/// assert_eq!(
///     MyStruct::describe(),
///     "MyStruct {opt: bool, my_string: String!}"
/// );
/// ```
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
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named
                    .iter()
                    .map(|f| &f.ident)
                    .zip(named.iter().map(|f| &f.ty));

                match fields
                    .map(|(name, ty)| {
                        let is_optional = is_optional(ty)?;
                        let inner = optional_inner(ty)?;
                        Ok(format!(
                            "{}: {}{}",
                            quote!(#name),
                            if is_optional {
                                inner
                            } else {
                                format!("{}", quote!(#ty))
                            },
                            if is_optional { "" } else { "!" }
                        ))
                    })
                    .collect::<Result<Vec<_>, Error>>()
                {
                    Ok(names) => Ok(names.join(", ")),
                    Err(err) => Err(err),
                }
            }
            syn::Fields::Unnamed(variant) => Err(Error::new(
                variant.span(),
                "The Tuple Struct variant is not yet supported",
            )),
            syn::Fields::Unit => Err(Error::new(
                s.fields.span(),
                "The Unit variant is not yet supported",
            )),
        },
        _ => panic!("The syn::Data variant is not yet supported"),
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
               format!("{} {{{}}}",stringify!(#ident), #field_names)
            }
        }
    };

    expanded.into()
}

fn is_optional(ty: &Type) -> Result<bool, Error> {
    match ty {
        Type::Path(_) => Ok(format!("{}", quote!(#ty)).starts_with("Option")),
        variant => Err(Error::new(
            variant.span(),
            format!("Type {} not supported in current version", quote!(#ty)),
        )),
    }
}

fn optional_inner(ty: &Type) -> Result<String, Error> {
    match ty {
        Type::Path(path) => {
            let inner = path
                .path
                .segments
                .iter()
                .flat_map(|seg| quote!(#seg.ident))
                .find(|segment| match segment {
                    proc_macro2::TokenTree::Ident(ident) => {
                        let ident = format!("{}", quote! {#ident});
                        ident != "Option" && ident != "qself"
                    }
                    _ => false,
                });
            inner
                .map(|seg| {
                    format!("{}", quote!(#seg.ident))
                        .strip_suffix(".ident")
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("{}", quote!(#seg.ident)))
                })
                .ok_or_else(|| Error::new(ty.span(), "Failed to identify Option generic type"))
        }
        variant => Err(Error::new(
            variant.span(),
            format!("Type {} not supported in current version", quote!(#ty)),
        )),
    }
}
