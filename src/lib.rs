#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::ext::base::{ItemModifier, ExtCtxt, Modifier};
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast::{Item, MetaItem};
use syntax::parse::token::intern;
use rustc::plugin::Registry;

struct HeckleExpander;

impl ItemModifier for HeckleExpander {
    fn expand(&self, ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: P<Item>) -> P<Item> {
        item
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let expander = Modifier(Box::new(HeckleExpander));
    reg.register_syntax_extension(intern("heckle"), expander);
}

