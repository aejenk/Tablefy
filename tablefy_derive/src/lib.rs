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
#[proc_macro_derive(Tablefy, attributes(header))]
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
        let name = get_field_header_name(&f, &f.ident.clone().unwrap());

        quote! {
            String::from(#name)
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

/// Note about the following two header extraction functions.
/// 
/// For now, these are specific to the one attibute:
/// `#[header(name = <value>)]`.
/// 
/// However they can be made generic by using the pattern:
/// `#[<name>(<field> = <value>[,]...)]`
/// 
/// Since this is not required for now however, it won't be implemented.
/// This comment serves as a reminder for when new attributes might be added.

/// Retrieves the value in #[header(name = <value>)] if there is such an attribute.
/// Otherwise, returns the default.
fn get_field_header_name(field: &Field, default: &syn::Ident) -> String {
    for attr in &field.attrs {
        let segs = &attr.path.segments;

        for seg in segs {
            let attrname = seg.ident.to_string();

            if attrname == "header" {
                return parse_header_name_param(attr);
            };
        }
    };

    default.to_string()
}

/// Parses the value from #[header(name = <value>)].
/// 
/// If any problems during parsing occur, the function panics.
fn parse_header_name_param(attr: &syn::Attribute) -> String {
    let t : Vec<proc_macro2::TokenTree> = attr.tts.clone().into_iter().collect();

    let stream = if let proc_macro2::TokenTree::Group(x) = &t[0] {
        x.stream()
    } else {
        unimplemented!()
    };

    let generic_error = "Only the following header attribute is currently valid: #[header(name = \"<str>\")]. This may change in the future, however.";

    // The first part needs to be an identifier called "name"
    // It specifies the name of the header.
    if let proc_macro2::TokenTree::Ident(i) = &stream.clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[0]{
        if i.to_string() != "name" {
            panic!("Found '{}' instead of 'name' for #[header(name = <string>)]", i);
        }
    }
    else {
        panic!(generic_error);
    }

    // The second part needs to be a '=' symbol.
    if let proc_macro2::TokenTree::Punct(i) = &stream.clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[1]{
        if i.to_string() != "=" {
            panic!("Found '{}' instead of 'name' for #[header(name = <string>)]", i);
        }
    }
    else {
        panic!(generic_error);
    }

    // If the past two parts were valid, and this part is a string literal, then it is valid.
    if let proc_macro2::TokenTree::Literal(i) = &stream.clone().into_iter().collect::<Vec<proc_macro2::TokenTree>>()[2]{
        let s = i.to_string();

        if s.starts_with("\"") && s.ends_with("\"") {
            // Remove escaped quotation marks and return
            return format!("{}",i.to_string().replace("\"", ""));
        }
        else {
            panic!("Value '{}' needs to be a string literal.", s);
        }
    }
    else {
        panic!(generic_error);
    }
}