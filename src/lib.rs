extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;


use proc_macro::TokenStream;

#[proc_macro_derive(FromReprEnum)]
pub fn from_repr_enum(input: TokenStream) -> TokenStream {
	let ast : syn::DeriveInput = syn::parse(input).unwrap();

	let gen = impl_from(&ast);

	gen.into()
}


fn impl_from(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	let mut rep: syn::Ident = ast.ident;
	for a in ast.attrs.iter() {
		if a.interpret_meta().unwrap().name() == "repr" {
			if let Some(syn::Meta::List(ml)) = a.interpret_meta() {
				let a = &ml.nested[0];
				if let &syn::NestedMeta::Meta(syn::Meta::Word(ref r)) = a {
					rep = *r;
				}
			}
		}
	}


	let mut variants = vec!();
	if let syn::Data::Enum(ref d) = ast.data {

		for v in &d.variants {
			let ident = v.ident;
			if let Some( (_, syn::Expr::Lit(ref lit))) = v.discriminant {
				if let syn::Lit::Int(ref i) = lit.lit {
					let tmp = quote!{
						#i => #ident,
					};
					variants.push(tmp);
				}
			}
		}
	} else {
		panic!("#[derive(FromReprEnum)] is only defined for Enum")
	}

	quote!{
		impl From<#rep> for #name {
			fn from(x: #rep) -> Self {
				use #name::*;
				match x {
					#(#variants)*
					_ => Unknown,
				}
			}
		}
	}
}
