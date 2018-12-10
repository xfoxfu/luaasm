use crate::parser::{LuaAsmParser, LuaFile};
use crate::writer::{WriteObj, Writer};
use clap::{App, Arg, ArgMatches, SubCommand};
use pest::Parser;
use std::fs::File;
use std::io::Read;

pub fn get_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("asm")
        .about("assemble lua bytecode")
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("output")
                .required(true)
                .default_value("luac.out"),
        )
        .arg(
            Arg::with_name("lua")
                .short("l")
                .long("lua")
                .takes_value(true)
                .default_value("5.2"),
        )
        .arg(
            Arg::with_name("endian")
                .short("e")
                .long("endian")
                .takes_value(true)
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
    let mut writer = Writer::new();
    // Lua bytecode signature
    writer.write(0x1Bu8);
    writer.write(0x4Cu8);
    writer.write(0x75u8);
    writer.write(0x61u8);
    // [u8 version] Version number (0x52 for Lua 5.2, etc)
    match args.value_of("lua").unwrap() {
        "5.2" => writer.write(0x52u8),
        v => panic!("unsupported lua version {}", v),
    }
    // [u8 impl] Implementation (0 for reference impl)
    writer.write(0x00u8);
    // [u8 endian] Big-endian flag
    match args.value_of("endian").unwrap() {
        "little" => writer.write(0x01u8),
        "big" => writer.write(0x00u8),
        v => panic!("unsupported endian {}", v),
    }
    // [u8 intsize] Size of integers (usually 4)
    writer.write(0x04u8);
    // [u8 size_t] Size of pointers
    writer.write(0x04u8);
    // [u8 instsize] Size of instructions (always 4)
    writer.write(0x04u8);
    //  [u8 numsize] Size of Lua numbers (usually 8)
    writer.write(0x08u8);
    // [u8 use_int] Use integers instead of floats (usually for embedded)
    writer.write(0x00u8);
    // Lua magic (used to detect presence of EOL conversion)
    writer.write(0x19u8);
    writer.write(0x93u8);
    writer.write(0x0Du8);
    writer.write(0x0Au8);
    writer.write(0x1Au8);
    writer.write(0x0Au8);

    // main function
    let content: Vec<u8> = file.into();
    writer.write(content);

    // output result
    let mut file = args
        .value_of("output")
        .and_then(|path| File::create(path).ok())
        .expect("cannot open write file");
    writer
        .write_to_file(&mut file)
        .expect("cannot write output");
}
