//  LIB.rs
//    by Lut99
// 
//  Created:
//    10 Dec 2022, 11:57:28
//  Last edited:
//    28 Jan 2023, 13:52:42
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
    let DeriveInput{ ident, data, attrs, generics, .. } = parse_macro_input!(input);

    // Match what we're parsing
    match data {
        Data::Enum(e) => {
            // Create the default name
            let name : String = ident.to_string();
            let mut name      = quote!(#name).into();

            // Find if we also have to derive the thing
            for attr in attrs {
                match attr.parse_meta() {
                    Ok(Meta::List(list)) => {
                        if list.path.segments.len() == 1 && list.path.segments[0].ident == "enum_debug" {
                            // Search the list to find if 'name' is there
                            for l in list.nested {
                                match l {
                                    NestedMeta::Meta(Meta::Path(path)) => {
                                        if let Some(id) = path.get_ident() {
                                            if id == "path" {
                                                // Override with the path
                                                name = quote!("{}", ::std::any::type_name::<Self>());
                                            } else if id != "name" {
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
                                                Lit::Str(set_name) => { let set_name = set_name.value(); name = quote!(#set_name); },
                                                lit                => { panic!("Illegal name '{}' (expected a string literal)", lit.to_token_stream()); },
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
            let fmt_name = quote!{
                fn fmt_type_name(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::write!(f, #name)
                }
            };

            // Find the variants
            let variants: Vec<&Ident> = e.variants.iter().map(|v| &v.ident).collect();

            // Emit the enum itself, either with generics or without
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            quote!{
                impl #impl_generics enum_debug::EnumDebug for #ident #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            #(#ident::#variants{ .. } => ::std::write!(f, ::std::stringify!(#variants)),)*
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
 