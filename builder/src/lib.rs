extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    
    let struct_name = &ast.ident;
    let builder_name = format!("{}Builder", struct_name);
    let builder_ident = syn::Ident::new(&builder_name, struct_name.span());

    let fields = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("Builder macro only supports named fields");
        }
    } else {
        panic!("Builder macro only supports structs");
    };

    let builder_methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        quote! {
            pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    });

    let build_checks = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            if self.#field_name.is_none() {
                return Err(format!("No '{}' value provided.", stringify!(#field_name)));
            }
        }
    });
    
    let build_assignments = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: self.#field_name.clone().unwrap()
        }
    });
    
    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        quote! {
            #field_name: Option<#field_ty>
        }
    });
    
    let builder_init_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: None
        }
    });
    
    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields,)*
        }
    
        impl #builder_ident {
            #(#builder_methods)*
    
            pub fn build(&self) -> Result<#struct_name, String> {
                #(#build_checks)*
    
                Ok(#struct_name {
                    #(#build_assignments,)*
                })
            }
        }
    
        impl #struct_name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_init_fields,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}