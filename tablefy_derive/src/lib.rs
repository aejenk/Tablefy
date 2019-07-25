//! A derive trait for auto-implementing the `Tablefy` trait for structs.
//! 
//! Currently only works for structs whose elements implement the display trait.
//! In the future, support may be added for Debug/Pretty printing, and customizing the 
//! names of the table headers.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct, FieldsNamed, punctuated::Punctuated, Field};

/// Derives the `Tablefy` trait for any struct.
#[proc_macro_derive(Tablefy)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    // Gets the Field of the struct
    let fields = extract_field_names(&ast.data);

    // Gets the Type of the struct
    let types  = extract_types(&fields);

    // Constructs the name of the headers
    let header_names = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            stringify!(#name)
        }
    });

    // Extract names from fields
    let valnames = fields.iter().map(|f| {
        &f.ident
    });

    // Constructs get_headers function
    let headerfn = quote! {
        fn get_headers() -> Row {
            return row![#(#header_names,)*] 
        }
    };

    // Specifies cell value for each struct.
    //
    // In the future, support for {:?} and {:#?} should be added
    let cells = (0..types.len()).map(|index| {
        let fname = &fields[index].ident;
        let ty = &types[index];
        let tname = format!("{}",&ty.ident);

        if tname != "Option" {
            quote! {
                let #fname = &self.#fname;
            }
        }
        else {
            quote! {
                let #fname = if let Some(x) = &self.#fname {
                    format!("{}", x)
                } else {
                    String::new()
                };
            }
        }
    });

    // Constructs into_row function
    let intorowfn = quote! {
        fn into_row(&self) -> Row {
            #(#cells)*

            row![#(#valnames,)*]
        }
    };

    // Expands the implementation
    let expand = quote! {
        impl Tablefy for #name {
            #headerfn

            #intorowfn
        }
    };

    expand.into()
}

/// Extracts the Fields from the AST
fn extract_field_names(data: &Data) -> &Punctuated<Field, syn::token::Comma>{
    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed {
            ref named,
            ..
        }), ..
    }) = data {
        named
    } else {
        unimplemented!()
    }
}

/// Extracts the Types of the fields from the AST
fn extract_types(fields: &Punctuated<Field, syn::token::Comma>) -> Vec<&syn::PathSegment>{
    // let type_idents: Vec<&syn::Ident> = 
    fields.iter().map(|field| {
        if let syn::Field {
            ty: syn::Type::Path(
                syn::TypePath {
                    path: syn::Path {
                        ref segments,
                        ..
                    },
                    ..
                }
            ),
            ..
        } = field {
            &segments[0]
        } else {
            unimplemented!()
        }
    }).collect::<Vec<&syn::PathSegment>>()
}