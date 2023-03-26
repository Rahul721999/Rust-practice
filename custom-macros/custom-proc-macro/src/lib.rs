use proc_macro::TokenStream;

//defining the proc-macro / attribute-like macro
#[proc_macro_attribute]
pub fn my_macro(arg : TokenStream) -> TokenStream{

}