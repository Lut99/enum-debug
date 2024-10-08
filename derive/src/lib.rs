//  LIB.rs
//    by Lut99
//
//  Created:
//    10 Dec 2022, 11:57:28
//  Last edited:
//    08 Oct 2024, 15:13:58
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements `#[derive(EnumDebug)]` for the `enum-debug` crate.
//

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Ident, Lit, Meta};


/***** HELPER MACROS *****/
/// Throws a [`syn::Error`].
macro_rules! err {
    ($span:expr, $message:expr) => {
        syn::Error::new($span, $message).into_compile_error().into()
    };
}





/***** LIBRARY *****/
/// Does the derivation for the EnumDebug.
#[proc_macro_derive(EnumDebug, attributes(enum_debug))]
pub fn derive_enum_debug(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, attrs, generics, .. } = parse_macro_input!(input);

    // Match what we're parsing
    match data {
        Data::Enum(e) => {
            // Create the default name
            let name: String = ident.to_string();
            let mut name = quote!(#name).into();

            // Find if we also have to derive the thing
            for attr in attrs {
                // Only do our own
                if !attr.path().is_ident("enum_debug") {
                    continue;
                }

                // Attempt to parse the list
                let metas: Punctuated<Meta, Comma> = match attr.parse_args_with(Punctuated::parse_terminated) {
                    Ok(metas) => metas,
                    // Not for us
                    Err(err) => {
                        return err!(err.span(), "Failed to parse `enum_debug(...)` arguments as valid attributes");
                    },
                };

                // Parse the attributes
                for meta in metas {
                    match meta {
                        Meta::Path(path) => {
                            if path.is_ident("path") {
                                // Override with the path
                                name = quote!(::std::any::type_name::<Self>());
                            // NOTE: Legacy here, path used to be the default but now `name` is no change compared to default behaviour
                            } else if !path.is_ident("name") {
                                return err!(path.span(), format!("Unknown attribute property '{}'", path.to_token_stream()));
                            }
                        },
                        Meta::NameValue(name_value) => {
                            if name_value.path.is_ident("name") {
                                // Set the literal as the string if it is one
                                match name_value.value {
                                    Expr::Lit(ExprLit { lit: Lit::Str(set_name), .. }) => {
                                        let set_name = set_name.value();
                                        name = quote!(#set_name);
                                    },
                                    expr => {
                                        return err!(expr.span(), "Name must be a string literal");
                                    },
                                }
                            } else {
                                return err!(name_value.path.span(), format!("Unknown attribute property '{}'", name_value.path.to_token_stream()));
                            }
                        },

                        l => {
                            return err!(l.span(), format!("Unknown attribute property '{}'", l.to_token_stream()));
                        },
                    }
                }
            }

            // Find the variants
            let variants: Vec<&Ident> = e.variants.iter().map(|v| &v.ident).collect();

            // Emit the enum itself, either with generics or without
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            quote! {
                impl #impl_generics ::enum_debug::EnumDebug for #ident #ty_generics #where_clause {
                    #[inline]
                    fn type_name() -> &'static ::std::primitive::str { #name }

                    fn variant_names() -> &'static [&'static ::std::primitive::str] {
                        &[#(::std::stringify!(#variants)),*]
                    }

                    fn variant_name(&self) -> &'static ::std::primitive::str {
                        match self {
                            #(#ident::#variants{ .. } => ::std::stringify!(#variants),)*
                            #[allow(dead_code)]
                            _ => ::std::unreachable!(),
                        }
                    }
                }
            }
            .into()
        },

        // Can only do enums, clearly
        _ => {
            err!(ident.span(), "EnumDebug can only be derived on enums")
        },
    }
}
