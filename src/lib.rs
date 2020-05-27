extern crate proc_macro;

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn logfn(_attr: StdTokenStream, tokens: StdTokenStream) -> StdTokenStream {
    tokens
}
