#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::ext::base::{ItemModifier, ExtCtxt, Modifier};
use syntax::codemap::{Span, Spanned};
use syntax::ptr::P;
use syntax::ast::{Item, MetaItem, Expr, ExprLit, Lit, LitBool};
use syntax::parse::token::intern;
use syntax::fold::Folder;
use rustc::plugin::Registry;

struct InvertBooleanMutation;

impl InvertBooleanMutation {
    fn invert_boolean(&self, expr: P<Expr>) -> P<Expr> {
        expr.map(|e| Expr {
            id: e.id,
            span: e.span,
            node: match e.node {
                ExprLit(lit) =>
                    ExprLit(lit.map(|l|
                        match l.node {
                            LitBool(value) => Spanned {
                                node: LitBool(!value),
                                span: l.span
                            },
                            _ => l
                        }
                    )),
                _ => e.node
            }
        })
    }
}

impl Folder for InvertBooleanMutation {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        self.invert_boolean(e)
    }
}

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

