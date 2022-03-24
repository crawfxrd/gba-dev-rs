// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopoyrightText: 2022 Tim Crawford <crawfxrd@gmail>

//! Rust procedural macros for GBA development.

use proc_macro::TokenStream;
use syn::spanned::Spanned;

/// Attribute to declare the main function of the GBA program.
///
/// The function should be declared as:
///
/// ```rust
/// use gba::entry;
///
/// #[entry]
/// fn main() {
///     // ...
/// }
/// ```
///
/// The function name can be anything. The exported symbol for the function
/// will be `main`.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, stream: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(stream as syn::ItemFn);

    if !args.is_empty() {
        return syn::Error::new(input.span(), "Attribute does not take any arguments")
            .to_compile_error()
            .into();
    }

    let attrs = input.attrs;
    let sig = input.sig;
    let block = input.block;

    // Validate the signature
    if sig.constness.is_some() {
        return syn::Error::new(sig.constness.span(), "Entry point must not be `const`")
            .to_compile_error()
            .into();
    }
    if sig.asyncness.is_some() {
        return syn::Error::new(sig.asyncness.span(), "Entry point must not be `async`")
            .to_compile_error()
            .into();
    }
    if sig.unsafety.is_some() {
        return syn::Error::new(sig.unsafety.span(), "Entry point must not be `unsafe`")
            .to_compile_error()
            .into();
    }
    if sig.abi.is_some() {
        return syn::Error::new(sig.abi.span(), "Entry point must not declare an ABI")
            .to_compile_error()
            .into();
    }
    if !sig.generics.params.is_empty() {
        return syn::Error::new(sig.span(), "Entry point must not use generics")
            .to_compile_error()
            .into();
    }
    if !sig.inputs.is_empty() {
        return syn::Error::new(sig.span(), "Entry point must not declare paramenters")
            .to_compile_error()
            .into();
    }

    match sig.output {
        syn::ReturnType::Default => (),
        syn::ReturnType::Type(..) => {
            return syn::Error::new(sig.output.span(), "Entry point must not return a value")
                .to_compile_error()
                .into()
        }
    }

    TokenStream::from(quote::quote! {
        #(#attrs)*
        #[export_name = "main"]
        extern "C" #sig -> !
        #block
    })
}
