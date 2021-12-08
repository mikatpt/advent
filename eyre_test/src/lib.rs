use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
/// Minimal color_eyre wrapper around the standard Rust test attribute.
/// This function initializes tracing and calls eyre::install() once
/// per process, using `std::sync::Once` to preserve idempotency.
pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(input) => input,
        Err(err) => return input_and_compile_error(input, err),
    };
    let mut has_test_attr = false;

    for attr in &attrs {
        if attr.path.is_ident("test") {
            has_test_attr = true;
        }
    }

    let test_attr = if has_test_attr {
        quote! {}
    } else {
        quote! { #[::core::prelude::v1::test] }
    };

    let initializer = syn::parse_str::<syn::Path>("crate::tracing").unwrap();

    let output = quote! {
        #test_attr
        #(#attrs)*
        #vis #sig {
            #initializer::init();
            #block
        }
    };

    output.into()
}

fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
