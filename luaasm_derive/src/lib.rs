extern crate proc_macro;

use proc_macro::TokenStream;

mod impl_datatype;
mod impl_opcode;

#[proc_macro]
pub fn impl_opcode(input: TokenStream) -> TokenStream {
    impl_opcode::fn_impl_opcode(input)
}

#[proc_macro]
pub fn impl_datatype(input: TokenStream) -> TokenStream {
    impl_datatype::fn_impl_datatype(input)
}
