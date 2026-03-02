extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Error, Field, Fields, Ident, Index, Member};

const TRAIT_NAME: &'static str = "Identity";
const KEY_ATTR: &'static str = "identity";

#[proc_macro_derive(Identity, attributes(identity))]
pub fn derive_identity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(struct_data) => derive_struct(name, struct_data),
        Data::Enum(_) => derive_enum(name),
        _ => Error::new_spanned(&name, format!("#[derive({})] only supports struct and enum", TRAIT_NAME))
            .to_compile_error().into()
    }
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

fn derive_enum(target: Ident) -> TokenStream {
    derive_hash_eq(target)
}

fn derive_struct(target: Ident, struct_data: DataStruct) -> TokenStream {
    let fields = struct_data.fields;
    match check_attr(&fields) {
        Err(err) => err.to_compile_error().into(),
        Ok(None) => {
            // Try to implement based on other traits
            derive_hash_eq(target)
        },
        Ok(Some(i)) => {
            // Specified identity field
            let key_field = fields.into_iter().nth(i).expect("index out of bounds");
            derive_struct_key(target, i, key_field)
        },
    }
}

fn derive_hash_eq(target: Ident) -> TokenStream {
    quote! {
        impl ::bittern::Identity for #target where Self: ::core::hash::Hash + Eq {
            type Index = Self;

            fn equivalent(&self, other: &Self) -> bool {
                self == other
            }

            fn index(key: &Self) -> &Self {
                key
            }
        }
    }.into()
}

fn derive_struct_key(target: Ident, key_index: usize, key_field: Field) -> TokenStream {
    let key_ty = key_field.ty;
    let key = member(key_index, key_field.ident);

    quote! {
        impl ::bittern::Identity for #target {
            type Index = <#key_ty as ::bittern::Identity>::Index;

            fn index(key: &Self) -> &Self::Index {
                <#key_ty as ::bittern::Identity>::index(&key.#key)
            }

            fn equivalent(&self, other: &Self) -> bool {
                <#key_ty as ::bittern::Identity>::equivalent(&self.#key, &other.#key)
            }
        }
        impl ::bittern::Identity<#key_ty> for #target {
            type Index = <#key_ty as ::bittern::Identity>::Index;

            fn index(key: &#key_ty) -> &Self::Index {
                <#key_ty as ::bittern::Identity>::index(key)
            }

            fn equivalent(&self, other: &#key_ty) -> bool {
                <#key_ty as ::bittern::Identity>::equivalent(&self.#key, other)
            }
        }
    }.into()
}