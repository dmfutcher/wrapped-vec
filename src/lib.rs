extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(WrappedVec, 
    attributes(
        CollectionName,
        CollectionDoc
    )
)]
pub fn wrapped_vec(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_wrapped_vec(&ast);
    gen.parse().unwrap()
}

fn impl_wrapped_vec(ast: &syn::DeriveInput) -> quote::Tokens {
    let item_ident = &ast.ident;

    let collection_name = attr_string_val(ast, "CollectionName").expect("Need [CollectionName=\"...\"]");
    let collection_ident = syn::Ident::from(collection_name);

    let collection_doc = attr_string_val(ast, "CollectionDoc").unwrap_or(format!("A collection of {}s", item_ident));

    quote! {
        #[doc=#collection_doc]
        pub struct #collection_ident(Vec<#item_ident>);
    }
}

fn attr_string_val(ast: &syn::DeriveInput, attr_name: &'static str) -> Option<String> {
    if let Some(ref a) = ast.attrs.iter().find(|a| a.name() == attr_name) {
        if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref val, _)) = a.value {
            return Some(val.clone())
        }
        else {
            return None
        }
    } else {
        return None
    }
}
