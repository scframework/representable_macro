extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, PathArguments, GenericArgument};

#[proc_macro_derive(Representable)]
pub fn derive_representable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields_formatter = match extract_named_fields(&input) {
        Ok(fields) => generate_field_formatters(&fields),
        Err(err) => return err,
    };
    generate_trait_impl(name, fields_formatter).into()
}

fn extract_named_fields(input: &DeriveInput) -> Result<Vec<(String, Type)>, TokenStream> {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            let fields = fields_named
                .named
                .iter()
                .map(|f| {
                    let name = f.ident.as_ref().expect("Field should have a name").to_string();
                    let ty = f.ty.clone();
                    (name, ty)
                })
                .collect();
            Ok(fields)
        } else {
            Err(compile_error("Representable only supports named fields."))
        }
    } else {
        Err(compile_error("Representable only supports structs."))
    }
}

fn generate_field_formatters(fields: &[(String, Type)]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|(name, ty)| {
            let field_name = name.clone();
            let field_ident = syn::Ident::new(&field_name, proc_macro2::Span::call_site());

            match ty {
                Type::Path(type_path) => {
                    let segment = type_path.path.segments.last().unwrap();
                    if segment.ident == "Vec" {
                        serialize_vec(&field_name, &field_ident, &segment.arguments)
                    } else {
                        serialize_regular(&field_name, &field_ident)
                    }
                },
                _ => serialize_generic(&field_name, &field_ident),
            }
        })
        .collect()
}

fn serialize_vec(field_name: &str, field_ident: &syn::Ident, arguments: &PathArguments) -> proc_macro2::TokenStream {
    if let PathArguments::AngleBracketed(ref args) = arguments {
        if let Some(GenericArgument::Type(Type::Path(ref inner_type_path))) = args.args.first() {
            let inner_type = &inner_type_path.path.segments.first().unwrap().ident;
            return match inner_type.to_string().as_str() {
                "String" => serialize_vec_string(field_name, field_ident),
                "i32" => serialize_vec_i32(field_name, field_ident),
                _ => serialize_vec_generic(field_name, field_ident),
            };
        }
    }
    panic!("Invalid arguments or unsupported Vec type");
}

fn serialize_vec_i32(field_name: &str, field_ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        format!("{}: [{}]", #field_name, self.#field_ident.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(", "))
    }
}

fn serialize_vec_string(field_name: &str, field_ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        format!("{}: [{}]", #field_name, self.#field_ident.iter().map(|item| format!(r#""{}""#, item)).collect::<Vec<String>>().join(", "))
    }
}

fn serialize_vec_generic(field_name: &str, field_ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        format!("{}: [{}]", #field_name, self.#field_ident.iter().map(|item| item.represent()).collect::<Vec<String>>().join(", "))
    }
}

fn serialize_regular(field_name: &str, field_ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        format!("{}: {}", #field_name, self.#field_ident)
    }
}

fn serialize_generic(field_name: &str, field_ident: &syn::Ident) -> proc_macro2::TokenStream {
    serialize_regular(field_name, field_ident)
}

fn generate_trait_impl(struct_name: &syn::Ident, field_formatters: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl representable_interface::Representable for #struct_name {
            fn represent(&self) -> String {
                let mut result = String::new();
                result.push_str(stringify!(#struct_name));
                result.push_str(" { ");
                result.push_str(&[
                    #(#field_formatters),*
                ].join(", "));
                result.push_str(" }");
                result
            }
        }
    }
}

fn compile_error(message: &str) -> TokenStream {
    let tokens = quote! {
        compile_error!(#message);
    };
    tokens.into()
}
