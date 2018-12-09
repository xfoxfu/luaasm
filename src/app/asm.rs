use crate::parser::{LuaAsmParser, LuaFile};
use clap::{App, Arg, ArgMatches, SubCommand};
use pest::Parser;
use std::fs::File;
use std::io::Read;

pub fn get_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("asm")
        .about("assemble lua bytecode")
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("lua")
                .alias("l")
                .required(true)
                .default_value("5.2"),
        )
        .arg(
            Arg::with_name("endian")
                .alias("e")
                .required(true)
                .default_value("little"),
        )
}
pub fn run(args: &ArgMatches) {
    let mut file = args
        .value_of("input")
        .and_then(|path| File::open(path).ok())
        .expect("cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read file content");
    let file: LuaFile = LuaAsmParser::parse(crate::parser::Rule::File, &contents)
        .unwrap()
        .next()
        .unwrap()
        .into();
    // print!("{}", serde_json::to_string(&result).unwrap());

    // common header
    let mut result: Vec<u8> = Vec::new();
    // Lua bytecode signature
    result.extend(&[0x1B, 0x4C, 0x75, 0x61]);
    // [u8 version] Version number (0x52 for Lua 5.2, etc)
    match args.value_of("lua").unwrap() {
        "5.2" => result.push(0x52),
        v => panic!("unsupported lua version {}", v),
    }
    // [u8 impl] Implementation (0 for reference impl)
    result.push(0x00);
    // [u8 endian] Big-endian flag
    match args.value_of("endian").unwrap() {
        "little" => result.push(0x01),
        "big" => result.push(0x00),
        v => panic!("unsupported endian {}", v),
    }
    // [u8 intsize] Size of integers (usually 4)
    result.push(0x04);
    // [u8 size_t] Size of pointers
    result.push(0x04);
    // [u8 instsize] Size of instructions (always 4)
    result.push(0x04);
    //  [u8 numsize] Size of Lua numbers (usually 8)
    result.push(0x08);
    // [u8 use_int] Use integers instead of floats (usually for embedded)
    result.push(0x00);
    // Lua magic (used to detect presence of EOL conversion)
    result.extend(&[0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A]);

    // main function
    result.append(&mut file.into());

    // output result
    for num in result {
        println!("{:02X}", num);
    }
}
