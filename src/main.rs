extern crate nom;
#[macro_use]
extern crate serde_derive;

use clap::App;

mod app;
mod lua;
mod parser;
mod writer;

fn main() {
    let app: clap::App = App::new("luaasm")
        .version("1.0")
        .author("coderfox <i@xfox.me>")
        .about("Lua bytecode assembler")
        .subcommand(app::asm::get_subcommand());
    let matches = app.get_matches();
    #[allow(clippy::single_match)]
    match matches.subcommand() {
        ("asm", Some(asm)) => app::asm::run(asm),
        _ => (),
    }
}
