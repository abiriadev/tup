use proc_macro2::{TokenStream, TokenTree};
use syn::{parse::Parse, parse2};

struct Tup {
	rep: usize,
	tt: TokenTree,
}

impl Parse for Tup {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> { todo!() }
}

pub fn tup(input: TokenStream) -> TokenStream {
	let res = match parse2::<Tup>(input) {
		Ok(v) => v,
		Err(e) => return e.into_compile_error(),
	};

	todo!()
}
