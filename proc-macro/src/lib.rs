use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::*;

#[allow(dead_code)]
#[proc_macro_attribute]
pub fn my_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as MyInput);
    quote! {}.into()
}

#[allow(dead_code)]
struct MyInput {}

impl Parse for MyInput {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<ItemFn>()?;
        Ok(Self {})
    }
}
