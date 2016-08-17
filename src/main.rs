extern crate syntex_syntax as syntax;

use std::env;
use std::rc::Rc;
use std::path::{Path, PathBuf};

use syntax::ast;
use syntax::ptr::P;
use syntax::codemap::{CodeMap, BytePos};
use syntax::errors::Handler;
use syntax::errors::emitter::ColorConfig;
use syntax::parse::{self, ParseSess, PResult};

fn new_parse_session(codemap: Rc<CodeMap>) -> ParseSess {
    let tty_handler =
        Handler::with_tty_emitter(ColorConfig::Auto, None, true, false, codemap.clone());
    ParseSess::with_span_handler(tty_handler, codemap.clone())
}

// Copied shamelessly from format_input in rustfmt.
fn parse_file<'a>(path: &Path, parse_session: &'a ParseSess) -> PResult<'a, ast::Crate> {
    parse::parse_crate_from_file(path, Vec::new(), &parse_session)
}

#[derive(Debug)]
struct SplicePosition {
    keep_to: BytePos,
    new_text: String,
    continue_from: BytePos,
}

// TODO: name this 'RenameLet' to avoid confusion with bindings
// introduced via match?
trait RenameLocalDef<T> {
    fn rename_local(&self, old: String, new: String) -> Vec<SplicePosition>;
}

impl RenameLocalDef<ast::DeclKind> for ast::DeclKind {
    // TODO: take a BytePos so we know *which* local we want to
    // rename.
    fn rename_local(&self, old: String, new: String) -> Vec<SplicePosition> {
        match *self {
            ast::DeclKind::Local(ref local) => {
                match local.pat.node {
                    ast::PatKind::Ident(_, ref ident, _) => {
                        if *ident.node.name.as_str() == old {
                            return vec![SplicePosition {
                                            keep_to: ident.span.lo,
                                            new_text: new.clone(),
                                            continue_from: ident.span.hi,
                                        }];
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        vec![]
    }
}

fn print_all_items(items: &Vec<P<ast::Item>>, codemap: &CodeMap) {
    for item in items {
        println!("{}", codemap.span_to_snippet(item.span).unwrap());
        match item.node {
            ast::ItemKind::Fn(_, _, _, _, _, ref block) => {
                for stmt in &block.stmts {
                    println!("from span: {:?}",
                             codemap.span_to_snippet(stmt.span).unwrap());
                    match &stmt.node {
                        &ast::StmtKind::Decl(ref decl, _) => {
                            println!("decl: {:?}", decl);
                            println!("rename pos: {:?}", decl.node.rename_local("x".to_owned(), "xxx".to_owned()));
                        }
                        _ => {}
                    }
                }
            }
            ast::ItemKind::Impl(..) => {
                println!("TODO: impl");
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let path = PathBuf::from(args[1].clone());

        let codemap = Rc::new(CodeMap::new());
        let parse_session = new_parse_session(codemap.clone());
        let result = parse_file(&path, &parse_session);

        match result {
            Ok(krate) => {
                let module = krate.module;

                print_all_items(&module.items, &codemap)
            }
            Err(err) => {
                println!("error {:?}", err);
            }
        }
    } else {
        println!("You need to specify a .rs file.");
    }

}
