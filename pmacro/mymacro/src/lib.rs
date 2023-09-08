use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, FieldsNamed, DataStruct, Fields, Data, DeriveInput};
use quote::quote;


#[proc_macro_derive(Speak)]
pub fn speak_macro_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = 
        parse_macro_input!(input as DeriveInput);
    impl_speak_trait(ident, data)
}

fn impl_speak_trait(ident: Ident, data: Data) -> TokenStream {
    let name = &ident;
    let fields = if let Data::Struct(
        DataStruct { 
            fields: Fields::Named(
                FieldsNamed { ref named, .. }
            ), .. 
        }
    ) = data {
        named
    } else {
        panic!("This data structure not supported!")
    };

    let newcode = quote! {
        impl Speak for #name {
            fn speak(&self, sound: &str) {
                println!("{} says {}", stringify!(#name), self.sound);
            }
        }
        impl #name {
            fn show_fields(&self) {
                println!("{}", stringify!(#fields));
            }
        }
    };
    newcode.into()
}


#[proc_macro]
pub fn author(_input: TokenStream) -> TokenStream {
    let fsign = "fn get_author() -> String";
    let fexpr = "{ return \"author@email.com\".to_string() }";
    let mut result = fsign.to_string();
    result.push_str(fexpr);
    result.parse().unwrap()
}


#[proc_macro_attribute]
pub fn exclude(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

