//  LIB.rs
//    by Lut99
// 
//  Created:
//    10 Dec 2022, 11:57:28
//  Last edited:
//    10 Dec 2022, 14:55:58
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements `#[derive(EnumDebug)]` for the `enum-debug` crate.
// 

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Ident, Lit, Meta, NestedMeta};


/***** LIBRARY *****/
/// Does the derivation for the EnumDebug.
#[proc_macro_derive(EnumDebug, attributes(enum_debug))]
pub fn derive_enum_debug(input: TokenStream) -> TokenStream {
    let DeriveInput{ ident, data, attrs, .. } = parse_macro_input!(input);

    // Match what we're parsing
    match data {
        Data::Enum(e) => {
            // Find if we also have to derive the thing
            let mut fmt_name: Option<String> = None;
            for attr in attrs {
                match attr.parse_meta() {
                    Ok(Meta::List(list)) => {
                        if list.path.segments.len() == 1 && list.path.segments[0].ident == "enum_debug" {
                            // Search the list to find if 'name' is there
                            for l in list.nested {
                                match l {
                                    NestedMeta::Meta(Meta::Path(path)) => {
                                        if let Some(id) = path.get_ident() {
                                            if id == "name" {
                                                fmt_name = Some(format!("{}", ident));
                                            } else {
                                                panic!("Unknown attribute property '{}'", id);
                                            }
                                        } else {
                                            panic!("Unknown attribute property '{}'", path.to_token_stream());
                                        }
                                    },
                                    NestedMeta::Meta(Meta::NameValue(name_value)) => {
                                        if name_value.path.segments.len() == 1 && name_value.path.segments[0].ident == "name" {
                                            // Set the literal as the string if it is one
                                            match name_value.lit {
                                                Lit::Str(name) => { fmt_name = Some(name.value()); },
                                                lit            => { panic!("Illegal name '{}' (expected a string)", lit.to_token_stream()); },
                                            }
                                        } else {
                                            panic!("Unknown attribute property '{}'", name_value.to_token_stream());
                                        }
                                    },

                                    l => { panic!("Unknown attribute property '{}'", l.to_token_stream()); },
                                }
                            }
                        }
                    },

                    // Ignore the rest (possibly other attributes)
                    _ => {},
                }
            }

            // Generate a proper implementation if there is a name to set
            let fmt_name = fmt_name.map(|name| {
                quote!{
                    fn fmt_type_name(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        write!(f, #name)
                    }
                }
            });

            // Find the variants
            let variants: Vec<&Ident> = e.variants.iter().map(|v| &v.ident).collect();

            // Emit the enum itself
            quote!{
                impl enum_debug::EnumDebug for #ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            #(#ident::#variants{ .. } => write!(f, stringify!(#variants)),)*
                        }
                    }

                    #fmt_name
                }
            }.into()
        },

        // Can only do enums, clearly
        _ => {
            panic!("EnumDebug can only be derived on enums");
        }
    }
}
 