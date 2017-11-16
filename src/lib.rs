extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, DeriveInput};

struct Idents {
    item: Ident,
    collection: Ident
}

impl Idents {

    fn new(ast: &DeriveInput) -> Result<Idents, String> {
        let collection_name = attr_string_val(ast, "CollectionName").expect("Need [CollectionName=\"...\"]");

        Ok(Idents{
            item: ast.ident.clone(),
            collection: Ident::from(collection_name)
        })
    }

    fn as_parts(&self) -> (&Ident, &Ident) {
        (&self.item, &self.collection)
    }

}

struct Docs {
    wrapper: String
}

impl Docs {

    fn new(ast: &DeriveInput, idents: &Idents) -> Docs {
        let wrapper = attr_string_val(ast, "CollectionDoc").unwrap_or(format!("A collection of {}s", idents.item));

        Docs {
            wrapper: wrapper
        }
    }

}

#[proc_macro_derive(WrappedVec, 
    attributes(
        CollectionName,
        CollectionDoc
    )
)]
pub fn wrapped_vec(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    match impl_wrapped_vec(&ast) {
        Ok(gen) => {
            gen.parse().unwrap()
        },
        Err(err) => {
            panic!(err);
        }
    }
}

fn impl_wrapped_vec(ast: &DeriveInput) -> Result<quote::Tokens, String> {
    let idents = Idents::new(ast)?;
    let docs = Docs::new(ast, &idents);

    Ok(generate_wrapped_vec(&idents, &docs))
}

fn generate_wrapped_vec(idents: &Idents, docs: &Docs) -> quote::Tokens {
    let (item_ident, collection_ident) = idents.as_parts();

    let collection_doc = &docs.wrapper;

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
