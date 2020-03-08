use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse_macro_input,
    punctuated::Punctuated,
    Error, Ident, LitInt, Token,
};
use syn::parse::{Parse, ParseStream, Result as ParseResult};

struct Opcode {
    inst_name: String,
    opmode: Opmode,
}

struct Opmode {
    t: LitInt,
    a: LitInt,
    b: Ident,
    c: Ident,
    mode: Ident,
}

struct MacroInput {
    var: Ident,
    val: Punctuated<Opcode, Token![,]>,
}

impl Parse for Opmode {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let t = input.parse()?;
        input.parse::<Token![,]>()?;
        let a = input.parse()?;
        input.parse::<Token![,]>()?;
        let b = input.parse()?;
        input.parse::<Token![,]>()?;
        let c = input.parse()?;
        input.parse::<Token![,]>()?;
        let mode = input.parse()?;
        Ok(Opmode { t, a, b, c, mode })
    }
}

impl Parse for Opcode {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let opmode_ident = input.parse::<Ident>()?;
        if opmode_ident != "opmode" {
            return Err(Error::new(opmode_ident.span(), "Expected `opmode`"));
        }

        let opmode_paren;
        parenthesized![opmode_paren in input];

        let inst_ident = input.parse::<Ident>()?;

        Ok(Opcode {
            inst_name: String::from(&inst_ident.to_string().as_str()[3..]),
            opmode: opmode_paren.parse()?,
        })
    }
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let var = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;
        let val = input.parse_terminated(Opcode::parse)?;
        Ok(MacroInput { var, val })
    }
}

pub fn fn_impl_opmode(input: TokenStream) -> TokenStream {
    let MacroInput { var, val } = parse_macro_input!(input as MacroInput);
    let mut counter = 0u32;
    let mut quotes = Vec::new();
    for Opcode {
        inst_name,
        opmode: Opmode { t, a, b, c, mode },
    } in val
    {
        quotes.push(quote! {
            #inst_name => (#counter, #t, #a, OpArgMask::#b, OpArgMask::#c, InstMode::#mode)
        });
        counter += 1;
    }
    (quote! {
        fn #var(opcode: &str) -> (u32, u8, u8, OpArgMask, OpArgMask, InstMode) {
            match opcode {
                #(#quotes),*,
                code => panic!("invalid opcode {}", code),
            }
        }
    })
    .into()
}
