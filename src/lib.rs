use proc_macro::TokenStream;

mod tup;

#[proc_macro]
pub fn tup(input: TokenStream) -> TokenStream { tup::tup(input.into()).into() }
