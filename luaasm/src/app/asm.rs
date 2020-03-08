use crate::parser::AstCheck;
use crate::writer::{WriteObj, Writer};
use clap::{App, Arg, ArgMatches, SubCommand};
use nom::error::convert_error;
use nom::Err;
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
    match crate::parser::parse_file(&contents) {
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!(
                "An error occurred when parsing:\n{}",
                convert_error(contents.as_str(), e)
            );
            return;
        }
        Ok((_, file)) => {
            file.check().unwrap();
            // print!("{}", serde_json::to_string(&result).unwrap());

            // file
            let mut writer = Writer::new();
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
        _ => unreachable!(),
    }
}
