use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro to implement Opdater for structs where all the fields implement it.
///
/// # Example:
/// ```rust,ignore
/// #[derive(Debug, PartialEq, Opdater)]
/// struct Bla {
///     a: Option<i32>,
///     b: Option<f32>,
/// }
///
/// let mut bla = Bla { a: None, b: None };
/// let bla_op = Bla {
///     a: Some(10),
///     b: Some(13.37),
/// };
///
/// bla.update(bla_op);
///
/// assert_eq!(
///     bla,
///     Bla {
///         a: Some(10),
///         b: Some(13.37)
///     }
/// );
/// ```
#[proc_macro_derive(Opdater)]
pub fn derive_opdater(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let syn::Data::Struct(data) = input.data else {
        return TokenStream::from(
            syn::Error::new(input.ident.span(), "Only structs can derive `Opdater`")
                .to_compile_error(),
        );
    };

    let syn::Fields::Named(fields) = data.fields else {
        return TokenStream::from(
            syn::Error::new(
                input.ident.span(),
                "Only structs with named fields can derive `Opdater`",
            )
            .to_compile_error(),
        );
    };

    let field_vals = fields
        .named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote!(b |= ::opdater::Opdater::update(&mut self.#name, other.#name);)
        })
        .collect::<Vec<_>>();

    let name = input.ident;

    let q = quote!(impl ::opdater::Opdater for #name {
        fn update(&mut self, other: Self) -> bool {
            let mut b = false;
            #(#field_vals)*
            b
        }
    });

    TokenStream::from(q)
}
