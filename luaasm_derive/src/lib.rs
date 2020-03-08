extern crate proc_macro;

use proc_macro::TokenStream;

mod impl_datatype;
mod impl_oparg;
mod impl_opmode;

#[proc_macro]
pub fn impl_opmode(input: TokenStream) -> TokenStream {
    impl_opmode::fn_impl_opmode(input)
}

#[proc_macro]
pub fn impl_datatype(input: TokenStream) -> TokenStream {
    impl_datatype::fn_impl_datatype(input)
}

#[proc_macro]
pub fn impl_oparg(input: TokenStream) -> TokenStream {
    impl_oparg::fn_impl_oparg(input)
}
