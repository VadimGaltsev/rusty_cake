extern crate syn;
extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_builder(&ast)
}

fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let builder_name = &format!("{}Builder", name);
    let director_name = &format!("{}Director", name);
    let director = &syn::Ident::new(director_name, ast.ident.span());
    let builder = &syn::Ident::new(builder_name, ast.ident.span());
    let struct_data = &ast.data;
    let current = match struct_data {
        Data::Union(union_) => {
            panic!("Unsupported type yet! :(")
        }
        Data::Enum(enum_) => {
            panic!("Unsupported type yet! :(")
        }
        Data::Struct(struct_) => {
            struct_
        }
    };
    let trait_methods = current.fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("Unsupported type yet %(");
            let current_type = &field.ty;
            let arg_type = quote! { #current_type };
            format!(
                "fn set_{}(self, {}: {}) -> {};",
                field_name,
                field_name,
                arg_type,
                director
            )
        }).map(|fn_string_sig| {
        parse_str::<TraitItemMethod>(&fn_string_sig).unwrap().sig
    })
        .collect::<Vec<MethodSig>>();
    let impl_methods = current.fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("Unsupported type yet %(");
            let current_type = &field.ty;
            let arg_type = quote! { #current_type };
            let block = quote! { self.target.#field_name = #field_name; self };
            format!(
                "fn set_{}(mut self, {}: {}) -> {} {} {} {}",
                field_name,
                field_name,
                arg_type,
                director,
                "{",
                block,
                "}"
            )
        }).map(|method| {
        parse_str::<ImplItemMethod>(&method).unwrap()
    }).collect::<Vec<ImplItemMethod>>();
    let gen = quote! {

        struct #director {
            target: #name
        }

        trait #builder {
            #(#trait_methods;)*

            fn build(self) -> #name;
        }

        impl #builder for #director {
            #(#impl_methods)*

            fn build(mut self) -> #name {
               self.target
            }
        }

        impl Builder<#director> for #name {
            type Builder = #director;

            fn builder() -> #director {
               #director { target: #name::default() }
            }
        }
    };
    gen.into()
}
