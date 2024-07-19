
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn async_main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let f = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs  = f.attrs;
    let block = f.block;
    quote! {
        #(#attrs)*
        fn main() {
            axstd::futures::block_on(move || async #block);
        }
    }.into()
}