extern crate proc_macro;

mod arg;

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn logfn(input_args: StdTokenStream, input: StdTokenStream) -> StdTokenStream {
    let input_args = syn::parse_macro_input!(input_args as syn::AttributeArgs);
    // let input = syn::parse_macro_input!(input as syn::ItemFn);

    let args = arg::from_input_vec(input_args);
    println!("{:?}", args);

    input
}
