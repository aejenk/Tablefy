extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct, FieldsNamed, punctuated::Punctuated, Field};

#[proc_macro_derive(Tablefy)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let fields = extract_field_names(&ast.data);
    let types  = extract_types(&fields);

    let header_names = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            stringify!(#name)
        }
    });

    let valnames = fields.iter().map(|f| {
        &f.ident
    });

    let headerfn = quote! {
        fn get_headers() -> Row {
            return row![#(#header_names,)*] 
        }
    };

    // let n : Vec<proc_macro2::TokenStream>;

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

    let intorowfn = quote! {
        fn into_row(&self) -> Row {
            #(#cells)*

            row![#(#valnames,)*]
        }
    };

    let expand = quote! {
        impl Tablefy for #name {
            #headerfn

            #intorowfn
        }
    };

    expand.into()
}

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