use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, FieldsNamed};

#[proc_macro_derive(Context, attributes(has))]
pub fn derive_context_attr(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident;
    let mut func_stream = TokenStream::default();

    if let syn::Data::Struct(s) = ast.data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);

            for (field, ftype) in fields.into_iter().zip(ftypes.into_iter()) {
                func_stream.extend::<TokenStream>(
                    quote! {
                        impl Has<#ftype> for #name {
                            fn get(&self) -> &#ftype {
                                &self.#field
                            }
                            fn get_mut(&mut self) -> &mut #ftype {
                                &mut self.#field
                            }
                        }
                    }
                    .into(),
                );
            }
        }
    };

    func_stream
}
