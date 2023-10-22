use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};
use syn::{
	parse::{Parse, ParseStream},
	parse2, LitInt, Token,
};
use tap::{Conv, Tap};

struct Tup {
	rep: usize,
	tt: TokenTree,
}

impl Parse for Tup {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let tt = input.parse::<TokenTree>()?;

		input.parse::<Token![;]>()?;

		let rep = input
			.parse::<LitInt>()?
			.base10_parse::<usize>()
			.unwrap();

		Ok(Self { rep, tt })
	}
}

pub fn tup(input: TokenStream) -> TokenStream {
	let res = match parse2::<Tup>(input) {
		Ok(v) => v,
		Err(e) => return e.into_compile_error(),
	};

	TokenTree::Group(Group::new(
		Delimiter::Parenthesis,
		(0..res.rep)
			.into_iter()
			.map(|_| {
				res.tt
					.clone()
					.conv::<TokenStream>()
					.tap_mut(|ts| {
						ts.extend([TokenTree::Punct(Punct::new(
							',',
							Spacing::Alone,
						))])
					})
			})
			.collect(),
	))
	.into()
}
