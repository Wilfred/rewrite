extern crate syntex_syntax as syntax;

use std::env;
use std::fs::File;
use std::rc::Rc;
use std::path::{Path, PathBuf};

use syntax::ast;
use syntax::codemap::CodeMap;
use syntax::errors::Handler;
use syntax::errors::emitter::ColorConfig;
use syntax::parse::{self, ParseSess, PResult};

fn new_parse_session() -> ParseSess {
    let codemap = Rc::new(CodeMap::new());
    let tty_handler =
        Handler::with_tty_emitter(ColorConfig::Auto, None, true, false, codemap.clone());
    ParseSess::with_span_handler(tty_handler, codemap.clone())
}

// Copied shamelessly from format_input in rustfmt.
fn parse_file<'a>(path: &Path, parse_session: &'a ParseSess) -> PResult<'a, ast::Crate> {
    parse::parse_crate_from_file(path, Vec::new(), &parse_session)
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let path = PathBuf::from(args[1].clone());

        let parse_session = new_parse_session();
        let result = parse_file(&path, &parse_session);

        match result {
            Ok(krate) => {
                let module = krate.module;

                for item in module.items {
                    match item.node {
                        ast::ItemKind::Fn(ref fn_decl, _, _, _, _, ref block) => {
                            println!("fn_decl: {:?}\nblock: {:?}\n\n", fn_decl, block);
                        }
                        ast::ItemKind::Impl(..) => {
                            println!("TODO: impl");
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                println!("error {:?}", err);
            }
        }
    } else {
        println!("You need to specify a .rs file.");
    }

}
