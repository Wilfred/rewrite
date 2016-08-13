extern crate syntex_syntax as syntax;

use std::rc::Rc;
use syntax::codemap::{CodeMap};
use syntax::errors::{Handler};
use syntax::errors::emitter::{ColorConfig};
use syntax::parse::{self, ParseSess};


fn main() {
    // Copied shamelessly from format_input in rustfmt.
    let codemap = Rc::new(CodeMap::new());
    let tty_handler =
        Handler::with_tty_emitter(ColorConfig::Auto, None, true, false, codemap.clone());
    let mut parse_session = ParseSess::with_span_handler(tty_handler, codemap.clone());

    // TODO: distil a simple example for
    // http://stackoverflow.com/questions/26575443/how-do-i-use-the-rustc-parser-libsyntax
    let src = "fn foo() { \n let x = 1; \n //bar \n let y = x + 1; }".to_owned();

    println!("source: {}", src);
    println!("----------");
    
    let result = parse::parse_crate_from_source_str("stdin".to_owned(), src, Vec::new(), &parse_session);

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
}
