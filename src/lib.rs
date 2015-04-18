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

trait Mutation<'a, 'b:'a> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self;
}

struct InvertBooleanMutation<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>
}

impl<'a, 'b:'a> Mutation<'a, 'b> for InvertBooleanMutation<'a, 'b> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self {
        InvertBooleanMutation {
            ecx: ecx
        }
    }
}

impl<'a, 'b> InvertBooleanMutation<'a, 'b> {
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

impl<'a, 'b> Folder for InvertBooleanMutation<'a, 'b> {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        self.invert_boolean(e)
    }
}

struct HeckleExpander;

impl ItemModifier for HeckleExpander {
    fn expand(&self, ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: P<Item>) -> P<Item> {
        let mut fld = InvertBooleanMutation::new(ecx);
        fld.fold_item(item).pop().unwrap()
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let expander = Modifier(Box::new(HeckleExpander));
    reg.register_syntax_extension(intern("heckle"), expander);
}

