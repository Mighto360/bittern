extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DataStruct, DeriveInput, Error, Field, Fields, Ident, Index, Member, Path};

const CRATE_NAME: &'static str = "bittern";
const TRAIT_NAME: &'static str = "Identity";
const KEY_ATTR: &'static str = "identity";

#[proc_macro_derive(Identity, attributes(identity))]
pub fn derive_identity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(struct_data) => derive_struct(name, struct_data),
        _ => Error::new_spanned(&name, format!("#[derive({})] only supports structs", TRAIT_NAME))
            .to_compile_error().into()
    }
}

fn find_crate() -> Path {
    parse_quote!(::#CRATE_NAME)
}

fn check_attr(fields: &Fields) -> Result<Option<usize>, Error> {
    let mut key_index: Option<usize> = None;
    for (i, field) in fields.iter().enumerate() {
        let has_key_attr = field.attrs.iter().any(|attr| attr.path().is_ident(KEY_ATTR));
        if has_key_attr {
            if key_index.is_some() {
                return Err(Error::new_spanned(field, format!("Only one field should be #[{}]", KEY_ATTR)));
            }
            key_index = Some(i);
        }
    }
    Ok(key_index)
}

fn member(index: usize, ident: Option<Ident>) -> Member {
    match ident {
        Some(ident) => Member::Named(ident),
        None => Member::Unnamed(Index::from(index)),
    }
}

fn derive_struct(target: Ident, struct_data: DataStruct) -> TokenStream {
    let fields = struct_data.fields;
    match check_attr(&fields) {
        Err(err) => err.to_compile_error().into(),
        Ok(None) => Error::new_spanned(&target, format!("Missing #[{}] attribute", KEY_ATTR))
            .to_compile_error().into(),
        Ok(Some(i)) => {
            let key_field = fields.into_iter().nth(i).expect("index out of bounds");
            derive_struct_key(target, i, key_field)
        },
    }
}

fn derive_struct_key(target: Ident, key_index: usize, key_field: Field) -> TokenStream {
    let bittern = find_crate();

    let key_ty = key_field.ty;
    let key = member(key_index, key_field.ident);

    quote! {
        impl #bittern::Identity for #target {
            type Index = <#key_ty as #bittern::Identity>::Index;

            fn index(key: &Self) -> Self::Index {
                <#key_ty as #bittern::Identity>::index(&key.#key)
            }

            fn equivalent(&self, other: &Self) -> bool {
                <#key_ty as #bittern::Identity>::equivalent(&self.#key, &other.#key)
            }
        }
        impl #bittern::Identity<#key_ty> for #target {
            type Index = <#key_ty as #bittern::Identity>::Index;

            fn index(key: &#key_ty) -> Self::Index {
                <#key_ty as #bittern::Identity>::index(key)
            }

            fn equivalent(&self, other: &#key_ty) -> bool {
                <#key_ty as #bittern::Identity>::equivalent(&self.#key, other)
            }
        }
    }.into()
}