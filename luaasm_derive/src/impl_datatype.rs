use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    Ident, LitInt, Token,
};

struct DataType {
    type_name: String,
    id: LitInt,
}

struct MacroInput {
    var: Ident,
    val: Punctuated<DataType, Option<Token![,]>>,
}

impl Parse for DataType {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        input.parse::<Token![#]>()?;
        let _define = input.parse::<Ident>()?.to_string();
        let type_name = input.parse::<Ident>()?.to_string();
        let id;
        if input.peek(Paren) {
            let par_content;
            parenthesized!(par_content in input);
            id = par_content.parse::<LitInt>()?;
        } else {
            id = input.parse::<LitInt>()?;
        }
        Ok(DataType { type_name, id })
    }
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let var = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;
        let val = input.parse_terminated(DataType::parse)?;
        Ok(MacroInput { var, val })
    }
}

pub fn fn_impl_datatype(input: TokenStream) -> TokenStream {
    let MacroInput { var, val } = parse_macro_input!(input as MacroInput);
    let mut quotes = Vec::new();
    for DataType { type_name, id } in val {
        quotes.push(quote! {
            #type_name => #id
        });
    }
    (quote! {
        fn #var(datatype: &str) -> isize {
            match datatype {
                #(#quotes),*,
                _ => panic!("invalid opcode"),
            }
        }
    })
    .into()
}
