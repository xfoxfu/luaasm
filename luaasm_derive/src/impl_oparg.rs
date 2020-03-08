use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{parse_macro_input, punctuated::Punctuated, Ident, LitStr, Token};

struct OpArg {
    inst_name: String,
    arg_0: Ident,
    arg_1: Ident,
    arg_2: Ident,
    explain: LitStr,
}

struct MacroInput {
    var: Ident,
    val: Punctuated<OpArg, Option<Token![,]>>,
}

impl Parse for OpArg {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let inst_name = String::from(&input.parse::<Ident>()?.to_string().as_str()[3..]);
        let arg_0 = input
            .parse::<Option<Ident>>()?
            .unwrap_or(Ident::new("N", Span::call_site()));
        let arg_1 = input
            .parse::<Option<Ident>>()?
            .unwrap_or(Ident::new("N", Span::call_site()));
        let arg_2 = input
            .parse::<Option<Ident>>()?
            .unwrap_or(Ident::new("N", Span::call_site()));
        let explain = input.parse()?;
        Ok(OpArg {
            inst_name,
            arg_0,
            arg_1,
            arg_2,
            explain,
        })
    }
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let var = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;
        let val = input.parse_terminated(OpArg::parse)?;
        Ok(MacroInput { var, val })
    }
}

pub fn fn_impl_oparg(input: TokenStream) -> TokenStream {
    let MacroInput { var, val } = parse_macro_input!(input as MacroInput);
    let mut quotes = Vec::new();
    for OpArg {
        inst_name,
        arg_0,
        arg_1,
        arg_2,
        explain,
    } in val
    {
        quotes.push(quote! {
            #inst_name => (OpArgMap::#arg_0, OpArgMap::#arg_1, OpArgMap::#arg_2, #explain)
        });
    }
    (quote! {
        fn #var(opcode: &str) -> (OpArgMap, OpArgMap, OpArgMap, &str) {
            match opcode {
                #(#quotes),*,
                code => panic!("invalid opcode {}", code),
            }
        }
    })
    .into()
}
