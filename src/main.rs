extern crate syntex_syntax as syntax;

use std::env;
use std::rc::Rc;
use std::path::{PathBuf};

use syntax::codemap::{CodeMap};
use syntax::errors::{Handler};
use syntax::errors::emitter::{ColorConfig};
use syntax::parse::{self, ParseSess};


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        // Copied shamelessly from format_input in rustfmt.
        let codemap = Rc::new(CodeMap::new());
        let tty_handler =
            Handler::with_tty_emitter(ColorConfig::Auto, None, true, false, codemap.clone());
        let parse_session = ParseSess::with_span_handler(tty_handler, codemap.clone());

        let path = PathBuf::from(args[1].clone());
        let result = parse::parse_crate_from_file(&path, Vec::new(), &parse_session);

        match result {
            Ok(krate) => {
                println!("crate attrs {:?}", krate.attrs);
                println!("crate config {:?}", krate.config);
                println!("crate span {:?}", krate.span);
                let module = krate.module;
                println!("module inner {:?}", module.inner);
                println!("module items {:?}", module.items);
            }
            Err(err) => {
                println!("error {:?}", err);
            }
        }
    } else {
        println!("You need to specify a .rs file.");
    }
    
}
