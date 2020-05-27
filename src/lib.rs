extern crate proc_macro;

mod attrs;

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn logfn(attr: StdTokenStream, input: StdTokenStream) -> StdTokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    // let input = syn::parse_macro_input!(input as syn::ItemFn);

    println!("{:?}", attr[0]);
    input
}
