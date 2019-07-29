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

    // eprintln!("{:#?}", fields);
    // panic!("RANDOMPANIC");

    // Gets the Type of the struct
    let types  = extract_types(&fields);

    // Constructs the name of the headers
    let header_names = fields.iter().map(|f| {
        let name = &f.ident;

        get_field_header_name(f, name.clone().unwrap());

        quote! {
            String::from(stringify!(#name))
        }
    });

    // Extract names from fields
    let valnames = fields.iter().map(|f| {
        &f.ident
    });

    // Constructs get_headers function
    let headerfn = quote! {
        fn get_headers() -> Vec<String> {
            use prettytable::{Row, Cell};
            return vec![#(#header_names,)*] 
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
                let #fname = format!("{}", &self.#fname);
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

    // Constructs into vec function
    let intovecfn = quote! {
        fn into_vec(&self) -> Vec<String> {
            use prettytable::{Row, Cell};
            #(#cells)*

            vec![#(#valnames,)*]
        }
    };

    // Expands the implementation
    let expand = quote! {
        impl Tablefy for #name {
            #headerfn

            #intovecfn
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

fn get_field_header_name(field: &Field, default: syn::Ident) -> String {
    for attr in &field.attrs {
        let segs = &attr.path.segments;

        for seg in segs {
            let attrname = seg.ident.to_string();

            if attrname == "name" {
                return parse_header_param(attr).to_string();
            };
        }
    };

    default.to_string()
}

fn parse_header_param(attr: &syn::Attribute) -> proc_macro2::Ident {
    let t : Vec<proc_macro2::TokenTree> = attr.tts.clone().into_iter().collect();

    if let proc_macro2::TokenTree::Group(x) = &t[0]{
        if let proc_macro2::TokenTree::Ident(i) = &x.stream().clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[0]{
            if i.to_string() != "name" {
                panic!("Only the following format is valid: #[header(name = \"<string>\"");
            }
        }
        else {
            panic!("Only the following format is valid: #[header(name = \"<string>\"");
        }
    };

    if let proc_macro2::TokenTree::Group(x) = &t[1]{
        if let proc_macro2::TokenTree::Punct(i) = &x.stream().clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[0]{
            if i.to_string() != "=" {
                panic!("Only the following format is valid: #[header(name = \"<string>\"");
            }
        }
        else {
            panic!("Only the following format is valid: #[header(name = \"<string>\"");
        }
    };

    if let proc_macro2::TokenTree::Group(x) = &t[2]{
        if let proc_macro2::TokenTree::Literal(i) = &x.stream().clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[0]{
            if i.to_string() != "name" {
                eprintln!("{}", i);
                panic!("Only the following format is valid: #[header(name = \"<string>\"");
            }
        }
        else {
            panic!("Only the following format is valid: #[header(name = \"<string>\"");
        }
    };

    panic!("Fail in parse_header_param: Attribute not found. One of the if-lets failed.");
    unimplemented!()
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