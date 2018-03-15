extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(FromReprEnum, attributes(ReprEnumDefault))]
pub fn from_repr_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let gen = impl_from(&ast);

    gen.into()
}

fn impl_from(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut rep: syn::Ident = ast.ident;
    let mut default_variant_name = "Unknown".to_owned();
    let mut default_variant = None;

    for a in ast.attrs.iter() {
        let meta = a.interpret_meta().unwrap();
        if meta.name() == "repr" {
            if let Some(syn::Meta::List(ml)) = a.interpret_meta() {
                let a = &ml.nested[0];
                if let &syn::NestedMeta::Meta(syn::Meta::Word(ref r)) = a {
                    rep = *r;
                }
            }
        }
        if meta.name() == "ReprEnumDefault" {
            if let syn::Meta::NameValue(ref name_value) = meta {
                if let syn::Lit::Str(ref s) = name_value.lit {
                    default_variant_name = s.value();
                }
            }
        }
    }

    let mut variants = vec![];
    if let syn::Data::Enum(ref d) = ast.data {
        for v in &d.variants {
            let ident = v.ident;
            if let Some((_, syn::Expr::Lit(ref lit))) = v.discriminant {
                if let syn::Lit::Int(ref i) = lit.lit {
                    let tmp = quote!{
                        #i => #ident,
                    };
                    variants.push(tmp);
                    if ident == default_variant_name {
                        default_variant = Some(v.ident);
                    }
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
                    _ => #default_variant,
                }
            }
        }
    }
}
