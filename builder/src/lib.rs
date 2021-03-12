use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let builder_fields: Vec<proc_macro2::TokenStream>;
    let setter_methods: Vec<proc_macro2::TokenStream>;
    let initial_values: Vec<proc_macro2::TokenStream>;
    match input.data {
        Data::Struct(st) => match st.fields {
            Fields::Named(fields) => {
                builder_fields = fields
                    .named
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! { #ident: Option<#ty> }
                    })
                    .collect();
                setter_methods = fields
                    .named
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                                self.#ident = Some(#ident);
                                self
                            }
                        }
                    })
                    .collect();
                initial_values = fields
                    .named
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        // let ty = &f.ty;
                        quote! { #ident: None }
                    })
                    .collect();
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#setter_methods)*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#initial_values,)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
