extern crate syntex_syntax as syntax;

use std::rc::Rc;
use syntax::codemap::{CodeMap};
use syntax::errors::{Handler};
use syntax::errors::emitter::{ColorConfig};
use syntax::parse::{self, ParseSess};


fn main() {
    println!("Hello, world!");

    // Copied shamelessly from format_input in rustfmt.
    let codemap = Rc::new(CodeMap::new());
    let tty_handler =
        Handler::with_tty_emitter(ColorConfig::Auto, None, true, false, codemap.clone());
    let mut parse_session = ParseSess::with_span_handler(tty_handler, codemap.clone());
    
}
